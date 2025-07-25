# Claude Control Implementation Plan

## Overview

This document outlines the implementation plan for major UI and functionality enhancements to claudectl, including sessions, projects, stats display, floating help menu, project management with file picker, and data persistence.

## 🎯 Requirements Summary

1. **UI Reconfiguration**: Display sessions, projects, and stats in main interface
2. **Floating Help Menu**: Toggle with '?' key showing all keyboard shortcuts
3. **Project Management**: Add/remove projects (repositories) with persistence
4. **File Picker**: Navigate filesystem to select project directories
5. **Data Persistence**: Projects persist across application restarts
6. **Data Validation**: Remove non-existent project directories

## 🏗️ Architecture Changes

### Current State
- Simple `App` struct with only `should_quit` field
- Component-based UI rendering
- Basic keyboard handling (q/ESC to quit)
- No state persistence

### Required State
- **Application State**: Current view mode (normal, help modal, file picker)
- **Project Data**: List of projects with paths and metadata
- **UI State**: Selected items, navigation positions, modal states
- **Session Data**: Active sessions and their status
- **Stats Data**: Usage statistics and metrics

## 📋 Phase 1: Foundation & Data Layer

### 1.1 Data Structures (`src/data/mod.rs`)

```rust
// Core data structures
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
}

pub struct Session {
    pub id: String,
    pub project_id: Option<String>,
    pub status: SessionStatus,
    pub created_at: DateTime<Utc>,
}

pub struct AppStats {
    pub total_projects: usize,
    pub active_sessions: usize,
    pub total_runtime: Duration,
}

pub struct AppData {
    pub projects: Vec<Project>,
    pub sessions: Vec<Session>,
    pub stats: AppStats,
}
```

### 1.2 Persistence Layer (`src/storage/mod.rs`)

```rust
pub trait Storage {
    fn load(&self) -> Result<AppData, StorageError>;
    fn save(&self, data: &AppData) -> Result<(), StorageError>;
    fn validate_projects(&self, projects: &mut Vec<Project>) -> Vec<String>; // Returns removed project IDs
}

pub struct JsonStorage {
    config_path: PathBuf,
}
```

**Implementation Details:**
- Store data in `~/.config/claudectl/data.json`
- Handle missing directories gracefully
- Implement atomic writes to prevent corruption
- Add migration support for future schema changes

### 1.3 Enhanced App State (`src/app.rs`)

```rust
pub enum AppMode {
    Normal,
    HelpModal,
    FilePickerModal,
    ConfirmationModal(String),
}

pub struct App {
    pub should_quit: bool,
    pub mode: AppMode,
    pub data: AppData,
    pub storage: Box<dyn Storage>,
    
    // UI State
    pub selected_project: Option<usize>,
    pub file_picker_state: Option<FilePickerState>,
    pub help_visible: bool,
}
```

## 📋 Phase 2: UI Component Restructure

### 2.1 New Component Architecture

```
src/components/
├── mod.rs
├── header.rs              (existing, minor updates)
├── footer.rs              (existing, update shortcuts)
├── sessions_panel.rs      (new)
├── projects_panel.rs      (new)
├── stats_panel.rs         (new)
├── modals/
│   ├── mod.rs
│   ├── help_modal.rs      (new)
│   ├── file_picker.rs     (new)
│   └── confirmation.rs    (new)
└── layout.rs              (new - layout management)
```

### 2.2 Main Layout Redesign

**New Layout Structure:**
```
┌─────────────────────────────────────────────────────────────┐
│ Header (claudectl v0.1.0)                                  │
├─────────────────────────────────────────────────────────────┤
│ Sessions (30%)    │ Projects (45%)     │ Stats (25%)       │
│                   │                    │                   │
│ □ Session 1       │ 📁 project-alpha   │ 📊 Total: 5      │
│ □ Session 2       │ 📁 project-beta    │ 🚀 Active: 2     │
│ + New Session     │ 📁 project-gamma   │ ⏱️  Uptime: 2h   │
│                   │                    │ 💾 Size: 120MB   │
│                   │ [P] Add Project    │                   │
│                   │ [D] Remove Project │                   │
├─────────────────────────────────────────────────────────────┤
│ Footer (Controls: ? Help | P Add Project | Q Quit)         │
└─────────────────────────────────────────────────────────────┘
```

### 2.3 Modal System

**Help Modal (triggered by '?'):**
```
┌─────────────────────────────────────────────────────────────┐
│                      Keyboard Shortcuts                     │
├─────────────────────────────────────────────────────────────┤
│ General:                                                    │
│   q, ESC        Quit application                           │
│   ?             Toggle this help menu                      │
│                                                            │
│ Projects:                                                  │
│   p             Add new project                            │
│   d             Remove selected project                    │
│   ↑/↓           Navigate project list                      │
│   Enter         Open project                               │
│                                                            │
│ Sessions:                                                  │
│   n             New session                                │
│   s             Stop selected session                      │
│                                                            │
│ Press ? or ESC to close                                    │
└─────────────────────────────────────────────────────────────┘
```

**File Picker Modal:**
```
┌─────────────────────────────────────────────────────────────┐
│                    Select Project Directory                  │
├─────────────────────────────────────────────────────────────┤
│ Current: /Users/username/Development                        │
├─────────────────────────────────────────────────────────────┤
│ > 📁 ..                                                     │
│   📁 project-alpha                                          │
│   📁 project-beta                                           │
│   📁 my-website                                             │
│   📁 rust-experiments                                       │
│   📄 README.md                                              │
├─────────────────────────────────────────────────────────────┤
│ Enter: Select/Navigate | Backspace: Go Up | ESC: Cancel    │
└─────────────────────────────────────────────────────────────┘
```

## 📋 Phase 3: File Picker Implementation

### 3.1 File Picker State Management

```rust
pub struct FilePickerState {
    pub current_path: PathBuf,
    pub entries: Vec<DirEntry>,
    pub selected_index: usize,
    pub error_message: Option<String>,
}

impl FilePickerState {
    pub fn navigate_to(&mut self, path: PathBuf) -> Result<(), io::Error>;
    pub fn navigate_up(&mut self) -> Result<(), io::Error>;
    pub fn get_selected_entry(&self) -> Option<&DirEntry>;
    pub fn select_current_directory(&self) -> PathBuf;
}
```

### 3.2 File Picker Navigation

**Keyboard Controls:**
- `↑/↓`: Navigate entries
- `Enter`: Enter directory or select if directory
- `Backspace`: Go up one level
- `ESC`: Cancel and close picker
- `Tab`: Toggle between files/folders view

### 3.3 Directory Validation

```rust
pub fn validate_project_directory(path: &Path) -> ProjectValidationResult {
    // Check if directory exists
    // Check if it's readable
    // Check if it looks like a project (optional: .git, package.json, etc.)
    // Return validation result with suggestions
}
```

## 📋 Phase 4: Event Handling Overhaul

### 4.1 Enhanced Event System

```rust
pub enum AppEvent {
    Quit,
    ToggleHelp,
    AddProject,
    RemoveProject,
    NavigateUp,
    NavigateDown,
    Select,
    Cancel,
    // File picker events
    FilePickerNavigate(PathBuf),
    FilePickerSelect,
    FilePickerCancel,
}

pub struct EventHandler {
    app: App,
}

impl EventHandler {
    pub fn handle_key_event(&mut self, key: KeyCode) -> Result<(), AppError> {
        match self.app.mode {
            AppMode::Normal => self.handle_normal_mode(key),
            AppMode::HelpModal => self.handle_help_modal(key),
            AppMode::FilePickerModal => self.handle_file_picker(key),
            AppMode::ConfirmationModal(_) => self.handle_confirmation(key),
        }
    }
}
```

### 4.2 Keyboard Shortcut Mapping

```rust
pub fn map_key_to_event(key: KeyCode, mode: &AppMode) -> Option<AppEvent> {
    match mode {
        AppMode::Normal => match key {
            KeyCode::Char('q') | KeyCode::Esc => Some(AppEvent::Quit),
            KeyCode::Char('?') => Some(AppEvent::ToggleHelp),
            KeyCode::Char('p') => Some(AppEvent::AddProject),
            KeyCode::Char('d') => Some(AppEvent::RemoveProject),
            KeyCode::Up => Some(AppEvent::NavigateUp),
            KeyCode::Down => Some(AppEvent::NavigateDown),
            KeyCode::Enter => Some(AppEvent::Select),
            _ => None,
        },
        // ... other modes
    }
}
```

## 📋 Phase 5: Integration & Polish

### 5.1 Error Handling

```rust
#[derive(Debug)]
pub enum AppError {
    Storage(StorageError),
    Filesystem(io::Error),
    InvalidProject(String),
    ConfigurationError(String),
}
```

### 5.2 Configuration Management

```rust
pub struct AppConfig {
    pub data_directory: PathBuf,
    pub max_recent_projects: usize,
    pub auto_cleanup_missing: bool,
    pub file_picker_show_hidden: bool,
}
```

### 5.3 Testing Strategy

- **Unit Tests**: Data structures, storage layer, validation logic
- **Integration Tests**: Component rendering, event handling
- **Manual Testing**: File picker navigation, modal interactions

## 🚀 Implementation Timeline

### Week 1: Foundation
- [x] Implement data structures and storage layer
- [x] Create enhanced App state management
- [x] Set up persistence with JSON storage
- [x] Add project validation logic

### Week 2: UI Restructure
- [ ] Create new panel components (sessions, projects, stats)
- [ ] Implement main layout redesign
- [ ] Update existing components for new layout
- [ ] Test basic rendering and navigation

### Week 3: Interactive Features
- [ ] Implement file picker component
- [ ] Add help modal component
- [ ] Create enhanced event handling system
- [ ] Integrate keyboard shortcuts

### Week 4: Integration & Polish
- [ ] Connect all components together
- [ ] Add error handling and edge cases
- [ ] Implement configuration management
- [ ] Add comprehensive testing
- [ ] Documentation and cleanup

## 🔧 Technical Considerations

### Dependencies to Add
```toml
[dependencies]
# Existing
clap = { version = "4.4", features = ["derive"] }
ratatui = "0.26"
crossterm = "0.27"
tokio = { version = "1.0", features = ["full"] }

# New dependencies
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
dirs = "5.0"  # For config directory
uuid = { version = "1.0", features = ["v4"] }
```

### Performance Considerations
- Lazy loading of large project lists
- Efficient directory scanning for file picker
- Debounced keyboard input for smooth navigation
- Caching of frequently accessed data

### Security Considerations
- Path traversal prevention in file picker
- Safe handling of filesystem permissions
- Input validation for project names/paths
- Atomic file operations to prevent corruption

## 📝 Success Criteria

### Functional Requirements
- ✅ UI displays sessions, projects, and stats
- ✅ Help modal toggles with '?' key
- ✅ Projects can be added via file picker
- ✅ Projects can be removed with confirmation
- ✅ Data persists across application restarts
- ✅ Non-existent project directories are cleaned up

### Quality Requirements
- ✅ Responsive keyboard navigation
- ✅ Clear visual feedback for all actions
- ✅ Graceful error handling and recovery
- ✅ Intuitive user experience
- ✅ Clean, maintainable code architecture

## 🔄 Future Enhancements (Post-MVP)

1. **Session Management**: Create, stop, and monitor active sessions
2. **Project Templates**: Quick project creation from templates
3. **Search & Filter**: Search projects by name or path
4. **Project Metadata**: Git branch, last commit, project size
5. **Themes & Customization**: User-configurable UI themes
6. **Backup & Sync**: Cloud sync for project configurations
7. **Plugin System**: Extensible architecture for custom features

---

**Next Steps**: Begin implementation with Phase 1 (Foundation & Data Layer), starting with data structures and storage implementation.