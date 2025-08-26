# claudectl AI Development Context

## Attribution Rules
- Never add Claude as a co-author on commits. Never mention Claude as an author of code or commits.
- Do not reference AI assistance in code comments or documentation.

## Codebase Understanding

### Project Purpose
`claudectl` is a CLI tool for managing git worktrees and integrating with Claude Code for multi-task development workflows. It creates isolated workspaces for concurrent development tasks.

### Core Architecture Patterns
- **Error Handling**: Uses hierarchical error types with `thiserror`. Always propagate errors with `?` operator
- **CLI Structure**: Uses `clap` with derive macros. Commands are in `src/commands/` modules
- **Output**: Consistent styling via `src/utils/output.rs` with Catppuccin theme colors
- **Configuration**: JSON-based config stored via `directories` crate, managed in `src/utils/config.rs`
- **Git Integration**: All git operations go through `src/utils/git.rs` with proper error handling

### Key Implementation Details
- **Status Display**: Use `format_status()` function for consistent status formatting with colored icons
- **Table Output**: Call `table(data, show_header)` - use `false` for minimal list displays
- **Error Messages**: Always use structured error types, never panic or unwrap in commands
- **File Operations**: Use `src/utils/fs.rs` functions for consistent file handling
- **Git Commands**: Execute via `std::process::Command` with proper stderr capture and error conversion

## Code Implementation Guidelines

### When Adding New Commands
1. **Command Module**: Create in `src/commands/{command}.rs` with:
   ```rust
   use clap::Args;
   use crate::commands::CommandResult;
   
   #[derive(Args)]
   pub struct {Command}Command {
       // Add fields for arguments
   }
   
   impl {Command}Command {
       pub fn execute(&self) -> CommandResult<()> {
           // Implementation
       }
   }
   ```

2. **Error Handling**: Always return `CommandResult<T>` and use `?` operator:
   ```rust
   let config = read_local_config_file()?;  // Don't unwrap()
   let worktrees = worktree_list().inspect_err(|e| {
       error(&format!("Failed to get tasks: {e}"));
   })?;
   ```

3. **Register Command**: Add to `src/commands/mod.rs` in the `Commands` enum

### When Working with Output
- **Colored Output**: Import `THEME` from `src/utils/theme.rs`
- **Icons**: Import `ICONS` from `src/utils/icons.rs`
- **Tables**: Use `table(&data, show_header)` from `src/utils/output.rs`
- **Error Display**: Use `error()`, `success()`, `standard()` functions

### When Working with Git Operations
- **All Git Commands**: Must go through `src/utils/git.rs` functions
- **Error Conversion**: Git operations return `GitResult<T>` which auto-converts to `CommandError`
- **Validation**: Always check `is_git_repository()` before git operations

### When Working with Configuration
- **Reading Config**: Use `read_local_config_file()` from `src/utils/fs.rs`
- **Config Types**: Use `Config::from_str()` for parsing JSON
- **Error Handling**: Config errors auto-convert to `CommandError`

### Testing Requirements for AI
When implementing new features, ALWAYS add:

1. **Unit Tests**: In same file with `#[cfg(test)]`:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_function_name() {
           // Test implementation
       }
   }
   ```

2. **Integration Tests**: In `tests/integration/{command}.rs`:
   ```rust
   use assert_cmd::Command;
   use tempfile::TempDir;
   
   #[test]
   fn test_command_behavior() {
       let temp_dir = TempDir::new().unwrap();
       // Setup git repo: fs::create_dir(temp_dir.path().join(".git")).unwrap();
       // Setup config: Create .claudectl/config.json
       let mut cmd = Command::cargo_bin("claudectl").unwrap();
       let output = cmd.arg("command").current_dir(&temp_dir).output().unwrap();
       // Assertions
   }
   ```

3. **Test Error Cases**: Always test:
   - No git repository
   - No configuration file  
   - Invalid inputs
   - External command failures

### Quality Standards
- **Before Committing**: Run `cargo fmt && cargo clippy && cargo test`
- **Type Annotations**: Add explicit types when compiler inference fails
- **Documentation**: Add doc comments to public functions
- **No Panics**: Never use `unwrap()`, `expect()`, or `panic!()` in command implementations

### Common Patterns to Follow
- **Status Formatting**: Use `format_status(status: Status) -> String`
- **File Operations**: Use functions from `src/utils/fs.rs`
- **Logging**: Use `tracing::info!()` for debug information
- **Color Theming**: Use `THEME.success`, `THEME.error`, etc. from theme module
- **Icon Usage**: Use `ICONS.status.circle`, etc. from icons module