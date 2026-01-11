use tokio::process::{Child, Command};
use std::process::Stdio;
use anyhow::{Result, Context};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use tempfile::NamedTempFile;
use crate::types::profile::{Profile, AuthMethod};
use super::{detect, args};

pub fn spawn_session(profile: &Profile) -> Result<(Child, Option<NamedTempFile>)> {
    let ssh_path = detect::find_ssh_binary()?;
    let ssh_args = args::build_ssh_args(profile);

    let safe_args = args::redact_ssh_args(&ssh_args);
    tracing::debug!("Spawning: {:?} {:?}", ssh_path, safe_args);

    let mut cmd = Command::new(ssh_path);
    cmd.args(ssh_args)
       .stdin(Stdio::null())
       .stdout(Stdio::piped()) 
       .stderr(Stdio::piped())
       .kill_on_drop(true);

    // Handle Password Auth via SSH_ASKPASS
    let askpass_file = if let AuthMethod::Password(pass) = &profile.auth {
        // Create a temporary script
        let mut file = NamedTempFile::new().context("Failed to create temp askpass file")?;
        
        // Write shell script content
        // We use single quotes for the password to avoid basic shell expansion issues, 
        // but robust escaping would be better for complex passwords. 
        // For this MVP, we assume reasonably standard passwords.
        let script = format!("#!/bin/sh\necho '{}'\n", pass.replace("'", "'\\''"));
        file.write_all(script.as_bytes()).context("Failed to write askpass script")?;
        
        // Make executable
        let mut perms = file.as_file().metadata()?.permissions();
        perms.set_mode(0o700);
        file.as_file().set_permissions(perms)?;

        // Set env vars
        cmd.env("SSH_ASKPASS", file.path());
        cmd.env("DISPLAY", ":0"); // Helper to trigger askpass
        
        // Setsid is often required so ssh doesn't think it has a controlling terminal
        // and force-reads from it. But tokio process spawning usually detaches enough.
        // If needed, we could use `.setsid()` from `std::os::unix::process::CommandExt`.
        
        Some(file)
    } else {
        None
    };

    let child = cmd.spawn().context("Failed to spawn SSH process")?;

    Ok((child, askpass_file))
}
