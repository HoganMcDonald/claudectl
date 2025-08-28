use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

use clap::{Args, CommandFactory, ValueEnum};
use clap_complete::{Shell, generate};
use tracing::{info, instrument};

use crate::commands::CommandResult;
use crate::utils::output::{error, standard, success};

#[derive(ValueEnum, Clone, Debug)]
pub enum CompletionShell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

impl From<CompletionShell> for Shell {
    fn from(shell: CompletionShell) -> Self {
        match shell {
            CompletionShell::Bash => Shell::Bash,
            CompletionShell::Zsh => Shell::Zsh,
            CompletionShell::Fish => Shell::Fish,
            CompletionShell::PowerShell => Shell::PowerShell,
            CompletionShell::Elvish => Shell::Elvish,
        }
    }
}

#[derive(Args, Debug)]
pub struct CompletionsCommand {
    /// The shell to generate completions for
    #[arg(value_enum)]
    pub shell: Option<CompletionShell>,

    /// Verify that completions are installed and working
    #[arg(long, help = "Verify completion installation")]
    pub verify: bool,
}

impl CompletionsCommand {
    #[instrument(name = "completions_command", fields(shell = ?self.shell, verify = self.verify))]
    pub fn execute(&self) -> CommandResult<()> {
        if self.verify {
            return self.verify_completions();
        }

        let shell = match &self.shell {
            Some(s) => s.clone(),
            None => {
                error("Shell must be specified when not using --verify");
                return Ok(());
            }
        };

        info!("Generating completions for shell: {:?}", shell);

        let mut app = crate::Cli::command();
        let shell_type: Shell = shell.into();

        generate(shell_type, &mut app, "claudectl", &mut io::stdout());

        Ok(())
    }

    fn verify_completions(&self) -> CommandResult<()> {
        info!("Verifying completion installation");

        let shell = self.detect_shell();
        standard(&format!("Detected shell: {shell}"));

        let completion_paths = self.get_completion_paths(&shell);
        let mut found_completion = false;
        let mut completion_file = String::new();

        for path in completion_paths {
            if Path::new(&path).exists() {
                success(&format!("✓ Found completion file: {path}"));
                completion_file = path;
                found_completion = true;
                break;
            }
        }

        if !found_completion {
            error("✗ No completion files found");
            standard("  Run: npm run setup -- repair");
            return Ok(());
        }

        // Check version info
        if let Ok(content) = fs::read_to_string(&completion_file) {
            if let Some(version) = self.extract_version(&content) {
                standard(&format!("Version: {version}"));
            }

            // Check for dynamic completion
            if content.contains("_claudectl_tasks") {
                success("✓ Dynamic task completion enabled");
            } else {
                error("✗ Dynamic task completion not found");
            }
        }

        // Test basic completion
        if self.test_completion() {
            success("✓ Basic completion test passed");
        } else {
            error("✗ Basic completion test failed");
        }

        success("Completion verification complete");
        Ok(())
    }

    fn detect_shell(&self) -> String {
        std::env::var("SHELL")
            .unwrap_or_else(|_| "bash".to_string())
            .split('/')
            .next_back()
            .unwrap_or("bash")
            .to_string()
    }

    fn get_completion_paths(&self, shell: &str) -> Vec<String> {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());

        match shell {
            "zsh" => vec![
                format!("{}/.zsh/completions/_claudectl", home),
                format!("{}/.zsh_completion.d/_claudectl", home),
                "/usr/local/share/zsh/site-functions/_claudectl".to_string(),
            ],
            "bash" => vec![
                format!("{}/.bash_completion.d/claudectl", home),
                "/etc/bash_completion.d/claudectl".to_string(),
            ],
            "fish" => vec![format!("{}/.config/fish/completions/claudectl.fish", home)],
            _ => vec![],
        }
    }

    fn extract_version(&self, content: &str) -> Option<String> {
        content
            .lines()
            .find(|line| line.starts_with("# Version:"))
            .and_then(|line| line.split_whitespace().nth(2))
            .map(|v| v.to_string())
    }

    fn test_completion(&self) -> bool {
        // Test if claudectl command is available
        Command::new("which")
            .arg("claudectl")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_shell_conversion() {
        assert!(matches!(Shell::from(CompletionShell::Bash), Shell::Bash));
        assert!(matches!(Shell::from(CompletionShell::Zsh), Shell::Zsh));
        assert!(matches!(Shell::from(CompletionShell::Fish), Shell::Fish));
    }

    #[test]
    fn test_completions_command_creation() {
        let cmd = CompletionsCommand {
            shell: Some(CompletionShell::Zsh),
            verify: false,
        };
        assert!(matches!(cmd.shell, Some(CompletionShell::Zsh)));
    }

    #[test]
    fn test_completions_verify_mode() {
        let cmd = CompletionsCommand {
            shell: None,
            verify: true,
        };
        assert!(cmd.verify);
    }
}
