use crate::types::profile::{Profile, AuthMethod};

pub fn build_ssh_args(profile: &Profile) -> Vec<String> {
    let mut args = Vec::new();

    // Options first
    if let AuthMethod::IdentityFile(path) = &profile.auth {
        args.push("-i".to_string());
        args.push(path.clone());
    }

    args.push("-p".to_string());
    args.push(profile.port.to_string());

    // Stability / Safety Options
    args.push("-o".to_string());
    args.push("ExitOnForwardFailure=yes".to_string());

    args.push("-o".to_string());
    args.push(format!("ServerAliveInterval={}", profile.advanced.server_alive_interval));

    args.push("-o".to_string());
    args.push(format!("ServerAliveCountMax={}", profile.advanced.server_alive_count_max));
    
    // Non-interactive options
    // Only use BatchMode for non-password auth
    if !matches!(profile.auth, AuthMethod::Password(_)) {
        args.push("-o".to_string());
        args.push("BatchMode=yes".to_string());
    } else {
        // Ensure we DON'T use batch mode for password, otherwise it fails immediately
        args.push("-o".to_string());
        args.push("BatchMode=no".to_string());
    }

    // Host key checking - reasonable default for automated tools
    args.push("-o".to_string());
    args.push("StrictHostKeyChecking=accept-new".to_string());

    // Reverse Forwards
    for forward in &profile.forwards {
        args.push("-R".to_string());
        args.push(forward.to_arg_string());
    }

    // Do not execute remote command (just forward)
    args.push("-N".to_string());

    // Custom args
    if let Some(custom) = &profile.advanced.custom_args {
        args.extend(custom.clone());
    }

    // Destination (User@Host)
    args.push(format!("{}@{}", profile.user, profile.host));

    args
}

pub fn redact_ssh_args(args: &[String]) -> Vec<String> {
    let mut redacted = Vec::new();
    let mut redact_next = false;
    
    for arg in args {
        if redact_next {
            redacted.push("[REDACTED]".to_string());
            redact_next = false;
        } else if arg == "-i" {
            redacted.push(arg.clone());
            redact_next = true;
        } else {
            redacted.push(arg.clone());
        }
    }
    
    redacted
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::profile::{Profile, ForwardRule};

    #[test]
    fn test_build_ssh_args_basic() {
        let mut profile = Profile::new("test", "example.com", "user");
        profile.port = 2222;
        profile.forwards.push(ForwardRule {
            remote_port: 8080,
            remote_bind: "0.0.0.0".to_string(),
            local_host: "localhost".to_string(),
            local_port: 80,
        });

        let args = build_ssh_args(&profile);
        
        assert!(args.contains(&"-p".to_string()));
        assert!(args.contains(&"2222".to_string()));
        assert!(args.contains(&"-R".to_string()));
        assert!(args.contains(&"0.0.0.0:8080:localhost:80".to_string()));
        assert!(args.contains(&"-N".to_string()));
        assert!(args.contains(&"user@example.com".to_string()));
    }

    #[test]
    fn test_redact_ssh_args() {
        let args = vec![
            "-i".to_string(),
            "/path/to/secret_key".to_string(),
            "-p".to_string(),
            "22".to_string(),
            "user@host".to_string(),
        ];
        
        let redacted = redact_ssh_args(&args);
        
        assert_eq!(redacted[0], "-i");
        assert_eq!(redacted[1], "[REDACTED]");
        assert_eq!(redacted[2], "-p");
        assert_eq!(redacted[3], "22");
        assert_eq!(redacted[4], "user@host");
    }
}
