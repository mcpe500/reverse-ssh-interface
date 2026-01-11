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
