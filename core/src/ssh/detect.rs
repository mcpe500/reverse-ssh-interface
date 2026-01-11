use anyhow::{Result, Context};
use std::path::PathBuf;

pub fn find_ssh_binary() -> Result<PathBuf> {
    which::which("ssh").context("Could not find 'ssh' binary in PATH")
}
