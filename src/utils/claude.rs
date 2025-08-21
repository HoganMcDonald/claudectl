use crate::utils::errors::ClaudeError;

pub fn is_claude_installed() -> Result<bool, ClaudeError> {
    let output = std::process::Command::new("which")
        .arg("claude")
        .output()
        .map_err(|e| ClaudeError::new(&format!("Failed to execute 'which claude': {}", e)))?;

    if output.status.success() {
        Ok(true)
    } else {
        Err(ClaudeError::new(
            "Claude is not installed or not found in PATH.",
        ))
    }
}
