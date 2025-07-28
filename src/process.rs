use crate::data::Session;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::RwLock;

#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("Failed to spawn process: {0}")]
    SpawnError(#[from] std::io::Error),
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    #[error("Process already running for session: {0}")]
    ProcessAlreadyRunning(String),
    #[error("Claude Code executable not found")]
    ClaudeCodeNotFound,
}

pub struct ProcessHandle {
    pub child: Child,
    pub session_id: String,
    pub project_path: Option<PathBuf>,
    pub output_buffer: Arc<std::sync::Mutex<String>>,
}

pub struct ProcessManager {
    processes: RwLock<HashMap<String, ProcessHandle>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: RwLock::new(HashMap::new()),
        }
    }

    /// Spawn a new Claude Code instance for the given session
    pub async fn spawn_claude_session(&self, session: &Session, project_path: Option<PathBuf>) -> Result<(), ProcessError> {
        let mut processes = self.processes.write().await;
        
        if processes.contains_key(&session.id) {
            return Err(ProcessError::ProcessAlreadyRunning(session.id.clone()));
        }

        // Check if claude command is available
        if !self.is_claude_available().await {
            return Err(ProcessError::ClaudeCodeNotFound);
        }

        let mut command = Command::new("claude");
        
        // Set up the command for interactive mode
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        // If we have a project path, change to that directory
        if let Some(ref path) = project_path {
            command.current_dir(path);
        }

        // Spawn the process
        let mut child = command.spawn()?;

        // Create output buffer
        let output_buffer = Arc::new(std::sync::Mutex::new(String::new()));

        // Take stdout and stderr for reading
        let stdout = child.stdout.take().expect("Failed to get stdout");
        let stderr = child.stderr.take().expect("Failed to get stderr");

        // Spawn tasks to read output
        let output_buffer_stdout = Arc::clone(&output_buffer);
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if let Ok(mut buffer) = output_buffer_stdout.lock() {
                    buffer.push_str(&format!("[OUT] {line}\n"));
                }
            }
        });

        let output_buffer_stderr = Arc::clone(&output_buffer);
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if let Ok(mut buffer) = output_buffer_stderr.lock() {
                    buffer.push_str(&format!("[ERR] {line}\n"));
                }
            }
        });

        let handle = ProcessHandle {
            child,
            session_id: session.id.clone(),
            project_path,
            output_buffer,
        };

        processes.insert(session.id.clone(), handle);
        Ok(())
    }

    /// Stop a Claude Code session
    pub async fn stop_session(&self, session_id: &str) -> Result<(), ProcessError> {
        let mut processes = self.processes.write().await;
        
        if let Some(mut handle) = processes.remove(session_id) {
            // Attempt graceful shutdown first
            if handle.child.kill().await.is_err() {
                // Force kill if graceful shutdown fails
                let _ = handle.child.start_kill();
            }
            Ok(())
        } else {
            Err(ProcessError::SessionNotFound(session_id.to_string()))
        }
    }

    /// Check if a session has a running process
    pub async fn is_session_running(&self, session_id: &str) -> bool {
        let processes = self.processes.read().await;
        processes.contains_key(session_id)
    }

    /// Get the status of all managed sessions
    pub async fn get_session_statuses(&self) -> HashMap<String, bool> {
        let mut processes = self.processes.write().await;
        let mut statuses = HashMap::new();
        let mut to_remove = Vec::new();

        for (session_id, handle) in processes.iter_mut() {
            match handle.child.try_wait() {
                Ok(Some(_)) => {
                    // Process has exited
                    statuses.insert(session_id.clone(), false);
                    to_remove.push(session_id.clone());
                }
                Ok(None) => {
                    // Process is still running
                    statuses.insert(session_id.clone(), true);
                }
                Err(_) => {
                    // Error checking status, assume dead
                    statuses.insert(session_id.clone(), false);
                    to_remove.push(session_id.clone());
                }
            }
        }

        // Clean up dead processes
        for session_id in to_remove {
            processes.remove(&session_id);
        }

        statuses
    }

    /// Get the output buffer for a session
    pub async fn get_session_output(&self, session_id: &str) -> Option<String> {
        let processes = self.processes.read().await;
        if let Some(handle) = processes.get(session_id) {
            if let Ok(buffer) = handle.output_buffer.lock() {
                Some(buffer.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Cleanup all processes
    pub async fn cleanup_all(&self) {
        let mut processes = self.processes.write().await;
        
        for (_, mut handle) in processes.drain() {
            let _ = handle.child.kill().await;
        }
    }

    /// Check if claude command is available in PATH
    async fn is_claude_available(&self) -> bool {
        (Command::new("claude").arg("--version").output().await).is_ok()
    }

    /// Restart a session (stop and start)
    pub async fn restart_session(&self, session: &Session, project_path: Option<PathBuf>) -> Result<(), ProcessError> {
        // Try to stop existing process (ignore if not running)
        let _ = self.stop_session(&session.id).await;
        
        // Start new process
        self.spawn_claude_session(session, project_path).await
    }

    /// Get count of active processes
    pub async fn active_process_count(&self) -> usize {
        let processes = self.processes.read().await;
        processes.len()
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

// Graceful shutdown handling
impl Drop for ProcessManager {
    fn drop(&mut self) {
        // Note: This is a synchronous drop, so we can't await
        // The processes will be killed when the handles are dropped due to kill_on_drop(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Session;

    #[tokio::test]
    async fn test_process_manager_creation() {
        let manager = ProcessManager::new();
        assert_eq!(manager.active_process_count().await, 0);
    }

    #[tokio::test]
    async fn test_session_running_check() {
        let manager = ProcessManager::new();
        let session = Session::new(None);
        
        assert!(!manager.is_session_running(&session.id).await);
    }

    #[tokio::test]
    async fn test_stop_nonexistent_session() {
        let manager = ProcessManager::new();
        let result = manager.stop_session("nonexistent").await;
        
        assert!(matches!(result, Err(ProcessError::SessionNotFound(_))));
    }

    #[tokio::test]
    async fn test_cleanup_all() {
        let manager = ProcessManager::new();
        manager.cleanup_all().await;
        
        assert_eq!(manager.active_process_count().await, 0);
    }
}