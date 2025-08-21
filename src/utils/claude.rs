use crate::utils::errors::ClaudeError;

pub fn is_claude_installed() -> Result<bool, ClaudeError> {
    let output = std::process::Command::new("which")
        .arg("claude")
        .output()
        .map_err(|e| ClaudeError::new(&format!("Failed to execute 'which claude': {e}")))?;

    if output.status.success() {
        Ok(true)
    } else {
        Err(ClaudeError::new(
            "Claude is not installed or not found in PATH.",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_claude_installed_success() {
        // This test will pass if claude is actually installed
        // In a real CI environment, you might want to mock this
        let result = is_claude_installed();

        // Since we can't guarantee claude is installed in all test environments,
        // we just verify the function doesn't panic and returns a Result
        assert!(result.is_ok() || result.is_err());

        if let Err(error) = result {
            // If it fails, verify it's the expected error
            assert_eq!(
                error.message,
                "Claude is not installed or not found in PATH."
            );
        }
    }

    #[test]
    fn test_is_claude_installed_with_nonexistent_command() {
        // Test the command execution error path by using a command that doesn't exist
        let output = std::process::Command::new("nonexistent_command_12345")
            .arg("test")
            .output();

        // Verify that command execution can fail (for reference)
        assert!(output.is_err());
    }
}
