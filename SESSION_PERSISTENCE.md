# Session Persistence Implementation

This document describes the implementation of background Claude Code instance spawning and session persistence in claudectl.

## Overview

The implementation allows claudectl to spawn Claude Code instances in the background that persist even if the claudectl TUI is closed. When starting a new session, a background Claude Code process is automatically spawned and managed.

## Key Components

### 1. Process Manager (`src/process.rs`)

The `ProcessManager` handles spawning, managing, and cleaning up Claude Code background processes:

- **Spawning**: Creates new Claude Code instances using `tokio::process::Command`
- **Management**: Tracks active processes and their states
- **Cleanup**: Gracefully terminates processes on session stop or app shutdown
- **Status Monitoring**: Checks process health and updates session states

Key features:
- Async process management using Tokio
- Automatic process cleanup with `kill_on_drop(true)`
- Error handling for Claude Code executable availability
- Support for project-specific working directories

### 2. App Integration (`src/app.rs`)

Enhanced the main App structure with:

- **Process Manager Integration**: Added `Arc<ProcessManager>` to enable shared async access
- **Session Restoration**: `restore_active_sessions()` method to restart processes on app startup
- **Status Synchronization**: `sync_session_statuses()` to update session states based on actual process status
- **Graceful Shutdown**: `cleanup_on_shutdown()` to terminate all processes when quitting

### 3. TUI Updates (`src/tui.rs`)

Modified the TUI to support async operations:

- **Async Event Loop**: Made `run_app()` async to handle process management
- **Periodic Sync**: Added periodic session status synchronization (every 10 seconds)
- **Session Restoration**: Automatically restores active sessions on startup
- **Process Cleanup**: Ensures all background processes are terminated on exit

### 4. Session Lifecycle

**Creating a New Session:**
1. User presses 'n' to create new session
2. Session data is created and saved to project-specific `.claudectl/sessions.json`
3. Background Claude Code process is spawned in the project directory
4. Process is tracked in ProcessManager

**Session Persistence:**
1. Sessions are stored in the project-specific `.claudectl/` directory
2. Only sessions for the current project are loaded when claudectl starts
3. Sessions marked as "Active" are restored on app startup
4. Background processes continue running even if claudectl is closed
5. Session status is periodically synchronized with actual process state

**Project Isolation:**
1. Each project has its own `.claudectl/sessions.json` file
2. Sessions from different projects are completely isolated
3. When you switch between projects, you only see sessions for that project
4. This prevents session conflicts between different development projects

**Stopping a Session:**
1. User stops session through UI
2. Session status is updated to "Stopped"
3. Background Claude Code process is gracefully terminated

**Error Handling:**
- Failed process spawns are logged but don't crash the app
- Dead processes are automatically detected and cleaned up
- Session states are updated to reflect actual process status

## Usage

### Prerequisites

- Claude Code executable must be available in PATH
- Tokio runtime is required (already included in dependencies)

### Starting a Session

1. Run `claudectl` (or just `claudectl tui`)
2. Select a project (or work without a project)
3. Press 'n' to create a new session
4. A Claude Code instance will spawn in the background
5. The session will persist even if you close claudectl

### Managing Sessions

- **New Session**: Press 'n'
- **Stop Session**: Select a session and press 's'
- **View Sessions**: Active sessions are displayed in the Sessions panel

### Session Persistence

Sessions automatically persist across claudectl restarts:

1. Close claudectl (background Claude Code processes continue running)
2. Reopen claudectl
3. Active sessions are automatically restored and reconnected

## Technical Details

### Storage Architecture

**Project-Specific Storage:**
```
project-directory/
├── .claudectl/
│   ├── project.json          # Project configuration
│   ├── sessions.json         # Project-specific sessions
│   └── sessions.json.backup  # Backup of sessions
└── your-code/
```

**Global Storage (for backward compatibility):**
```
~/.config/claudectl/
├── data.json          # Global project list (deprecated for sessions)
└── data.json.backup   # Backup
```

**Storage Separation:**
- **Projects**: Managed globally for cross-project navigation
- **Sessions**: Stored per-project for isolation
- **Process Management**: Tracks active processes per session

### Process Spawning

```rust
let mut command = Command::new("claude");
command
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .kill_on_drop(true);

if let Some(ref path) = project_path {
    command.current_dir(path);
}

let child = command.spawn()?;
```

### Session Restoration

```rust
pub async fn restore_active_sessions(&self) -> Result<(), AppError> {
    let active_sessions: Vec<_> = self.data.sessions
        .iter()
        .filter(|s| matches!(s.status, SessionStatus::Active))
        .collect();

    for session in active_sessions {
        let project_path = session.project_id
            .as_ref()
            .and_then(|id| self.data.get_project(id))
            .map(|p| p.path.clone());

        if let Err(e) = self.process_manager.spawn_claude_session(session, project_path).await {
            eprintln!("Failed to restore session {}: {}", session.id, e);
        }
    }
    Ok(())
}
```

### Error Handling

The implementation includes comprehensive error handling:

- `ProcessError::ClaudeCodeNotFound`: When Claude Code executable is not available
- `ProcessError::SpawnError`: When process spawning fails
- `ProcessError::SessionNotFound`: When trying to operate on non-existent sessions
- `ProcessError::ProcessAlreadyRunning`: Prevents duplicate processes for the same session

## Future Enhancements

Potential improvements to consider:

1. **IPC Communication**: Enable communication with background Claude Code instances
2. **Session Recovery**: More robust session recovery mechanisms
3. **Resource Limits**: Implement limits on number of concurrent sessions
4. **Logging**: Enhanced logging for process management events
5. **Configuration**: Allow customization of Claude Code command and arguments

## Testing

The implementation includes unit tests for:

- Process manager creation and basic operations
- Error handling for non-existent sessions
- Session running checks
- Cleanup functionality

Run tests with:
```bash
cargo test
```

## Dependencies

No additional dependencies were required. The implementation uses existing dependencies:

- `tokio`: For async process management
- `crossterm`: For TUI event handling with timeouts
- Existing project dependencies for data structures and storage

## Conclusion

This implementation provides a robust foundation for background Claude Code session management with persistence. Sessions can be created, managed, and persist across application restarts, providing a seamless user experience for long-running development workflows.