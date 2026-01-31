use std::path::PathBuf;

use directories::ProjectDirs;

/// Application identifier for directory paths
const APP_QUALIFIER: &str = "com";
const APP_ORGANIZATION: &str = "reverse-ssh";
const APP_NAME: &str = "reverse-ssh-interface";

/// Get the project directories for this application
fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME)
}

/// Get the configuration directory path
/// - Linux: ~/.config/reverse-ssh-interface/
/// - macOS: ~/Library/Application Support/com.reverse-ssh.reverse-ssh-interface/
/// - Windows: C:\Users\<User>\AppData\Roaming\reverse-ssh\reverse-ssh-interface\config\
pub fn config_dir() -> PathBuf {
    project_dirs()
        .map(|dirs| dirs.config_dir().to_path_buf())
        .unwrap_or_else(|| {
            // Fallback to current directory
            PathBuf::from(".").join("config")
        })
}

/// Get the data directory path (for state, sessions, etc.)
/// - Linux: ~/.local/share/reverse-ssh-interface/
/// - macOS: ~/Library/Application Support/com.reverse-ssh.reverse-ssh-interface/
/// - Windows: C:\Users\<User>\AppData\Roaming\reverse-ssh\reverse-ssh-interface\data\
pub fn data_dir() -> PathBuf {
    project_dirs()
        .map(|dirs| dirs.data_dir().to_path_buf())
        .unwrap_or_else(|| {
            // Fallback to current directory
            PathBuf::from(".").join("data")
        })
}

/// Get the cache directory path
/// - Linux: ~/.cache/reverse-ssh-interface/
/// - macOS: ~/Library/Caches/com.reverse-ssh.reverse-ssh-interface/
/// - Windows: C:\Users\<User>\AppData\Local\reverse-ssh\reverse-ssh-interface\cache\
pub fn cache_dir() -> PathBuf {
    project_dirs()
        .map(|dirs| dirs.cache_dir().to_path_buf())
        .unwrap_or_else(|| {
            // Fallback to current directory
            PathBuf::from(".").join("cache")
        })
}

/// Get the logs directory path
pub fn logs_dir() -> PathBuf {
    data_dir().join("logs")
}

/// Get the main configuration file path
pub fn config_file() -> PathBuf {
    config_dir().join("config.toml")
}

/// Get the profiles directory path
pub fn profiles_dir() -> PathBuf {
    config_dir().join("profiles")
}

/// Get the state file path (runtime state persistence)
pub fn state_file() -> PathBuf {
    data_dir().join("state.json")
}

/// Get the known_hosts file path (app-managed)
pub fn known_hosts_file() -> PathBuf {
    config_dir().join("known_hosts")
}

/// Ensure all necessary directories exist
pub fn ensure_directories() -> std::io::Result<()> {
    std::fs::create_dir_all(config_dir())?;
    std::fs::create_dir_all(data_dir())?;
    std::fs::create_dir_all(cache_dir())?;
    std::fs::create_dir_all(logs_dir())?;
    std::fs::create_dir_all(profiles_dir())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths_are_absolute_or_relative() {
        // Just ensure these don't panic
        let _ = config_dir();
        let _ = data_dir();
        let _ = cache_dir();
        let _ = logs_dir();
        let _ = config_file();
        let _ = profiles_dir();
        let _ = state_file();
        let _ = known_hosts_file();
    }
}
