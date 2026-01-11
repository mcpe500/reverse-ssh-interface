use anyhow::{Context, Result};
use tokio::fs;
use crate::config::{model::AppConfig, paths};

pub async fn load_config() -> Result<AppConfig> {
    let path = paths::main_config_file()?;
    
    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let content = fs::read_to_string(&path)
        .await
        .context(format!("Failed to read config file at {:?}", path))?;
    
    let config: AppConfig = toml::from_str(&content)
        .context("Failed to parse config file (TOML)")?;
        
    Ok(config)
}

pub async fn save_config(config: &AppConfig) -> Result<()> {
    let path = paths::main_config_file()?;
    
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .await
            .context("Failed to create config directory")?;
    }

    let content = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;
        
    fs::write(&path, content)
        .await
        .context(format!("Failed to write config file to {:?}", path))?;
        
    Ok(())
}
