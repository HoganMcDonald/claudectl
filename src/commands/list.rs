use clap::Args;
use owo_colors::OwoColorize;
use tabled::Tabled;
use tracing::info;

use crate::{
    commands::CommandResult,
    utils::{
        claude::{Status, get_session},
        config::Config,
        fs::read_local_config_file,
        git::worktree_list,
        icons::ICONS,
        output::{error, table},
        theme::THEME,
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
            .map(|wt| -> CommandResult<TaskRow> {
                let name = wt.branch.unwrap_or_else(|| "N/A".to_string());
                let session = get_session(name.as_str())?;
                Ok(TaskRow {
                    name: name.clone(),
                    status: format_status(session.status),
                    commit: wt.commit,
                    worktree: wt.path.as_str().color(THEME.muted).to_string(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        table(&data, false);

        Ok(())
    }
}

fn format_status(status: Status) -> String {
    let color = match status {
        Status::Ready => THEME.success,
        Status::Working => THEME.warning,
        Status::Waiting => THEME.info,
        Status::Unknown => THEME.error,
    };

    format!(
        "{} {}",
        ICONS.status.circle.color(color),
        format!("{status:?}").color(THEME.muted)
    )
}
