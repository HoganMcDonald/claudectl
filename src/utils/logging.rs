use directories::ProjectDirs;
use std::path::PathBuf;

/// Get the path to the claudectl log file
pub fn get_log_file_path() -> PathBuf {
    if let Some(project_dirs) = ProjectDirs::from("com", "claudectl", "claudectl") {
        let log_dir = project_dirs.config_dir().join("logs");
        log_dir.join("claudectl.log")
    } else {
        PathBuf::from("./claudectl.log")
    }
}

/// Get the path to the claudectl log directory
pub fn get_log_dir_path() -> PathBuf {
    if let Some(project_dirs) = ProjectDirs::from("com", "claudectl", "claudectl") {
        project_dirs.config_dir().join("logs")
    } else {
        PathBuf::from("./")
    }
}

/// Rotate log file if it gets too large (> 10MB)
pub fn rotate_log_if_needed() -> std::io::Result<()> {
    let log_path = get_log_file_path();

    if !log_path.exists() {
        return Ok(());
    }

    let metadata = std::fs::metadata(&log_path)?;
    let file_size = metadata.len();

    // If log file is larger than 10MB, rotate it
    if file_size > 10 * 1024 * 1024 {
        let backup_path = log_path.with_extension("log.old");

        // Remove old backup if it exists
        if backup_path.exists() {
            std::fs::remove_file(&backup_path)?;
        }

        // Move current log to backup
        std::fs::rename(&log_path, &backup_path)?;
    }

    Ok(())
}

/// Print log location information to the user (for debugging)
#[allow(dead_code)]
pub fn print_log_info() {
    use crate::utils::output::standard;

    let log_path = get_log_file_path();
    standard(&format!("Logs are written to: {}", log_path.display()));
}
