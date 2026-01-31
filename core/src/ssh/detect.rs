use std::path::PathBuf;
use std::process::Stdio;

use tokio::process::Command;

use crate::error::{CoreError, Result};

/// Information about the detected SSH binary
#[derive(Debug, Clone)]
pub struct SshInfo {
    /// Path to the SSH binary
    pub path: PathBuf,
    /// SSH version string (e.g., "OpenSSH_8.9p1")
    pub version: Option<String>,
    /// Whether this is OpenSSH (vs other implementations)
    pub is_openssh: bool,
}

impl SshInfo {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            version: None,
            is_openssh: false,
        }
    }
}

/// Detect the SSH binary on the system
/// 
/// Search order:
/// 1. Custom path (if provided)
/// 2. `which`/`where` lookup for "ssh"
/// 3. Common installation paths
pub async fn detect_ssh(custom_path: Option<&PathBuf>) -> Result<SshInfo> {
    // Try custom path first
    if let Some(path) = custom_path {
        if path.exists() {
            let mut info = SshInfo::new(path.clone());
            if let Ok(version) = get_ssh_version(path).await {
                info.version = Some(version.clone());
                info.is_openssh = version.contains("OpenSSH");
            }
            return Ok(info);
        } else {
            return Err(CoreError::SshNotExecutable(path.clone()));
        }
    }

    // Try to find ssh using `which` crate
    if let Ok(path) = which::which("ssh") {
        let mut info = SshInfo::new(path.clone());
        if let Ok(version) = get_ssh_version(&path).await {
            info.version = Some(version.clone());
            info.is_openssh = version.contains("OpenSSH");
        }
        return Ok(info);
    }

    // Try common paths as fallback
    let common_paths = get_common_ssh_paths();
    for path in common_paths {
        if path.exists() {
            let mut info = SshInfo::new(path.clone());
            if let Ok(version) = get_ssh_version(&path).await {
                info.version = Some(version.clone());
                info.is_openssh = version.contains("OpenSSH");
            }
            return Ok(info);
        }
    }

    Err(CoreError::SshNotFound)
}

/// Get common SSH binary paths for the current platform
fn get_common_ssh_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    #[cfg(windows)]
    {
        // Windows OpenSSH
        if let Ok(system_root) = std::env::var("SystemRoot") {
            paths.push(PathBuf::from(format!("{}\\System32\\OpenSSH\\ssh.exe", system_root)));
        }
        paths.push(PathBuf::from("C:\\Windows\\System32\\OpenSSH\\ssh.exe"));
        
        // Git for Windows
        if let Ok(program_files) = std::env::var("ProgramFiles") {
            paths.push(PathBuf::from(format!("{}\\Git\\usr\\bin\\ssh.exe", program_files)));
        }
        paths.push(PathBuf::from("C:\\Program Files\\Git\\usr\\bin\\ssh.exe"));
        
        // Chocolatey OpenSSH
        if let Ok(program_data) = std::env::var("ProgramData") {
            paths.push(PathBuf::from(format!("{}\\chocolatey\\bin\\ssh.exe", program_data)));
        }
    }

    #[cfg(unix)]
    {
        paths.push(PathBuf::from("/usr/bin/ssh"));
        paths.push(PathBuf::from("/usr/local/bin/ssh"));
        paths.push(PathBuf::from("/bin/ssh"));
        
        // Homebrew on macOS
        #[cfg(target_os = "macos")]
        {
            paths.push(PathBuf::from("/opt/homebrew/bin/ssh"));
            paths.push(PathBuf::from("/usr/local/opt/openssh/bin/ssh"));
        }
        
        // Termux on Android
        paths.push(PathBuf::from("/data/data/com.termux/files/usr/bin/ssh"));
    }

    paths
}

/// Get the SSH version string by running `ssh -V`
async fn get_ssh_version(ssh_path: &PathBuf) -> Result<String> {
    let output = Command::new(ssh_path)
        .arg("-V")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| CoreError::SshVersionDetection(e.to_string()))?;

    // SSH -V outputs to stderr
    let version = if !output.stderr.is_empty() {
        String::from_utf8_lossy(&output.stderr).trim().to_string()
    } else {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    };

    if version.is_empty() {
        return Err(CoreError::SshVersionDetection("Empty version output".to_string()));
    }

    Ok(version)
}

/// Verify that the SSH binary is functional
pub async fn verify_ssh(ssh_info: &SshInfo) -> Result<()> {
    let output = Command::new(&ssh_info.path)
        .arg("-V")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| CoreError::SshSpawnFailed(e.to_string()))?;

    // ssh -V exits with 0 on success
    if !output.status.success() && output.status.code() != Some(0) {
        // Some SSH implementations return non-zero for -V, check stderr
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.contains("SSH") && !stderr.contains("ssh") {
            return Err(CoreError::SshNotExecutable(ssh_info.path.clone()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_ssh() {
        // This test may fail if SSH is not installed
        let result = detect_ssh(None).await;
        // We just check it doesn't panic
        match result {
            Ok(info) => {
                println!("Found SSH at: {:?}", info.path);
                println!("Version: {:?}", info.version);
                println!("Is OpenSSH: {}", info.is_openssh);
            }
            Err(CoreError::SshNotFound) => {
                println!("SSH not found on this system");
            }
            Err(e) => {
                panic!("Unexpected error: {}", e);
            }
        }
    }
}
