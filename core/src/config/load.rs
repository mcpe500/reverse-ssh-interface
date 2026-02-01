use std::path::Path;

use crate::error::{CoreError, Result};
use crate::types::Profile;

use super::model::AppConfig;
use super::paths;

/// Load the application configuration from the default location
pub fn load_config() -> Result<AppConfig> {
    let config_path = paths::config_file();
    load_config_from(&config_path)
}

/// Load the application configuration from a specific path
pub fn load_config_from(path: &Path) -> Result<AppConfig> {
    if !path.exists() {
        // Return default config if file doesn't exist
        return Ok(AppConfig::default());
    }

    let content = std::fs::read_to_string(path).map_err(|e| {
        CoreError::ConfigParse(format!("Failed to read config file: {}", e))
    })?;

    toml::from_str(&content).map_err(|e| {
        CoreError::ConfigParse(format!("Failed to parse config file: {}", e))
    })
}

/// Save the application configuration to the default location
pub fn save_config(config: &AppConfig) -> Result<()> {
    let config_path = paths::config_file();
    save_config_to(config, &config_path)
}

/// Save the application configuration to a specific path
pub fn save_config_to(config: &AppConfig, path: &Path) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            CoreError::ConfigWrite(format!("Failed to create config directory: {}", e))
        })?;
    }

    let content = toml::to_string_pretty(config).map_err(|e| {
        CoreError::ConfigWrite(format!("Failed to serialize config: {}", e))
    })?;

    std::fs::write(path, content).map_err(|e| {
        CoreError::ConfigWrite(format!("Failed to write config file: {}", e))
    })?;

    Ok(())
}

/// Load all profiles from the profiles directory
pub fn load_profiles() -> Result<Vec<Profile>> {
    let profiles_dir = paths::profiles_dir();
    load_profiles_from(&profiles_dir)
}

/// Load all profiles from a specific directory
pub fn load_profiles_from(dir: &Path) -> Result<Vec<Profile>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut profiles = Vec::new();

    let entries = std::fs::read_dir(dir).map_err(|e| {
        CoreError::ConfigParse(format!("Failed to read profiles directory: {}", e))
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            CoreError::ConfigParse(format!("Failed to read directory entry: {}", e))
        })?;

        let path = entry.path();
        
        // Only process .toml files
        if path.extension().map_or(false, |ext| ext == "toml") {
            match load_profile_from(&path) {
                Ok(profile) => profiles.push(profile),
                Err(e) => {
                    tracing::warn!("Failed to load profile from {:?}: {}", path, e);
                }
            }
        }
    }

    Ok(profiles)
}

/// Load a single profile from a file
pub fn load_profile_from(path: &Path) -> Result<Profile> {
    let content = std::fs::read_to_string(path).map_err(|e| {
        CoreError::ConfigParse(format!("Failed to read profile file: {}", e))
    })?;

    toml::from_str(&content).map_err(|e| {
        CoreError::ConfigParse(format!("Failed to parse profile file: {}", e))
    })
}

/// Save a profile to the profiles directory
pub fn save_profile(profile: &Profile) -> Result<()> {
    let profiles_dir = paths::profiles_dir();
    save_profile_to(profile, &profiles_dir)
}

/// Save a profile to a specific directory
pub fn save_profile_to(profile: &Profile, dir: &Path) -> Result<()> {
    // Ensure directory exists
    std::fs::create_dir_all(dir).map_err(|e| {
        CoreError::ConfigWrite(format!("Failed to create profiles directory: {}", e))
    })?;

    // Use profile name (sanitized) as filename
    let filename = sanitize_filename(&profile.name);
    let path = dir.join(format!("{}.toml", filename));

    let content = toml::to_string_pretty(profile).map_err(|e| {
        CoreError::ConfigWrite(format!("Failed to serialize profile: {}", e))
    })?;

    std::fs::write(&path, content).map_err(|e| {
        CoreError::ConfigWrite(format!("Failed to write profile file: {}", e))
    })?;

    tracing::info!("Saved profile '{}' to {:?}", profile.name, path);
    Ok(())
}

/// Delete a profile file
pub fn delete_profile(profile: &Profile) -> Result<()> {
    let profiles_dir = paths::profiles_dir();
    let filename = sanitize_filename(&profile.name);
    let path = profiles_dir.join(format!("{}.toml", filename));

    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| {
            CoreError::ConfigWrite(format!("Failed to delete profile file: {}", e))
        })?;
        tracing::info!("Deleted profile '{}' from {:?}", profile.name, path);
    }

    Ok(())
}

/// Update an existing profile by its current name.
///
/// - If `updated.name` differs from `existing_name`, this performs a rename:
///   it saves the new profile and deletes the old profile file.
/// - If a different profile already exists with the new name, returns `ProfileAlreadyExists`.
pub fn update_profile(existing_name: &str, updated: &Profile) -> Result<()> {
    let profiles = load_profiles()?;

    let existing = profiles
        .iter()
        .find(|p| p.name == existing_name)
        .ok_or_else(|| CoreError::ProfileNotFound(existing_name.to_string()))?;

    if updated.name != existing_name && profiles.iter().any(|p| p.name == updated.name) {
        return Err(CoreError::ProfileAlreadyExists(updated.name.clone()));
    }

    // Save new/updated profile
    save_profile(updated)?;

    // If renamed, delete the old profile file.
    if updated.name != existing_name {
        delete_profile(existing)?;
    }

    Ok(())
}

/// Sanitize a string for use as a filename
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Initialize configuration directories and create default config if needed
pub fn init_config() -> Result<AppConfig> {
    paths::ensure_directories()?;

    let config_path = paths::config_file();
    
    if !config_path.exists() {
        let default_config = AppConfig::default();
        save_config(&default_config)?;
        tracing::info!("Created default configuration at {:?}", config_path);
        Ok(default_config)
    } else {
        load_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("my-profile"), "my-profile");
        assert_eq!(sanitize_filename("my profile"), "my_profile");
        assert_eq!(sanitize_filename("test@server"), "test_server");
        assert_eq!(sanitize_filename("profile/with/slashes"), "profile_with_slashes");
    }
}
