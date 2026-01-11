use std::path::PathBuf;
use anyhow::{Context, Result};

pub fn app_config_dir() -> Result<PathBuf> {
    let mut dir = dirs::config_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
        .context("Could not determine config directory")?;
    
    dir.push("reverse-ssh-interface");
    Ok(dir)
}

pub fn main_config_file() -> Result<PathBuf> {
    Ok(app_config_dir()?.join("config.toml"))
}
