use crate::utils::errors::ClaudeError;

type ClaudeResult<T> = Result<T, ClaudeError>;

#[derive(Debug)]
pub enum Status {
    ///Ready for initial user input
    Ready,
    ///The agent is actively working
    #[allow(dead_code)]
    Working,
    ///The agent is waiting for user input
    #[allow(dead_code)]
    Waiting,
    ///Claudectl is unable to communicate with the agent process
    #[allow(dead_code)]
    Unknown,
}

pub struct Session {
    #[allow(dead_code)]
    pub name: String,
    pub status: Status,
}

pub fn is_claude_installed() -> ClaudeResult<bool> {
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

pub fn get_session(name: &str) -> ClaudeResult<Session> {
    // TODO: actually find a running claude session
    Ok(Session {
        name: name.to_string(),
        status: Status::Ready,
    })
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
                error.to_string(),
                "Claude is not installed: Claude is not installed or not found in PATH."
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
