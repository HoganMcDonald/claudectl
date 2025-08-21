use clap::Parser;
use crate::utils::output::{blank, error};

mod commands;
mod utils;

#[derive(Parser)]
#[command(name = "claudectl")]
#[command(about = "A CLI tool for orchestrating Claude Code agents through the use of git worktrees.")]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = commands::handle_command(cli.command) {
        blank();
        error(&err.message);
        std::process::exit(1);
    }
}
