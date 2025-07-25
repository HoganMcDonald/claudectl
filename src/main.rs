mod cli;
mod components;
mod data;
mod storage;
mod tui;

use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse_args();

    match &cli.command {
        Some(Commands::Tui) | None => {
            if let Err(err) = tui::run() {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    }
}
