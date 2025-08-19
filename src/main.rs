use clap::Parser;

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
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
