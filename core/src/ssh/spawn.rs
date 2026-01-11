use tokio::process::{Child, Command};
use std::process::Stdio;
use anyhow::{Result, Context};
use crate::types::profile::Profile;
use super::{detect, args};

pub fn spawn_session(profile: &Profile) -> Result<Child> {
    let ssh_path = detect::find_ssh_binary()?;
    let ssh_args = args::build_ssh_args(profile);

    // TODO: Logging the command for debug purposes (redact info later)
    tracing::debug!("Spawning: {:?} {:?}", ssh_path, ssh_args);

    let child = Command::new(ssh_path)
        .args(ssh_args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped()) 
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .context("Failed to spawn SSH process")?;

    Ok(child)
}
