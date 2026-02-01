use std::path::{PathBuf};
use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;

use crate::error::{CoreError, Result};
use crate::types::{AuthMethod, Profile};

use super::args::{validate_args, SshArgs};
use super::detect::SshInfo;

/// Output from the SSH process
#[derive(Debug, Clone)]
pub enum SshOutput {
    Stdout(String),
    Stderr(String),
    Exit(Option<i32>),
}

/// Handle to a spawned SSH process
pub struct SshProcess {
    /// The child process
    child: Child,
    /// Process ID
    pub pid: u32,
    /// Channel for receiving output
    pub output_rx: mpsc::Receiver<SshOutput>,
}

impl SshProcess {
    /// Wait for the process to exit
    pub async fn wait(&mut self) -> Result<Option<i32>> {
        let status = self.child.wait().await?;
        Ok(status.code())
    }

    /// Kill the process
    pub async fn kill(&mut self) -> Result<()> {
        self.child.kill().await?;
        Ok(())
    }

    /// Check if the process is still running
    pub fn try_wait(&mut self) -> Result<Option<Option<i32>>> {
        match self.child.try_wait()? {
            Some(status) => Ok(Some(status.code())),
            None => Ok(None),
        }
    }
}

/// Spawn an SSH process for the given profile.
///
/// `password` is only used when `profile.auth` is `AuthMethod::Password`.
/// It is applied to the spawned child process environment as `SSHPASS`.
pub async fn spawn_ssh(
    ssh_info: &SshInfo,
    profile: &Profile,
    password: Option<&str>,
    sshpass_path: Option<&str>,
) -> Result<SshProcess> {
    let args = SshArgs::from_profile(profile).build_tunnel_mode();
    match profile.auth {
        AuthMethod::Password => spawn_ssh_with_password(ssh_info, args, password, sshpass_path).await,
        _ => spawn_ssh_with_args(ssh_info, args).await,
    }
}

fn find_in_path(exe_base_name: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    let separator = if cfg!(windows) { ';' } else { ':' };

    let candidates: Vec<String> = if cfg!(windows) {
        vec![
            format!("{}.exe", exe_base_name),
            format!("{}.cmd", exe_base_name),
            format!("{}.bat", exe_base_name),
            exe_base_name.to_string(),
        ]
    } else {
        vec![exe_base_name.to_string()]
    };

    for dir in std::env::split_paths(&path_var) {
        // split_paths handles platform separators, but keep `separator` above as a fallback guard.
        let _ = separator;
        for candidate in &candidates {
            let full = dir.join(candidate);
            if full.is_file() {
                return Some(full);
            }
        }
    }
    None
}

async fn spawn_ssh_with_password(
    ssh_info: &SshInfo,
    args: Vec<String>,
    password: Option<&str>,
    sshpass_path: Option<&str>,
) -> Result<SshProcess> {
    // Validate SSH args before spawning
    validate_args(&args).map_err(|e| CoreError::SshSpawnFailed(e))?;

    // sshpass reads the password from SSHPASS when using -e.
    // Prefer an explicitly-provided password (e.g. from frontend), otherwise fall back
    // to server environment.
    let has_password = password.is_some() || std::env::var("SSHPASS").is_ok();
    if !has_password {
        return Err(CoreError::SshSpawnFailed(
            "Password auth requires a password. Provide it via the start-session request (recommended) or set SSHPASS in the parent process environment.".to_string(),
        ));
    }

    let sshpass = if let Some(p) = sshpass_path {
        let p = PathBuf::from(p);
        if !p.is_file() {
            return Err(CoreError::SshSpawnFailed(
                "Password auth requires a valid sshpass_path (file not found).".to_string(),
            ));
        }
        p
    } else {
        find_in_path("sshpass").ok_or_else(|| {
            CoreError::SshSpawnFailed(
                "Password auth requires 'sshpass' to be installed and available in PATH, or provide sshpass_path.".to_string(),
            )
        })?
    };

    tracing::debug!(
        "Spawning SSH with password via sshpass. sshpass={:?} ssh={:?} args={:?}",
        sshpass,
        ssh_info.path,
        args
    );

    let mut cmd = Command::new(sshpass);
    cmd.arg("-e").arg(&ssh_info.path).args(&args);

    if let Some(pw) = password {
        cmd.env("SSHPASS", pw);
    }

    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| CoreError::SshSpawnFailed(e.to_string()))?;

    let pid = child.id().ok_or_else(|| {
        CoreError::SshSpawnFailed("Failed to get process ID".to_string())
    })?;

    let (tx, rx) = mpsc::channel(100);

    let stdout = child.stdout.take();
    let tx_stdout = tx.clone();
    if let Some(stdout) = stdout {
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if tx_stdout.send(SshOutput::Stdout(line)).await.is_err() {
                    break;
                }
            }
        });
    }

    let stderr = child.stderr.take();
    let tx_stderr = tx.clone();
    if let Some(stderr) = stderr {
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if tx_stderr.send(SshOutput::Stderr(line)).await.is_err() {
                    break;
                }
            }
        });
    }

    tracing::info!("Spawned SSH process (password) with PID {}", pid);

    Ok(SshProcess {
        child,
        pid,
        output_rx: rx,
    })
}

/// Spawn an SSH process with custom arguments
pub async fn spawn_ssh_with_args(ssh_info: &SshInfo, args: Vec<String>) -> Result<SshProcess> {
    // Validate arguments before spawning
    validate_args(&args).map_err(|e| CoreError::SshSpawnFailed(e))?;

    tracing::debug!("Spawning SSH with args: {:?}", args);

    let mut child = Command::new(&ssh_info.path)
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| CoreError::SshSpawnFailed(e.to_string()))?;

    let pid = child.id().ok_or_else(|| {
        CoreError::SshSpawnFailed("Failed to get process ID".to_string())
    })?;

    // Create channel for output
    let (tx, rx) = mpsc::channel(100);

    // Spawn task to read stdout
    let stdout = child.stdout.take();
    let tx_stdout = tx.clone();
    if let Some(stdout) = stdout {
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if tx_stdout.send(SshOutput::Stdout(line)).await.is_err() {
                    break;
                }
            }
        });
    }

    // Spawn task to read stderr
    let stderr = child.stderr.take();
    let tx_stderr = tx.clone();
    if let Some(stderr) = stderr {
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if tx_stderr.send(SshOutput::Stderr(line)).await.is_err() {
                    break;
                }
            }
        });
    }

    tracing::info!("Spawned SSH process with PID {}", pid);

    Ok(SshProcess {
        child,
        pid,
        output_rx: rx,
    })
}

/// Test SSH connection without establishing tunnels
/// Returns Ok(()) if connection succeeds, Err otherwise
pub async fn test_connection(ssh_info: &SshInfo, profile: &Profile) -> Result<()> {
    let args = SshArgs::new()
        .option("ServerAliveInterval", &profile.keepalive_interval.to_string())
        .option("ServerAliveCountMax", "1")
        .option("ConnectTimeout", "10")
        .option("BatchMode", "yes")
        .no_tty()
        .port(profile.port)
        .destination(&profile.destination())
        .build();

    // Add "exit" command to just test connection
    let mut full_args = args;
    full_args.push("exit".to_string());

    let output = Command::new(&ssh_info.path)
        .args(&full_args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| CoreError::SshSpawnFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(CoreError::SshExitError {
            code: output.status.code().unwrap_or(-1),
            message: stderr.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests would require actual SSH setup
    // These are placeholder tests

    #[test]
    fn test_ssh_output_enum() {
        let stdout = SshOutput::Stdout("test".to_string());
        let stderr = SshOutput::Stderr("error".to_string());
        let exit = SshOutput::Exit(Some(0));

        match stdout {
            SshOutput::Stdout(s) => assert_eq!(s, "test"),
            _ => panic!("Expected stdout"),
        }

        match stderr {
            SshOutput::Stderr(s) => assert_eq!(s, "error"),
            _ => panic!("Expected stderr"),
        }

        match exit {
            SshOutput::Exit(code) => assert_eq!(code, Some(0)),
            _ => panic!("Expected exit"),
        }
    }
}
