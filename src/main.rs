mod app;
mod cli;
mod components;
mod data;
mod process;
mod project_init;
mod storage;
mod tui;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse_args();

    match &cli.command {
        Some(Commands::Tui) | None => {
            if let Err(err) = tui::run().await {
                eprintln!("Error: {err}");
                std::process::exit(1);
            }
        }
    }
}
