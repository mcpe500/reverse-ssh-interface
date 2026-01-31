use std::collections::HashMap;

use crate::config::StrictHostKeyChecking;
use crate::types::{AuthMethod, Profile, TunnelSpec};

/// SSH argument builder
/// 
/// Builds safe argument arrays (never shell strings) for SSH commands.
/// This is critical for security - we never construct shell command strings.
#[derive(Debug, Clone)]
pub struct SshArgs {
    args: Vec<String>,
}

impl SshArgs {
    /// Create a new SSH argument builder
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }

    /// Build SSH arguments from a profile
    pub fn from_profile(profile: &Profile) -> Self {
        let mut builder = Self::new();

        // Add reverse tunnel specifications (-R)
        for tunnel in &profile.tunnels {
            builder = builder.reverse_tunnel(tunnel);
        }

        // Add keepalive options
        builder = builder
            .option("ServerAliveInterval", &profile.keepalive_interval.to_string())
            .option("ServerAliveCountMax", &profile.keepalive_count.to_string());

        // Add safety options
        builder = builder
            .option("ExitOnForwardFailure", "yes")
            .option("BatchMode", "yes"); // Disable password prompts in non-interactive mode

        // Add authentication options
        match &profile.auth {
            AuthMethod::Agent => {
                // Use SSH agent (default behavior)
                builder = builder.option("IdentitiesOnly", "yes");
            }
            AuthMethod::KeyFile { path } => {
                builder = builder
                    .identity_file(path)
                    .option("IdentitiesOnly", "yes");
            }
            AuthMethod::Password => {
                // Password auth - BatchMode will be disabled
                builder.args.retain(|a| !a.contains("BatchMode"));
            }
        }

        // Add custom identity file if specified
        if let Some(ref identity) = profile.identity_file {
            builder = builder.identity_file(identity);
        }

        // Add custom known_hosts file if specified
        if let Some(ref known_hosts) = profile.known_hosts_file {
            builder = builder.option("UserKnownHostsFile", known_hosts);
        }

        // Add extra options
        for (key, value) in &profile.extra_options {
            builder = builder.option(key, value);
        }

        // Add port if not default
        if profile.port != 22 {
            builder = builder.port(profile.port);
        }

        // Add destination (must be last before any command)
        builder = builder.destination(&profile.destination());

        builder
    }

    /// Add a generic SSH option (-o key=value)
    pub fn option(mut self, key: &str, value: &str) -> Self {
        self.args.push("-o".to_string());
        self.args.push(format!("{}={}", key, value));
        self
    }

    /// Add multiple options from a HashMap
    pub fn options(mut self, options: &HashMap<String, String>) -> Self {
        for (key, value) in options {
            self = self.option(key, value);
        }
        self
    }

    /// Add strict host key checking option
    pub fn strict_host_key_checking(self, mode: StrictHostKeyChecking) -> Self {
        self.option("StrictHostKeyChecking", mode.to_ssh_option())
    }

    /// Add a reverse tunnel (-R)
    pub fn reverse_tunnel(mut self, tunnel: &TunnelSpec) -> Self {
        self.args.push("-R".to_string());
        self.args.push(tunnel.to_ssh_arg());
        self
    }

    /// Add identity file (-i)
    pub fn identity_file(mut self, path: &str) -> Self {
        self.args.push("-i".to_string());
        self.args.push(path.to_string());
        self
    }

    /// Add port (-p)
    pub fn port(mut self, port: u16) -> Self {
        self.args.push("-p".to_string());
        self.args.push(port.to_string());
        self
    }

    /// Add destination (user@host)
    pub fn destination(mut self, dest: &str) -> Self {
        self.args.push(dest.to_string());
        self
    }

    /// Add verbose flag (-v, -vv, -vvv)
    pub fn verbose(mut self, level: u8) -> Self {
        let flag = match level {
            0 => return self,
            1 => "-v",
            2 => "-vv",
            _ => "-vvv",
        };
        self.args.push(flag.to_string());
        self
    }

    /// Add no-TTY flag (-T) - useful for tunnels
    pub fn no_tty(mut self) -> Self {
        self.args.push("-T".to_string());
        self
    }

    /// Add no-command flag (-N) - useful for tunnels only
    pub fn no_command(mut self) -> Self {
        self.args.push("-N".to_string());
        self
    }

    /// Add compression flag (-C)
    pub fn compression(mut self) -> Self {
        self.args.push("-C".to_string());
        self
    }

    /// Build the final argument array
    pub fn build(self) -> Vec<String> {
        self.args
    }

    /// Build arguments specifically for tunnel-only mode
    /// Adds -N (no command) and -T (no TTY)
    pub fn build_tunnel_mode(self) -> Vec<String> {
        let mut args = vec!["-N".to_string(), "-T".to_string()];
        args.extend(self.args);
        args
    }
}

impl Default for SshArgs {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate that arguments don't contain dangerous patterns
pub fn validate_args(args: &[String]) -> Result<(), String> {
    for arg in args {
        // Check for shell metacharacters that shouldn't be in args
        // when passed as an array (not shell string)
        if arg.contains('\0') {
            return Err("Argument contains null byte".to_string());
        }
        
        // Check for ProxyCommand injection attempts
        if arg.to_lowercase().contains("proxycommand") {
            let value = arg.split('=').nth(1).unwrap_or("");
            // Allow simple ProxyCommand but warn about complex ones
            if value.contains(';') || value.contains('|') || value.contains('`') {
                return Err("Potentially dangerous ProxyCommand detected".to_string());
            }
        }

        // Check for LocalCommand injection
        if arg.to_lowercase().contains("localcommand") {
            return Err("LocalCommand option is not allowed".to_string());
        }

        // Check for PermitLocalCommand
        if arg.to_lowercase().contains("permitlocalcommand") {
            return Err("PermitLocalCommand option is not allowed".to_string());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TunnelSpec;

    #[test]
    fn test_ssh_args_basic() {
        let args = SshArgs::new()
            .option("ServerAliveInterval", "20")
            .port(2222)
            .destination("user@example.com")
            .build();

        assert!(args.contains(&"-o".to_string()));
        assert!(args.contains(&"ServerAliveInterval=20".to_string()));
        assert!(args.contains(&"-p".to_string()));
        assert!(args.contains(&"2222".to_string()));
        assert!(args.contains(&"user@example.com".to_string()));
    }

    #[test]
    fn test_ssh_args_reverse_tunnel() {
        let tunnel = TunnelSpec::new(8080, 3000);
        let args = SshArgs::new()
            .reverse_tunnel(&tunnel)
            .destination("user@example.com")
            .build();

        assert!(args.contains(&"-R".to_string()));
        assert!(args.contains(&"localhost:8080:localhost:3000".to_string()));
    }

    #[test]
    fn test_ssh_args_from_profile() {
        let mut profile = Profile::new("test", "example.com", "testuser");
        profile.tunnels.push(TunnelSpec::new(8080, 3000));
        profile.port = 2222;

        let args = SshArgs::from_profile(&profile).build();

        assert!(args.contains(&"-R".to_string()));
        assert!(args.contains(&"-p".to_string()));
        assert!(args.contains(&"2222".to_string()));
        assert!(args.contains(&"testuser@example.com".to_string()));
    }

    #[test]
    fn test_validate_args_safe() {
        let args = vec![
            "-o".to_string(),
            "ServerAliveInterval=20".to_string(),
            "-R".to_string(),
            "localhost:8080:localhost:3000".to_string(),
            "user@example.com".to_string(),
        ];
        assert!(validate_args(&args).is_ok());
    }

    #[test]
    fn test_validate_args_dangerous() {
        let args = vec![
            "-o".to_string(),
            "LocalCommand=rm -rf /".to_string(),
        ];
        assert!(validate_args(&args).is_err());
    }
}
