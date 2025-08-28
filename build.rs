use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint};
use clap_complete::{Shell, generate_to};
use std::io::Error;
use std::path::PathBuf;

// Minimal CLI definition for build-time completion generation
#[derive(Parser)]
#[command(name = "claudectl")]
#[command(
    about = "A CLI tool for orchestrating Claude Code agents through the use of git worktrees."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long, global = true, help = "Enable debug logging output")]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    Init(InitCommand),
    Task(TaskCommand),
    List(ListCommand),
    Rm(RmCommand),

    #[command(next_help_heading = "Utility Commands")]
    Completions(CompletionsCommand),
    Repair(RepairCommand),
}

#[derive(Args)]
struct InitCommand {
    project_name: Option<String>,
}

#[derive(Args)]
struct TaskCommand {
    task_name: String,
}

#[derive(Args)]
struct ListCommand {}

#[derive(Args)]
struct RmCommand {
    #[arg(value_hint = ValueHint::Other)]
    task_name: String,
}

#[derive(Args)]
struct CompletionsCommand {
    #[arg(value_enum)]
    shell: Option<CompletionShell>,
    #[arg(long)]
    verify: bool,
}

#[derive(Args)]
struct RepairCommand {
    #[arg(long)]
    force: bool,
}

#[derive(ValueEnum, Clone)]
enum CompletionShell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

fn main() -> Result<(), Error> {
    let mut cmd = Cli::command();

    // Create completions directory in the project root
    let completions_dir = PathBuf::from("completions");
    std::fs::create_dir_all(&completions_dir)?;

    // Generate completion files for each shell
    let shells = [
        Shell::Bash,
        Shell::Zsh,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Elvish,
    ];

    for &shell in shells.iter() {
        generate_to(shell, &mut cmd, "claudectl", &completions_dir)?;
    }

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=Cargo.toml");

    Ok(())
}
