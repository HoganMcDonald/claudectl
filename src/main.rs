use crate::utils::output::{blank, error};
use clap::Parser;
use tracing::{error as log_error, info};

mod commands;
mod utils;

#[derive(Parser)]
#[command(name = "claudectl")]
#[command(
    about = "A CLI tool for orchestrating Claude Code agents through the use of git worktrees."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<commands::Commands>,

    /// Show log file location and exit
    #[arg(long, help = "Show where log files are stored")]
    logs: bool,
}

fn main() {
    let cli = Cli::parse();

    // Handle --logs flag first, before initializing logging
    if cli.logs {
        use crate::utils::logging::{get_log_dir_path, get_log_file_path};
        use crate::utils::output::standard;

        let log_file = get_log_file_path();
        let log_dir = get_log_dir_path();

        standard(&format!("Log directory: {}", log_dir.display()));
        standard(&format!("Log file: {}", log_file.display()));

        if log_file.exists() {
            let metadata = std::fs::metadata(&log_file).ok();
            if let Some(meta) = metadata {
                let size = meta.len();
                let size_kb = size / 1024;
                standard(&format!("Log file size: {size_kb} KB"));
            }
        } else {
            standard("Log file does not exist yet (will be created on first run)");
        }

        return;
    }

    // Initialize logging - write to file to keep CLI output clean
    init_logging();

    info!("Starting claudectl");

    let command = match cli.command {
        Some(cmd) => cmd,
        None => {
            // This shouldn't happen with clap's validation, but just in case
            error("No command provided");
            std::process::exit(1);
        }
    };

    if let Err(err) = commands::handle_command(command) {
        log_error!("Command failed: {}", err);
        blank();
        error(&err.message());
        std::process::exit(1);
    }

    info!("Command completed successfully");
}

fn init_logging() {
    use crate::utils::logging::{get_log_dir_path, get_log_file_path, rotate_log_if_needed};
    use std::fs;
    use tracing_subscriber::fmt::writer::MakeWriterExt;

    // Rotate log file if needed
    let _ = rotate_log_if_needed();

    // Create log directory
    let log_dir = get_log_dir_path();
    if fs::create_dir_all(&log_dir).is_err() {
        // If we can't create the log directory, disable logging
        return tracing_subscriber::fmt()
            .with_writer(std::io::sink)
            .with_env_filter(tracing_subscriber::EnvFilter::new("off"))
            .init();
    }

    let log_file_path = get_log_file_path();

    // Create the log file writer
    let log_file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
    {
        Ok(file) => file,
        Err(_) => {
            // If we can't create the log file, disable logging
            return tracing_subscriber::fmt()
                .with_writer(std::io::sink)
                .with_env_filter(tracing_subscriber::EnvFilter::new("off"))
                .init();
        }
    };

    // Configure tracing to write to file only
    tracing_subscriber::fmt()
        .with_writer(log_file.with_max_level(tracing::Level::DEBUG))
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("claudectl=info")),
        )
        .with_ansi(false) // No ANSI colors in log file
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .init();
}
