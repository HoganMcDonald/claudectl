use clap::Args;
use tabled::Tabled;
use tracing::info;

use crate::{
    commands::CommandResult,
    utils::{
        config::Config,
        fs::read_local_config_file,
        git::worktree_list,
        output::{error, table},
    },
};

#[derive(Tabled)]
struct TaskRow {
    name: String,
    status: String,
    commit: String,
    worktree: String,
}

#[derive(Args)]
pub struct ListCommand {}

impl ListCommand {
    pub fn execute(&self) -> CommandResult<()> {
        info!("Executing list command.");
        let raw_config = read_local_config_file()?;
        let config = Config::from_str(&raw_config)?;
        info!("Loaded configuration for project: {}", config.project_name);

        // 1. get all worktrees
        let worktrees = worktree_list().inspect_err(|e| {
            error(&format!("Failed to get active tasks: {e}"));
        })?;

        // 2. get status of each task (worktree)
        let data: Vec<TaskRow> = worktrees
            .into_iter()
            .map(|wt| TaskRow {
                name: wt.branch.unwrap_or_else(|| "N/A".to_string()),
                status: "active".to_string(), // Placeholder for status
                commit: wt.commit,
                worktree: wt.path,
            })
            .collect();

        table(&data);

        Ok(())
    }
}
