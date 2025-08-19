use clap::Args;
use crate::utils::{validate_git_repository, ValidationResult};
use crate::utils::ICONS;
use owo_colors::OwoColorize;

#[derive(Args)]
pub struct InitCommand {
    #[arg(short, long, default_value = "")]
    pub name: String,
}

impl InitCommand {
    pub fn execute(&self) -> ValidationResult<()> {
        validate_git_repository(None)?;

        println!("{} Git repository validation passed!", ICONS.status.success.green().bold());
        println!("Initializing project: {}", self.name.blue().bold());

        Ok(())
    }
}
