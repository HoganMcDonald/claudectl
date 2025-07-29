use crate::data::{AppData, Session, SessionData};
use crate::process::ProcessManager;
use crate::storage::{JsonStorage, SessionStorage, Storage, StorageError};
use crossterm::event::KeyCode;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    HelpModal,
    FilePickerModal,
    ConfirmationModal(String),
    ProjectInitModal,
    MetricsModal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FocusArea {
    Projects,
    Sessions,
}

#[derive(Debug, Clone)]
pub struct FilePickerState {
    pub current_path: PathBuf,
    pub entries: Vec<DirEntry>,
    pub selected_index: usize,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
}

impl FilePickerState {
    pub fn new(initial_path: PathBuf) -> Result<Self, std::io::Error> {
        let mut state = Self {
            current_path: initial_path,
            entries: Vec::new(),
            selected_index: 0,
            error_message: None,
        };
        state.refresh_entries()?;
        Ok(state)
    }

    pub fn refresh_entries(&mut self) -> Result<(), std::io::Error> {
        self.entries.clear();
        self.selected_index = 0;
        self.error_message = None;

        // Add parent directory entry if not at root
        if let Some(parent) = self.current_path.parent() {
            self.entries.push(DirEntry {
                name: "..".to_string(),
                path: parent.to_path_buf(),
                is_directory: true,
            });
        }

        // Read directory entries
        let entries = std::fs::read_dir(&self.current_path)?;
        let mut dir_entries = Vec::new();

        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip hidden files/directories (starting with .)
            if name.starts_with('.') && name != ".." {
                continue;
            }

            dir_entries.push(DirEntry {
                name,
                path: entry.path(),
                is_directory: metadata.is_dir(),
            });
        }

        // Sort: directories first, then files, both alphabetically
        dir_entries.sort_by(|a, b| match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        self.entries.extend(dir_entries);
        Ok(())
    }

    pub fn navigate_to(&mut self, path: PathBuf) -> Result<(), std::io::Error> {
        if path.is_dir() {
            self.current_path = path;
            self.refresh_entries()?;
        }
        Ok(())
    }

    pub fn navigate_up(&mut self) -> Result<(), std::io::Error> {
        if let Some(parent) = self.current_path.parent() {
            self.navigate_to(parent.to_path_buf())?;
        }
        Ok(())
    }

    pub fn get_selected_entry(&self) -> Option<&DirEntry> {
        self.entries.get(self.selected_index)
    }

    pub fn select_current_directory(&self) -> PathBuf {
        self.current_path.clone()
    }

    pub const fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub const fn move_selection_down(&mut self) {
        if self.selected_index < self.entries.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Process error: {0}")]
    Process(#[from] crate::process::ProcessError),
    #[error("Project not found: {0}")]
    ProjectNotFound(String),
    #[error("Invalid project path: {0}")]
    InvalidProjectPath(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvent {
    Quit,
    ToggleHelp,
    ToggleMetrics,
    AddProject,
    RemoveProject,
    NavigateUp,
    NavigateDown,
    Select,
    Cancel,
    // File picker specific events
    FilePickerNavigate(PathBuf),
    FilePickerSelect,
    FilePickerCancel,
    FilePickerUp,
    FilePickerDown,
    // Session events
    NewSession,
    StopSession,
    DeleteSession,
    SelectSession,
    // Focus events
    SwitchFocus,
    // Project initialization events
    ProjectInitChar(char),
    ProjectInitBackspace,
    ProjectInitSubmit,
}

pub struct App {
    pub should_quit: bool,
    pub mode: AppMode,
    pub data: AppData,
    pub session_data: SessionData,
    pub storage: Box<dyn Storage>,
    pub session_storage: Box<dyn SessionStorage>,
    pub process_manager: Arc<ProcessManager>,

    // UI State
    pub focus_area: FocusArea,
    pub selected_project_index: Option<usize>,
    pub selected_session_index: Option<usize>,
    pub selected_session_output: Option<String>,
    pub file_picker_state: Option<FilePickerState>,
    pub error_message: Option<String>,
    
    // Project initialization state
    pub project_init_name: String,
    pub project_init_cursor_visible: bool,
}

impl App {
    pub fn new() -> Result<Self, AppError> {
        // Use global storage for projects (backward compatibility)
        let storage = Box::new(JsonStorage::new()?);
        let mut data = storage.load()?;
        
        // Use project-specific storage for sessions
        let session_storage = Box::new(JsonStorage::new_for_sessions()?);
        let session_data = session_storage.load_sessions()?;
        
        // Remove sessions from AppData since they're now stored separately
        data.sessions.clear();
        data.stats.active_sessions = session_data.stats.active_sessions;
        
        let process_manager = Arc::new(ProcessManager::new());
        
        // Check if we need to show project initialization modal
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let needs_init = !crate::project_init::ProjectInitializer::has_claudectl_dir(&current_dir);
        let default_name = if needs_init {
            crate::project_init::ProjectInitializer::get_default_project_name(&current_dir)
        } else {
            String::new()
        };

        Ok(Self {
            should_quit: false,
            mode: if needs_init { AppMode::ProjectInitModal } else { AppMode::Normal },
            data,
            session_data,
            storage,
            session_storage,
            process_manager,
            focus_area: FocusArea::Sessions,
            selected_project_index: None,
            selected_session_index: None,
            selected_session_output: None,
            file_picker_state: None,
            error_message: None,
            project_init_name: default_name,
            project_init_cursor_visible: true,
        })
    }

    pub fn with_storage(storage: Box<dyn Storage>) -> Result<Self, AppError> {
        let mut data = storage.load()?;
        
        // Use project-specific storage for sessions
        let session_storage = Box::new(JsonStorage::new_for_sessions()?);
        let session_data = session_storage.load_sessions()?;
        
        // Remove sessions from AppData since they're now stored separately
        data.sessions.clear();
        data.stats.active_sessions = session_data.stats.active_sessions;
        
        let process_manager = Arc::new(ProcessManager::new());

        Ok(Self {
            should_quit: false,
            mode: AppMode::Normal,
            data,
            session_data,
            storage,
            session_storage,
            process_manager,
            focus_area: FocusArea::Sessions,
            selected_project_index: None,
            selected_session_index: None,
            selected_session_output: None,
            file_picker_state: None,
            error_message: None,
            project_init_name: String::new(),
            project_init_cursor_visible: true,
        })
    }

    pub fn save_data(&mut self) -> Result<(), AppError> {
        self.storage.save(&self.data)?;
        Ok(())
    }

    pub fn save_session_data(&mut self) -> Result<(), AppError> {
        self.session_storage.save_sessions(&self.session_data)?;
        Ok(())
    }

    pub fn save_all_data(&mut self) -> Result<(), AppError> {
        self.save_data()?;
        self.save_session_data()?;
        Ok(())
    }

    pub fn handle_key_event(&mut self, key: KeyCode) -> Result<(), AppError> {
        let event = self.map_key_to_event(key);
        if let Some(event) = event {
            self.handle_event(event)?;
        }
        Ok(())
    }

    const fn map_key_to_event(&self, key: KeyCode) -> Option<AppEvent> {
        match self.mode {
            AppMode::Normal => match key {
                KeyCode::Char('q') | KeyCode::Esc => Some(AppEvent::Quit),
                KeyCode::Char('?') => Some(AppEvent::ToggleHelp),
                KeyCode::Char('m') => Some(AppEvent::ToggleMetrics),
                KeyCode::Char('p') => Some(AppEvent::AddProject),
                KeyCode::Char('d') => Some(AppEvent::RemoveProject),
                KeyCode::Char('n') => Some(AppEvent::NewSession),
                KeyCode::Char('s') => Some(AppEvent::StopSession),
                KeyCode::Char('x') => Some(AppEvent::DeleteSession),
                KeyCode::Up | KeyCode::Char('k') => Some(AppEvent::NavigateUp),
                KeyCode::Down | KeyCode::Char('j') => Some(AppEvent::NavigateDown),
                KeyCode::Enter => Some(AppEvent::Select),
                KeyCode::Tab => Some(AppEvent::SwitchFocus),
                _ => None,
            },
            AppMode::HelpModal => match key {
                KeyCode::Char('?') | KeyCode::Esc => Some(AppEvent::ToggleHelp),
                _ => None,
            },
            AppMode::MetricsModal => match key {
                KeyCode::Char('m') | KeyCode::Esc => Some(AppEvent::ToggleMetrics),
                _ => None,
            },
            AppMode::FilePickerModal => match key {
                KeyCode::Esc => Some(AppEvent::FilePickerCancel),
                KeyCode::Enter => Some(AppEvent::FilePickerSelect),
                KeyCode::Up => Some(AppEvent::FilePickerUp),
                KeyCode::Down => Some(AppEvent::FilePickerDown),
                KeyCode::Backspace => Some(AppEvent::NavigateUp),
                _ => None,
            },
            AppMode::ConfirmationModal(_) => match key {
                KeyCode::Char('y') | KeyCode::Enter => Some(AppEvent::Select),
                KeyCode::Char('n') | KeyCode::Esc => Some(AppEvent::Cancel),
                _ => None,
            },
            AppMode::ProjectInitModal => match key {
                KeyCode::Esc => Some(AppEvent::Quit),
                KeyCode::Enter => Some(AppEvent::ProjectInitSubmit),
                KeyCode::Backspace => Some(AppEvent::ProjectInitBackspace),
                KeyCode::Char(c) => Some(AppEvent::ProjectInitChar(c)),
                _ => None,
            },
        }
    }

    fn handle_event(&mut self, event: AppEvent) -> Result<(), AppError> {
        match event {
            AppEvent::Quit => {
                self.should_quit = true;
            }
            AppEvent::ToggleHelp => {
                self.mode = match self.mode {
                    AppMode::HelpModal => AppMode::Normal,
                    _ => AppMode::HelpModal,
                };
            }
            AppEvent::ToggleMetrics => {
                self.mode = match self.mode {
                    AppMode::MetricsModal => AppMode::Normal,
                    _ => AppMode::MetricsModal,
                };
            }
            AppEvent::AddProject => {
                self.start_file_picker()?;
            }
            AppEvent::RemoveProject => {
                self.handle_remove_project()?;
            }
            AppEvent::NavigateUp => {
                self.move_selection_up();
            }
            AppEvent::NavigateDown => {
                self.move_selection_down();
            }
            AppEvent::Select => {
                self.handle_select()?;
            }
            AppEvent::Cancel => {
                self.handle_cancel();
            }
            AppEvent::FilePickerSelect => {
                self.handle_file_picker_select()?;
            }
            AppEvent::FilePickerCancel => {
                self.mode = AppMode::Normal;
                self.file_picker_state = None;
            }
            AppEvent::FilePickerUp => {
                if let Some(ref mut picker) = self.file_picker_state {
                    picker.move_selection_up();
                }
            }
            AppEvent::FilePickerDown => {
                if let Some(ref mut picker) = self.file_picker_state {
                    picker.move_selection_down();
                }
            }
            AppEvent::NewSession => {
                self.handle_new_session()?;
            }
            AppEvent::StopSession => {
                self.handle_stop_session()?;
            }
            AppEvent::DeleteSession => {
                self.handle_delete_session()?;
            }
            AppEvent::SelectSession => {
                self.handle_select_session()?;
            }
            AppEvent::SwitchFocus => {
                self.switch_focus();
            }
            AppEvent::ProjectInitChar(c) => {
                self.project_init_name.push(c);
            }
            AppEvent::ProjectInitBackspace => {
                self.project_init_name.pop();
            }
            AppEvent::ProjectInitSubmit => {
                self.handle_project_init_submit()?;
            }
            _ => {
                // Handle other events as needed
            }
        }
        Ok(())
    }

    fn start_file_picker(&mut self) -> Result<(), AppError> {
        let initial_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let picker_state = FilePickerState::new(initial_path)?;
        self.file_picker_state = Some(picker_state);
        self.mode = AppMode::FilePickerModal;
        Ok(())
    }

    fn handle_file_picker_select(&mut self) -> Result<(), AppError> {
        if let Some(ref mut picker) = self.file_picker_state {
            if let Some(entry) = picker.get_selected_entry() {
                if entry.is_directory {
                    if entry.name == ".." {
                        picker.navigate_up()?;
                    } else {
                        picker.navigate_to(entry.path.clone())?;
                    }
                }
            } else {
                // Select current directory
                let selected_path = picker.select_current_directory();
                self.add_project_from_path(selected_path)?;
                self.mode = AppMode::Normal;
                self.file_picker_state = None;
            }
        }
        Ok(())
    }

    fn add_project_from_path(&mut self, path: PathBuf) -> Result<(), AppError> {
        if !path.exists() || !path.is_dir() {
            return Err(AppError::InvalidProjectPath(
                path.to_string_lossy().to_string(),
            ));
        }

        let project_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let project = crate::data::Project::new(project_name, path);
        self.data.add_project(project);
        self.save_data()?;
        Ok(())
    }

    fn handle_remove_project(&mut self) -> Result<(), AppError> {
        if let Some(index) = self.selected_project_index {
            if index < self.data.projects.len() {
                let project_name = self.data.projects[index].name.clone();
                let message = format!("Remove project '{project_name}'?");
                self.mode = AppMode::ConfirmationModal(message);
            }
        }
        Ok(())
    }

    fn handle_select(&mut self) -> Result<(), AppError> {
        match self.mode {
            AppMode::ConfirmationModal(_) => {
                // Confirm removal
                if let Some(index) = self.selected_project_index {
                    if index < self.data.projects.len() {
                        let project_id = self.data.projects[index].id.clone();
                        self.data.remove_project(&project_id);
                        self.save_data()?;

                        // Adjust selection if necessary
                        if self.selected_project_index.unwrap_or(0) >= self.data.projects.len() {
                            self.selected_project_index = if self.data.projects.is_empty() {
                                None
                            } else {
                                Some(self.data.projects.len() - 1)
                            };
                        }
                    }
                }
                self.mode = AppMode::Normal;
            }
            AppMode::Normal => {
                match self.focus_area {
                    FocusArea::Sessions => {
                        self.handle_select_session()?;
                    }
                    FocusArea::Projects => {
                        // Handle project selection if needed
                    }
                }
            }
            _ => {
                // Handle other select actions
            }
        }
        Ok(())
    }

    fn handle_cancel(&mut self) {
        self.mode = AppMode::Normal;
    }

    fn handle_new_session(&mut self) -> Result<(), AppError> {
        let (project_id, project_path) = self
            .selected_project_index
            .and_then(|index| self.data.projects.get(index))
            .map_or((None, None), |project| (Some(project.id.clone()), Some(project.path.clone())));

        let session = crate::data::Session::new(project_id);
        let session_id = session.id.clone();
        
        self.session_data.add_session(session);
        self.save_session_data()?;

        // Spawn Claude Code instance in background
        let process_manager = Arc::clone(&self.process_manager);
        let session_clone = self.session_data.sessions.iter().find(|s| s.id == session_id).unwrap().clone();
        
        tokio::spawn(async move {
            if let Err(e) = process_manager.spawn_claude_session(&session_clone, project_path).await {
                eprintln!("Failed to spawn Claude Code session: {e}");
            }
        });

        Ok(())
    }

    fn handle_stop_session(&mut self) -> Result<(), AppError> {
        if let Some(index) = self.selected_session_index {
            if let Some(session) = self.session_data.sessions.get_mut(index) {
                let session_id = session.id.clone();
                session.stop();
                self.session_data.update_stats();
                self.save_session_data()?;

                // Stop the Claude Code process
                let process_manager = Arc::clone(&self.process_manager);
                tokio::spawn(async move {
                    if let Err(e) = process_manager.stop_session(&session_id).await {
                        eprintln!("Failed to stop Claude Code session: {e}");
                    }
                });
            }
        }
        Ok(())
    }

    fn handle_delete_session(&mut self) -> Result<(), AppError> {
        if let Some(index) = self.selected_session_index {
            if let Some(session) = self.session_data.sessions.get(index) {
                let session_id = session.id.clone();
                
                // Remove the session from data
                if let Some(_removed_session) = self.session_data.remove_session(&session_id) {
                    // Clear selected session output if this was the selected one
                    self.selected_session_output = None;
                    
                    // Adjust selected index if needed
                    if self.session_data.sessions.is_empty() {
                        self.selected_session_index = None;
                    } else if index >= self.session_data.sessions.len() {
                        self.selected_session_index = Some(self.session_data.sessions.len() - 1);
                    }
                    
                    self.save_session_data()?;

                    // Stop the Claude Code process
                    let process_manager = Arc::clone(&self.process_manager);
                    tokio::spawn(async move {
                        if let Err(e) = process_manager.stop_session(&session_id).await {
                            eprintln!("Failed to stop Claude Code session: {e}");
                        }
                    });
                }
            }
        }
        Ok(())
    }

    const fn move_selection_up(&mut self) {
        match self.focus_area {
            FocusArea::Projects => {
                if let Some(index) = self.selected_project_index {
                    if index > 0 {
                        self.selected_project_index = Some(index - 1);
                    }
                } else if !self.data.projects.is_empty() {
                    self.selected_project_index = Some(self.data.projects.len() - 1);
                }
            }
            FocusArea::Sessions => {
                if let Some(index) = self.selected_session_index {
                    if index > 0 {
                        self.selected_session_index = Some(index - 1);
                    }
                } else if !self.session_data.sessions.is_empty() {
                    self.selected_session_index = Some(self.session_data.sessions.len() - 1);
                }
            }
        }
    }

    const fn move_selection_down(&mut self) {
        match self.focus_area {
            FocusArea::Projects => {
                if let Some(index) = self.selected_project_index {
                    if index < self.data.projects.len().saturating_sub(1) {
                        self.selected_project_index = Some(index + 1);
                    }
                } else if !self.data.projects.is_empty() {
                    self.selected_project_index = Some(0);
                }
            }
            FocusArea::Sessions => {
                if let Some(index) = self.selected_session_index {
                    if index < self.session_data.sessions.len().saturating_sub(1) {
                        self.selected_session_index = Some(index + 1);
                    }
                } else if !self.session_data.sessions.is_empty() {
                    self.selected_session_index = Some(0);
                }
            }
        }
    }

    const fn switch_focus(&mut self) {
        self.focus_area = match self.focus_area {
            FocusArea::Projects => FocusArea::Sessions,
            FocusArea::Sessions => FocusArea::Projects,
        };
    }

    fn handle_select_session(&mut self) -> Result<(), AppError> {
        if let Some(index) = self.selected_session_index {
            if let Some(session) = self.session_data.sessions.get(index) {
                let session_id = session.id.clone();
                let process_manager = Arc::clone(&self.process_manager);
                
                // Get the session output asynchronously
                let session_info = format!(
                    "Session: {}\nStatus: {:?}\nProject: {}\nCreated: {}\n\n--- Output ---\n",
                    &session.id[..8],
                    session.status,
                    session.project_id.as_deref().unwrap_or("No project"),
                    session.created_at.format("%Y-%m-%d %H:%M:%S")
                );

                // For now, set a placeholder until we can get the actual output
                // In a more advanced implementation, we'd make this async
                self.selected_session_output = Some(format!(
                    "{session_info}Loading session output...\n\n(Note: This would show real Claude Code output in a complete implementation)"
                ));

                // Spawn a task to get the real output
                tokio::spawn(async move {
                    if let Some(_output) = process_manager.get_session_output(&session_id).await {
                        // Output retrieved, would update UI in a complete implementation
                    }
                });
            }
        }
        Ok(())
    }

    fn handle_project_init_submit(&mut self) -> Result<(), AppError> {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        
        match crate::project_init::ProjectInitializer::initialize_project(
            &current_dir,
            self.project_init_name.trim().to_string(),
        ) {
            Ok(()) => {
                // Successfully initialized, switch to normal mode
                self.mode = AppMode::Normal;
                self.project_init_name.clear();
            }
            Err(e) => {
                // Handle initialization error
                self.error_message = Some(format!("Failed to initialize project: {e}"));
                // For now, still quit on error, but could show error modal instead
                self.should_quit = true;
            }
        }
        Ok(())
    }

    pub fn cleanup_missing_projects(&mut self) -> Result<Vec<String>, AppError> {
        let removed_ids = self.data.cleanup_missing_projects();
        if !removed_ids.is_empty() {
            self.save_data()?;
        }
        Ok(removed_ids)
    }

    pub fn get_current_project_name(&self) -> String {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        
        // Try to load project config from .claudectl directory
        if let Ok(config) = crate::project_init::ProjectInitializer::load_project_config(&current_dir) {
            config.name
        } else {
            // Fallback to directory name or "claudectl" if no project config exists
            crate::project_init::ProjectInitializer::get_default_project_name(&current_dir)
        }
    }

    /// Restore active sessions by spawning Claude Code processes for sessions marked as Active
    pub async fn restore_active_sessions(&self) -> Result<(), AppError> {
        let active_sessions: Vec<_> = self.session_data.sessions
            .iter()
            .filter(|s| matches!(s.status, crate::data::SessionStatus::Active))
            .collect();

        for session in active_sessions {
            let project_path = session.project_id
                .as_ref()
                .and_then(|id| self.data.get_project(id))
                .map(|p| p.path.clone());

            if let Err(e) = self.process_manager.spawn_claude_session(session, project_path).await {
                eprintln!("Failed to restore session {}: {}", session.id, e);
                // Mark session as error state instead of stopping it
                // This would require making session mutable, so we'll just log for now
            }
        }
        Ok(())
    }

    /// Update session statuses based on actual process states
    pub async fn sync_session_statuses(&mut self) -> Result<(), AppError> {
        let statuses = self.process_manager.get_session_statuses().await;
        let mut needs_save = false;

        for session in &mut self.session_data.sessions {
            if matches!(session.status, crate::data::SessionStatus::Active) {
                if let Some(&is_running) = statuses.get(&session.id) {
                    if !is_running {
                        session.set_error();
                        needs_save = true;
                    }
                }
            }
        }

        if needs_save {
            self.session_data.update_stats();
            self.save_session_data()?;
        }

        Ok(())
    }

    /// Cleanup all processes on app shutdown
    pub async fn cleanup_on_shutdown(&self) {
        self.process_manager.cleanup_all().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::JsonStorage;
    use tempfile::TempDir;

    fn create_test_app() -> (App, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage = Box::new(JsonStorage::with_custom_path(temp_dir.path().to_path_buf()));
        let app = App::with_storage(storage).unwrap();
        (app, temp_dir)
    }

    #[test]
    #[ignore]
    fn test_app_creation() {
        let (app, _temp_dir) = create_test_app();

        assert!(!app.should_quit);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.data.projects.len(), 0);
        assert_eq!(app.session_data.sessions.len(), 0);
    }

    #[test]
    fn test_key_mapping_normal_mode() {
        let (app, _temp_dir) = create_test_app();

        assert_eq!(
            app.map_key_to_event(KeyCode::Char('q')),
            Some(AppEvent::Quit)
        );
        assert_eq!(
            app.map_key_to_event(KeyCode::Char('?')),
            Some(AppEvent::ToggleHelp)
        );
        assert_eq!(
            app.map_key_to_event(KeyCode::Char('m')),
            Some(AppEvent::ToggleMetrics)
        );
        assert_eq!(
            app.map_key_to_event(KeyCode::Char('p')),
            Some(AppEvent::AddProject)
        );
        assert_eq!(
            app.map_key_to_event(KeyCode::Up),
            Some(AppEvent::NavigateUp)
        );
        assert_eq!(app.map_key_to_event(KeyCode::Char('x')), None);
    }

    #[test]
    fn test_help_modal_toggle() {
        let (mut app, _temp_dir) = create_test_app();

        app.handle_key_event(KeyCode::Char('?')).unwrap();
        assert_eq!(app.mode, AppMode::HelpModal);

        app.handle_key_event(KeyCode::Char('?')).unwrap();
        assert_eq!(app.mode, AppMode::Normal);
    }

    #[test]
    fn test_metrics_modal_toggle() {
        let (mut app, _temp_dir) = create_test_app();

        // Test opening metrics modal
        app.handle_key_event(KeyCode::Char('m')).unwrap();
        assert_eq!(app.mode, AppMode::MetricsModal);

        // Test closing metrics modal with 'm'
        app.handle_key_event(KeyCode::Char('m')).unwrap();
        assert_eq!(app.mode, AppMode::Normal);

        // Test opening again
        app.handle_key_event(KeyCode::Char('m')).unwrap();
        assert_eq!(app.mode, AppMode::MetricsModal);

        // Test closing metrics modal with Esc
        app.handle_key_event(KeyCode::Esc).unwrap();
        assert_eq!(app.mode, AppMode::Normal);
    }

    #[test]
    fn test_file_picker_state() {
        let temp_dir = TempDir::new().unwrap();
        let picker = FilePickerState::new(temp_dir.path().to_path_buf()).unwrap();

        assert_eq!(picker.current_path, temp_dir.path());
        assert_eq!(picker.selected_index, 0);
        assert!(picker.error_message.is_none());
    }

    #[test]
    fn test_file_picker_navigation() {
        let temp_dir = TempDir::new().unwrap();
        let mut picker = FilePickerState::new(temp_dir.path().to_path_buf()).unwrap();

        // Test selection movement
        let initial_len = picker.entries.len();
        if initial_len > 1 {
            picker.move_selection_down();
            assert_eq!(picker.selected_index, 1);

            picker.move_selection_up();
            assert_eq!(picker.selected_index, 0);
        }
    }

    #[test]
    fn test_add_project_from_path() {
        let (mut app, temp_dir) = create_test_app();

        // Create a test directory
        let project_dir = temp_dir.path().join("test-project");
        std::fs::create_dir(&project_dir).unwrap();

        app.add_project_from_path(project_dir).unwrap();

        assert_eq!(app.data.projects.len(), 1);
        assert_eq!(app.data.projects[0].name, "test-project");
    }

    #[test]
    fn test_add_invalid_project_path() {
        let (mut app, _temp_dir) = create_test_app();

        let invalid_path = PathBuf::from("/non/existent/path");
        let result = app.add_project_from_path(invalid_path);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AppError::InvalidProjectPath(_)
        ));
    }

    #[test]
    fn test_project_selection() {
        let (mut app, temp_dir) = create_test_app();

        // Add test projects
        let project1_dir = temp_dir.path().join("project1");
        let project2_dir = temp_dir.path().join("project2");
        std::fs::create_dir(&project1_dir).unwrap();
        std::fs::create_dir(&project2_dir).unwrap();

        app.add_project_from_path(project1_dir).unwrap();
        app.add_project_from_path(project2_dir).unwrap();

        // Set focus to projects for testing navigation
        app.focus_area = FocusArea::Projects;

        // Test navigation
        app.move_selection_down();
        assert_eq!(app.selected_project_index, Some(0));

        app.move_selection_down();
        assert_eq!(app.selected_project_index, Some(1));

        app.move_selection_up();
        assert_eq!(app.selected_project_index, Some(0));
    }

    #[tokio::test]
    #[ignore]
    async fn test_session_management() {
        let (mut app, temp_dir) = create_test_app();

        // Add a project first
        let project_dir = temp_dir.path().join("test-project");
        std::fs::create_dir(&project_dir).unwrap();
        app.add_project_from_path(project_dir).unwrap();
        app.selected_project_index = Some(0);

        // Create a session
        app.handle_new_session().unwrap();

        assert_eq!(app.session_data.sessions.len(), 1);
        assert_eq!(
            app.session_data.sessions[0].project_id,
            Some(app.data.projects[0].id.clone())
        );

        // Stop the session
        app.selected_session_index = Some(0);
        app.handle_stop_session().unwrap();

        assert_eq!(
            app.session_data.sessions[0].status,
            crate::data::SessionStatus::Stopped
        );
    }

    #[tokio::test]
    async fn test_delete_session() {
        let (mut app, _temp_dir) = create_test_app();
        
        // Clear any existing sessions and add our test session
        app.session_data.sessions.clear();
        let session = Session::new(None);
        app.session_data.add_session(session);
        app.selected_session_index = Some(0);
        
        assert_eq!(app.session_data.sessions.len(), 1);
        
        // Delete the session
        app.handle_delete_session().unwrap();
        
        // Verify session was deleted
        assert_eq!(app.session_data.sessions.len(), 0);
        assert_eq!(app.selected_session_index, None);
        assert_eq!(app.selected_session_output, None);
    }

    #[test]
    fn test_confirmation_modal() {
        let (mut app, temp_dir) = create_test_app();

        // Add a project
        let project_dir = temp_dir.path().join("test-project");
        std::fs::create_dir(&project_dir).unwrap();
        app.add_project_from_path(project_dir).unwrap();
        app.selected_project_index = Some(0);

        // Trigger remove
        app.handle_remove_project().unwrap();
        assert!(matches!(app.mode, AppMode::ConfirmationModal(_)));

        // Confirm removal
        app.handle_select().unwrap();
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.data.projects.len(), 0);
    }

    #[test]
    fn test_data_persistence() {
        let temp_dir = TempDir::new().unwrap();
        let storage_path = temp_dir.path().to_path_buf();

        // Create app and add project
        {
            let storage = Box::new(JsonStorage::with_custom_path(storage_path.clone()));
            let mut app = App::with_storage(storage).unwrap();

            let project_dir = temp_dir.path().join("test-project");
            std::fs::create_dir(&project_dir).unwrap();
            app.add_project_from_path(project_dir).unwrap();
        }

        // Create new app instance and verify data persisted
        {
            let storage = Box::new(JsonStorage::with_custom_path(storage_path));
            let app = App::with_storage(storage).unwrap();

            assert_eq!(app.data.projects.len(), 1);
            assert_eq!(app.data.projects[0].name, "test-project");
        }
    }
}
