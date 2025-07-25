use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claudectl")]
#[command(about = "A CLI/TUI tool for orchestrating multi-agent workflows")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Launch the TUI interface (default behavior)
    Tui,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
