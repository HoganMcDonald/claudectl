# Session Navigation and Output Display Implementation

This document describes the implementation of session navigation and Claude Code output display in claudectl.

## Overview

The implementation adds comprehensive session navigation and real-time output display capabilities to claudectl, allowing users to navigate through sessions and view their Claude Code interaction output in the main content area.

## Key Features Implemented

### 1. Session Navigation
- **Vim-style navigation**: Use `j`/`k` or `↑`/`↓` to navigate through sessions
- **Focus system**: Use `Tab` to switch focus between different UI areas
- **Session selection**: Press `Enter` to select and view a session's output

### 2. Claude Code Output Capture
- **Real-time capture**: Captures both stdout and stderr from Claude Code processes
- **Async streaming**: Uses tokio to stream output without blocking the UI
- **Buffer management**: Maintains output buffers for each active session

### 3. UI Enhancements
- **Focus highlighting**: Focused areas are clearly highlighted with different colors
- **Session output display**: Selected session output appears in the main content area
- **Interactive instructions**: Clear guidance for users on how to navigate

## Technical Implementation

### Focus Management

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum FocusArea {
    Projects,
    Sessions,
}
```

The app maintains a focus state that determines which area responds to navigation keys.

### Navigation Controls

- **j/k or ↑/↓**: Navigate within the focused area
- **Tab**: Switch focus between Projects and Sessions
- **Enter**: Select the current item (shows session output)

### Output Capture Architecture

```rust
pub struct ProcessHandle {
    pub child: Child,
    pub session_id: String,
    pub project_path: Option<PathBuf>,
    pub output_buffer: Arc<std::sync::Mutex<String>>,
}
```

Each Claude Code process has an associated output buffer that captures:
- **[OUT]**: Standard output lines
- **[ERR]**: Standard error lines

### Async Output Reading

```rust
// Spawn tasks to read output
tokio::spawn(async move {
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        if let Ok(mut buffer) = output_buffer_stdout.lock() {
            buffer.push_str(&format!("[OUT] {}\n", line));
        }
    }
});
```

Each process spawns separate tasks for reading stdout and stderr, ensuring non-blocking capture.

## User Experience

### Navigation Flow

1. **Start claudectl**: Focus begins on Sessions area
2. **Navigate sessions**: Use `j`/`k` to move through available sessions
3. **Select session**: Press `Enter` to view output in main content area
4. **Switch focus**: Use `Tab` to switch between Projects and Sessions

### Visual Feedback

- **Focused area**: Title shows `[FOCUSED]` with brighter colors
- **Selected item**: Highlighted with different background color
- **Output display**: Shows session info and real-time Claude Code output

### Key Bindings

| Key | Action |
|-----|--------|
| `j` or `↓` | Navigate down |
| `k` or `↑` | Navigate up |
| `Tab` | Switch focus area |
| `Enter` | Select session/show output |
| `n` | New session |
| `s` | Stop session |
| `q` | Quit |

## Code Structure

### App State Updates

- Added `focus_area: FocusArea` to track current focus
- Added `selected_session_output: Option<String>` for display
- Updated navigation methods to be focus-aware

### Process Manager Enhancements

- Enhanced `ProcessHandle` with output buffer
- Added `get_session_output()` method for retrieving captured output
- Implemented async output capture tasks

### UI Components

- Updated `SessionsPanel` to show focus state
- Created `render_main_content_area()` for output display
- Added visual indicators for focused areas

## File Changes

### Core Files Modified

1. **`src/app.rs`**:
   - Added focus management
   - Updated navigation logic
   - Added session selection handling

2. **`src/process.rs`**:
   - Enhanced output capture
   - Added async output reading
   - Implemented output retrieval methods

3. **`src/tui.rs`**:
   - Updated main content rendering
   - Added focus-aware UI components

4. **`src/components/sessions_panel.rs`**:
   - Added focus highlighting
   - Enhanced visual feedback

## Usage Example

```bash
# Start claudectl
claudectl

# Navigate sessions with j/k
# Press Enter to view session output
# Use Tab to switch between areas
# Press n to create new session
# Press s to stop selected session
```

## Future Enhancements

Potential improvements for the output capture system:

1. **Real-time UI updates**: Update the display as new output arrives
2. **Output filtering**: Filter by output type (stdout/stderr)
3. **Output scrolling**: Implement scrollable output view
4. **Search functionality**: Search within session output
5. **Export output**: Save session output to files
6. **Output streaming**: Live streaming of output as it happens

## Benefits

This implementation provides:

- **Intuitive navigation**: Familiar vim-style key bindings
- **Real-time feedback**: See Claude Code output as it happens
- **Session management**: Easy switching between multiple sessions
- **Visual clarity**: Clear focus indicators and session highlighting
- **Non-blocking operation**: Async output capture doesn't freeze the UI

The implementation creates a smooth, responsive user experience for managing multiple Claude Code sessions while providing real-time visibility into their output and status.