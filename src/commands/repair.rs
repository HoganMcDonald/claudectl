use std::fs;
use std::path::Path;
use std::process::Command;

use clap::Args;
use tracing::{info, instrument};

use crate::commands::CommandResult;
use crate::utils::output::{error, standard, success};

#[derive(Args, Debug)]
pub struct RepairCommand {
    /// Force repair even if completions appear to be working
    #[arg(long, help = "Force repair even if completions appear working")]
    pub force: bool,
}

impl RepairCommand {
    #[instrument(name = "repair_command", fields(force = self.force))]
    pub fn execute(&self) -> CommandResult<()> {
        info!("Starting completion repair process");

        standard("🔧 Repairing claudectl shell completions...");

        // Step 1: Check current state
        if !self.force && self.check_completions_working() {
            success("✓ Completions appear to be working correctly");
            standard("Use --force to repair anyway");
            return Ok(());
        }

        // Step 2: Run the npm install script
        standard("Running completion installer...");
        if let Err(e) = self.run_install_script() {
            error(&format!("Failed to run installer: {e}"));
            self.print_manual_instructions();
            return Ok(());
        }

        // Step 3: Verify repair was successful
        standard("Verifying repair...");
        if self.check_completions_working() {
            success("🎉 Repair completed successfully!");
            standard("Restart your terminal or run 'exec $SHELL' to activate completions");
        } else {
            error("⚠️  Repair completed but verification failed");
            self.print_troubleshooting();
        }

        Ok(())
    }

    fn check_completions_working(&self) -> bool {
        let shell = self.detect_shell();
        let completion_paths = self.get_completion_paths(&shell);

        // Check if any completion file exists
        for path in &completion_paths {
            if Path::new(path).exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    // Check for version info (indicates npm-installed completion)
                    if content.contains("# Version:") {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn run_install_script(&self) -> Result<(), String> {
        // Try to find the install script relative to the current binary
        let install_script = self.find_install_script()?;

        let output = Command::new("node")
            .arg(install_script)
            .output()
            .map_err(|e| format!("Failed to execute installer: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Installer failed: {stderr}"));
        }

        // Print installer output
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            standard(&format!("  {line}"));
        }

        Ok(())
    }

    fn find_install_script(&self) -> Result<std::path::PathBuf, String> {
        // Get the path of the current executable
        let exe_path = std::env::current_exe()
            .map_err(|_| "Could not determine executable path".to_string())?;

        // For npm installations, look for npm/install.js relative to the binary
        // Try several possible locations:
        let possible_paths = [
            // npm global install: binary is in node_modules/.bin/, script is in node_modules/claudectl/npm/
            exe_path
                .parent()
                .and_then(|p| p.parent())
                .map(|p| p.join("claudectl/npm/install.js")),
            // Local development: script is in project root
            exe_path
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .map(|p| p.join("npm/install.js")),
            // Current directory (fallback)
            Some(std::path::PathBuf::from("npm/install.js")),
        ];

        for path in possible_paths.iter().flatten() {
            if path.exists() {
                return Ok(path.clone());
            }
        }

        Err("Install script not found. Please reinstall claudectl via npm.".to_string())
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

    fn print_manual_instructions(&self) {
        standard("\n📋 Manual repair instructions:");
        standard("1. Reinstall claudectl:");
        standard("   npm uninstall -g claudectl && npm install -g claudectl");
        standard("\n2. Or manually copy completion files:");

        let shell = self.detect_shell();
        match shell.as_str() {
            "zsh" => {
                standard("   cp completions/_claudectl ~/.zsh_completion.d/");
                standard("   echo 'fpath+=~/.zsh_completion.d' >> ~/.zshrc");
                standard("   echo 'autoload -U compinit && compinit' >> ~/.zshrc");
            }
            "bash" => {
                standard("   cp completions/claudectl.bash ~/.bash_completion.d/claudectl");
                standard("   echo 'source ~/.bash_completion.d/claudectl' >> ~/.bashrc");
            }
            "fish" => {
                standard("   mkdir -p ~/.config/fish/completions");
                standard("   cp completions/claudectl.fish ~/.config/fish/completions/");
            }
            _ => {
                standard("   Check claudectl documentation for your shell");
            }
        }
    }

    fn print_troubleshooting(&self) {
        standard("\n🛠️  Troubleshooting steps:");
        standard("1. Check shell configuration:");
        standard("   - Ensure your shell RC file sources completions");
        standard("   - Verify completion directories are in PATH");

        standard("\n2. Check file permissions:");
        standard("   - Completion files should be readable");
        standard("   - Completion directories should be writable");

        standard("\n3. Test manually:");
        standard("   claudectl completions --verify");

        standard("\n4. Get help:");
        standard("   - Check: https://github.com/yourusername/claudectl/issues");
        standard("   - Run: claudectl --help");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repair_command_creation() {
        let cmd = RepairCommand { force: false };
        assert!(!cmd.force);
    }

    #[test]
    fn test_shell_detection() {
        let cmd = RepairCommand { force: false };
        let shell = cmd.detect_shell();
        // Should return some shell name
        assert!(!shell.is_empty());
    }
}
