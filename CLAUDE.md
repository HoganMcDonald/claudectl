# claudectl Development Guidelines

## Attribution
- Never add Claude as a co-author on commits. Never mention Claude as an author of code or commits.

## Testing Standards

### Testing Strategy
We use a comprehensive testing approach with both unit and integration tests:

- **Unit Tests**: Located within modules using `#[cfg(test)]` for testing individual functions and utilities
- **Integration Tests**: Located in `tests/` directory using `assert_cmd` for end-to-end CLI testing
- **Test Structure**: Follow the pattern `tests/integration/{command}.rs` for command-specific integration tests

### Test Requirements
All new features must include:

1. **Unit Tests** for utility functions and core logic
   - Test both success and error cases  
   - Test edge cases and boundary conditions
   - Use descriptive test names like `test_format_status_ready()`

2. **Integration Tests** for commands
   - Test command execution with `assert_cmd::Command::cargo_bin()`
   - Test error conditions (no git repo, no config, etc.)
   - Test output format and content
   - Use `tempfile::TempDir` for filesystem isolation

3. **Mock External Dependencies** where possible
   - Prefer testing logic over external command execution
   - Use dependency injection for testability
   - Mock file system operations when needed

### Test Organization
```rust
// Unit tests within modules
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_specific_function() {
        // Test implementation
    }
}

// Integration tests in tests/ directory
use assert_cmd::Command;
use tempfile::TempDir;

#[test]
fn test_command_behavior() {
    let temp_dir = TempDir::new().unwrap();
    // Set up test environment
    // Execute command
    // Assert results
}
```

### Error Testing
Always test error conditions:
- Commands run outside git repositories
- Missing configuration files
- Invalid input parameters
- External command failures

### Output Testing
For CLI output:
- Test both stdout and stderr
- Verify error messages are user-friendly
- Test colored output contains expected ANSI codes
- Verify table formatting and alignment

### Running Tests
- Unit tests: `cargo test`
- Integration tests: `cargo test --test main`
- Specific test: `cargo test test_name`
- With output: `cargo test -- --nocapture`

### Test Guidelines
- Tests should be fast and deterministic
- Use meaningful assertions with clear error messages
- Clean up resources (tempfile handles this automatically)
- Test names should describe what is being tested
- Group related tests in modules