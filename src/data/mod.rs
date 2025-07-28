use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
}

impl Project {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            path,
            created_at: Utc::now(),
            last_accessed: None,
        }
    }

    pub fn update_last_accessed(&mut self) {
        self.last_accessed = Some(Utc::now());
    }

    pub fn exists(&self) -> bool {
        self.path.exists() && self.path.is_dir()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionStatus {
    Active,
    Stopped,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Session {
    pub id: String,
    pub project_id: Option<String>,
    pub status: SessionStatus,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn new(project_id: Option<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_id,
            status: SessionStatus::Active,
            created_at: Utc::now(),
        }
    }

    pub const fn stop(&mut self) {
        self.status = SessionStatus::Stopped;
    }

    pub const fn set_error(&mut self) {
        self.status = SessionStatus::Error;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppStats {
    pub total_projects: usize,
    pub active_sessions: usize,
    pub total_runtime: u64, // Duration in seconds for serialization
}

impl AppStats {
    pub const fn new() -> Self {
        Self {
            total_projects: 0,
            active_sessions: 0,
            total_runtime: 0,
        }
    }

    pub const fn get_total_runtime(&self) -> Duration {
        Duration::from_secs(self.total_runtime)
    }

    pub const fn set_total_runtime(&mut self, duration: Duration) {
        self.total_runtime = duration.as_secs();
    }
}

impl Default for AppStats {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionData {
    pub sessions: Vec<Session>,
    pub stats: SessionStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionStats {
    pub active_sessions: usize,
    pub total_runtime: u64, // Duration in seconds for serialization
}

impl SessionStats {
    pub const fn new() -> Self {
        Self {
            active_sessions: 0,
            total_runtime: 0,
        }
    }

    pub const fn get_total_runtime(&self) -> Duration {
        Duration::from_secs(self.total_runtime)
    }

    pub const fn set_total_runtime(&mut self, duration: Duration) {
        self.total_runtime = duration.as_secs();
    }
}

impl Default for SessionStats {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionData {
    pub const fn new() -> Self {
        Self {
            sessions: Vec::new(),
            stats: SessionStats::new(),
        }
    }

    pub fn add_session(&mut self, session: Session) {
        self.sessions.push(session);
        self.update_stats();
    }

    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut Session> {
        self.sessions.iter_mut().find(|s| s.id == session_id)
    }

    pub fn remove_session(&mut self, session_id: &str) -> Option<Session> {
        if let Some(pos) = self.sessions.iter().position(|s| s.id == session_id) {
            let removed = self.sessions.remove(pos);
            self.update_stats();
            Some(removed)
        } else {
            None
        }
    }

    pub fn update_stats(&mut self) {
        self.stats.active_sessions = self
            .sessions
            .iter()
            .filter(|s| matches!(s.status, SessionStatus::Active))
            .count();
    }
}

impl Default for SessionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppData {
    pub projects: Vec<Project>,
    pub sessions: Vec<Session>,
    pub stats: AppStats,
}

impl AppData {
    pub const fn new() -> Self {
        Self {
            projects: Vec::new(),
            sessions: Vec::new(),
            stats: AppStats::new(),
        }
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
        self.update_stats();
    }

    pub fn remove_project(&mut self, project_id: &str) -> Option<Project> {
        if let Some(pos) = self.projects.iter().position(|p| p.id == project_id) {
            let removed = self.projects.remove(pos);
            self.update_stats();
            Some(removed)
        } else {
            None
        }
    }

    pub fn get_project(&self, project_id: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.id == project_id)
    }

    pub fn get_project_mut(&mut self, project_id: &str) -> Option<&mut Project> {
        self.projects.iter_mut().find(|p| p.id == project_id)
    }

    pub fn add_session(&mut self, session: Session) {
        self.sessions.push(session);
        self.update_stats();
    }

    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut Session> {
        self.sessions.iter_mut().find(|s| s.id == session_id)
    }

    pub fn cleanup_missing_projects(&mut self) -> Vec<String> {
        let mut removed_ids = Vec::new();
        self.projects.retain(|project| {
            if project.exists() {
                true
            } else {
                removed_ids.push(project.id.clone());
                false
            }
        });
        if !removed_ids.is_empty() {
            self.update_stats();
        }
        removed_ids
    }

    fn update_stats(&mut self) {
        self.stats.total_projects = self.projects.len();
        self.stats.active_sessions = self
            .sessions
            .iter()
            .filter(|s| matches!(s.status, SessionStatus::Active))
            .count();
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_project_creation() {
        let project = Project::new("test-project".to_string(), PathBuf::from("/tmp/test"));

        assert_eq!(project.name, "test-project");
        assert_eq!(project.path, PathBuf::from("/tmp/test"));
        assert!(project.last_accessed.is_none());
        assert!(!project.id.is_empty());
    }

    #[test]
    fn test_project_update_last_accessed() {
        let mut project = Project::new("test".to_string(), PathBuf::from("/tmp"));
        assert!(project.last_accessed.is_none());

        project.update_last_accessed();
        assert!(project.last_accessed.is_some());
    }

    #[test]
    fn test_project_exists() {
        // Test with existing directory
        let temp_dir = TempDir::new().unwrap();
        let project = Project::new("test".to_string(), temp_dir.path().to_path_buf());
        assert!(project.exists());

        // Test with non-existing directory
        let project = Project::new("test".to_string(), PathBuf::from("/non/existent/path"));
        assert!(!project.exists());
    }

    #[test]
    fn test_session_creation() {
        let session = Session::new(Some("project-id".to_string()));

        assert_eq!(session.project_id, Some("project-id".to_string()));
        assert_eq!(session.status, SessionStatus::Active);
        assert!(!session.id.is_empty());
    }

    #[test]
    fn test_session_status_changes() {
        let mut session = Session::new(None);
        assert_eq!(session.status, SessionStatus::Active);

        session.stop();
        assert_eq!(session.status, SessionStatus::Stopped);

        session.set_error();
        assert_eq!(session.status, SessionStatus::Error);
    }

    #[test]
    fn test_app_stats_runtime() {
        let mut stats = AppStats::new();
        let duration = Duration::from_secs(3600); // 1 hour

        stats.set_total_runtime(duration);
        assert_eq!(stats.get_total_runtime(), duration);
        assert_eq!(stats.total_runtime, 3600);
    }

    #[test]
    fn test_app_data_project_management() {
        let mut app_data = AppData::new();
        let project = Project::new("test".to_string(), PathBuf::from("/tmp"));
        let project_id = project.id.clone();

        // Add project
        app_data.add_project(project);
        assert_eq!(app_data.projects.len(), 1);
        assert_eq!(app_data.stats.total_projects, 1);

        // Get project
        let retrieved = app_data.get_project(&project_id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "test");

        // Remove project
        let removed = app_data.remove_project(&project_id);
        assert!(removed.is_some());
        assert_eq!(app_data.projects.len(), 0);
        assert_eq!(app_data.stats.total_projects, 0);
    }

    #[test]
    fn test_app_data_session_management() {
        let mut app_data = AppData::new();
        let session = Session::new(None);
        let session_id = session.id.clone();

        app_data.add_session(session);
        assert_eq!(app_data.sessions.len(), 1);
        assert_eq!(app_data.stats.active_sessions, 1);

        // Stop session
        let session_mut = app_data.get_session_mut(&session_id);
        assert!(session_mut.is_some());
        session_mut.unwrap().stop();

        // Update stats manually (in real app this would be done automatically)
        app_data.update_stats();
        assert_eq!(app_data.stats.active_sessions, 0);
    }

    #[test]
    fn test_cleanup_missing_projects() {
        let mut app_data = AppData::new();

        // Add existing project
        let temp_dir = TempDir::new().unwrap();
        let existing_project = Project::new("existing".to_string(), temp_dir.path().to_path_buf());
        let existing_id = existing_project.id.clone();
        app_data.add_project(existing_project);

        // Add non-existing project
        let missing_project = Project::new("missing".to_string(), PathBuf::from("/non/existent"));
        let missing_id = missing_project.id.clone();
        app_data.add_project(missing_project);

        assert_eq!(app_data.projects.len(), 2);

        // Cleanup
        let removed_ids = app_data.cleanup_missing_projects();

        assert_eq!(app_data.projects.len(), 1);
        assert_eq!(removed_ids.len(), 1);
        assert_eq!(removed_ids[0], missing_id);
        assert_eq!(app_data.projects[0].id, existing_id);
    }

    #[test]
    fn test_serialization() {
        let app_data = AppData::new();

        // Test serialization
        let json = serde_json::to_string(&app_data).unwrap();
        assert!(!json.is_empty());

        // Test deserialization
        let deserialized: AppData = serde_json::from_str(&json).unwrap();
        assert_eq!(app_data, deserialized);
    }
}
