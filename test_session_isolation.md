# Testing Session Isolation

This document outlines how to test that sessions are properly isolated between projects.

## Test Scenario

1. **Create Project A**:
   ```bash
   mkdir project-a
   cd project-a
   # Run claudectl and initialize project
   # Create some sessions
   ```

2. **Create Project B**:
   ```bash
   cd ..
   mkdir project-b
   cd project-b
   # Run claudectl and initialize project
   # Create different sessions
   ```

3. **Verify Isolation**:
   - Sessions from Project A should not appear in Project B
   - Sessions from Project B should not appear in Project A
   - Each project should have its own `.claudectl/sessions.json` file

## Expected File Structure

**Project A:**
```
project-a/
├── .claudectl/
│   ├── project.json      # Contains project A name
│   └── sessions.json     # Contains only project A sessions
└── ... (project files)
```

**Project B:**
```
project-b/
├── .claudectl/
│   ├── project.json      # Contains project B name
│   └── sessions.json     # Contains only project B sessions
└── ... (project files)
```

## Implementation Details

The session isolation is achieved through:

1. **Storage Location**: Sessions are stored in project-specific `.claudectl/sessions.json` files
2. **Loading Logic**: The `JsonStorage::new()` method automatically detects the current directory's `.claudectl/` folder
3. **Session Data Structure**: `SessionData` is separate from global `AppData`
4. **Process Management**: Each session spawns Claude Code in the correct project directory

## Key Code Changes

- `JsonStorage::new()` now checks for `.claudectl/` directory in current working directory
- New `SessionData` and `SessionStorage` trait for project-specific session management
- App structure split to handle global projects and project-specific sessions separately
- Session file name changed from `data.json` to `sessions.json` for clarity

This ensures complete isolation of sessions between different projects while maintaining backward compatibility for global project management.