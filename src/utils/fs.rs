use std::path::PathBuf;

use crate::utils::errors::FileSystemError;

type FileSystemResult<T> = Result<T, FileSystemError>;

fn current_dir() -> FileSystemResult<PathBuf> {
    std::env::current_dir()
        .map_err(|_| FileSystemError::new("Failed to get current directory", "./"))
}

fn home_dir() -> FileSystemResult<PathBuf> {
    std::env::var("HOME")
        .map(PathBuf::from)
        .map_err(|_| FileSystemError::new("Failed to get home directory", "~/"))
}

pub fn create_global_configuration_dir(project_name: &str) -> FileSystemResult<String> {
    let config_dir = home_dir()?.join(".config");
    let claudectl_config = config_dir.join("claudectl");
    let global_projects_dir = claudectl_config.join("projects");

    std::fs::create_dir_all(&global_projects_dir).map_err(|e| {
        FileSystemError::new(
            &format!("Failed to create configuration directories ({e})"),
            "~/.config/claudectl/projects/",
        )
    })?;

    let mut global_project_dir = global_projects_dir.join(project_name);
    if global_project_dir.exists() {
        // If a directory with this project name already exists globally, find an available name.
        let mut n: u32 = 1;
        while global_projects_dir
            .join(format!("{project_name}{n}"))
            .exists()
        {
            n += 1;
        }
        global_project_dir = global_projects_dir.join(format!("{project_name}{n}"));
    }
    std::fs::create_dir(&global_project_dir).map_err(|e| {
        FileSystemError::new(
            &format!("Failed to create global project directory ({e})"),
            &global_project_dir.to_string_lossy(),
        )
    })?;

    Ok(global_project_dir.to_string_lossy().to_string())
}

pub fn create_local_configuration_dir() -> FileSystemResult<()> {
    let current_dir = current_dir()?;
    let local_config_dir = current_dir.join(".claudectl");
    std::fs::create_dir_all(&local_config_dir).map_err(|e| {
        FileSystemError::new(
            &format!("Failed to create local configuration directory ({e})"),
            "./.claudectl/",
        )
    })?;
    Ok(())
}

#[allow(dead_code)]
pub fn read_local_config_file() -> FileSystemResult<String> {
    let local_config_dir = current_dir()?.join(".claudectl");
    let config_file_path = local_config_dir.join("config.json");

    // Check if the configuration file exists
    if !config_file_path.exists() {
        return Err(FileSystemError::new(
            "Local configuration file does not exist. Please run `claudectl init` to create it.",
            "./.claudectl/config.json",
        ));
    }

    // Read the configuration file
    std::fs::read_to_string(&config_file_path).map_err(|e| {
        FileSystemError::new(
            &format!("Failed to read local configuration file ({e})"),
            &config_file_path.to_string_lossy(),
        )
    })
}

pub fn write_local_config_file(config: String) -> FileSystemResult<()> {
    let local_config_dir = current_dir()?.join(".claudectl");
    let config_file_path = local_config_dir.join("config.json");

    // Write the provided config to the file
    std::fs::write(&config_file_path, config).map_err(|e| {
        FileSystemError::new(
            &format!("Failed to write config to configuration file ({e})"),
            "./.claudectl/config.json",
        )
    })?;

    Ok(())
}
