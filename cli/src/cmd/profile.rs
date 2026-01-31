use anyhow::{Result, Context};
use reverse_ssh_core::{
    config::{load_profiles, paths, save_profile, delete_profile},
    types::{Profile, TunnelSpec, AuthMethod},
};
use uuid::Uuid;

use crate::output::OutputFormat;

pub async fn run_list(format: OutputFormat) -> Result<()> {
    let profiles = load_profiles()?;

    if profiles.is_empty() {
        println!("No profiles configured.");
        println!("Create one with: rssh profile add <name> --host <host> --user <user> --tunnel <spec>");
        return Ok(());
    }

    match format {
        OutputFormat::Human => {
            println!("Configured profiles:\n");
            for profile in &profiles {
                println!("  [{}]", profile.name);
                println!("    Host: {}@{}:{}", profile.user, profile.host, profile.port);
                println!("    Tunnels: {}", profile.tunnels.len());
                for tunnel in &profile.tunnels {
                    println!("      -R {}:{}:{}:{}", 
                        tunnel.remote_bind, tunnel.remote_port, 
                        tunnel.local_host, tunnel.local_port);
                }
                println!();
            }
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&profiles)?;
            println!("{}", json);
        }
    }

    Ok(())
}

pub async fn run_show(name: String, format: OutputFormat) -> Result<()> {
    let profiles = load_profiles()?;

    let profile = profiles.iter()
        .find(|p| p.name == name)
        .context(format!("Profile '{}' not found", name))?;

    match format {
        OutputFormat::Human => {
            println!("Profile: {}\n", profile.name);
            println!("  ID:       {}", profile.id);
            println!("  Host:     {}", profile.host);
            println!("  Port:     {}", profile.port);
            println!("  User:     {}", profile.user);
            println!("  Auth:     {}", format_auth(&profile.auth));
            println!("\n  Tunnels:");
            for tunnel in &profile.tunnels {
                println!("    -R {}:{}:{}:{}", 
                    tunnel.remote_bind, tunnel.remote_port,
                    tunnel.local_host, tunnel.local_port);
            }
            if !profile.extra_options.is_empty() {
                println!("\n  Extra SSH options: {:?}", profile.extra_options);
            }
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(profile)?;
            println!("{}", json);
        }
    }

    Ok(())
}

pub async fn run_add(
    name: String,
    host: String,
    user: String,
    port: Option<u16>,
    tunnels: Vec<String>,
    key_file: Option<String>,
) -> Result<()> {
    let profiles = load_profiles()?;

    if profiles.iter().any(|p| p.name == name) {
        anyhow::bail!("Profile '{}' already exists. Use 'profile remove' first to replace it.", name);
    }

    let parsed_tunnels = tunnels.iter()
        .map(|t| parse_tunnel_spec(t))
        .collect::<Result<Vec<_>>>()?;

    if parsed_tunnels.is_empty() {
        anyhow::bail!("At least one tunnel specification is required. Use --tunnel <remote_port>:<local_port>");
    }

    let auth = if let Some(key) = key_file {
        AuthMethod::KeyFile { path: key }
    } else {
        AuthMethod::Agent
    };

    let profile = Profile {
        id: Uuid::new_v4(),
        name: name.clone(),
        host,
        port: port.unwrap_or(22),
        user,
        auth,
        tunnels: parsed_tunnels,
        keepalive_interval: 20,
        keepalive_count: 3,
        auto_reconnect: true,
        max_reconnect_attempts: 0,
        extra_options: std::collections::HashMap::new(),
        ssh_path: None,
        known_hosts_file: None,
        identity_file: None,
    };

    save_profile(&profile)?;

    println!("Profile '{}' created successfully.", name);
    println!("Configuration saved to: {}", paths::profiles_dir().display());

    Ok(())
}

pub async fn run_remove(name: String) -> Result<()> {
    let profiles = load_profiles()?;

    let profile = profiles.iter()
        .find(|p| p.name == name)
        .context(format!("Profile '{}' not found.", name))?;

    delete_profile(profile)?;

    println!("Profile '{}' removed.", name);

    Ok(())
}

fn parse_tunnel_spec(spec: &str) -> Result<TunnelSpec> {
    // Format: remote_port:local_host:local_port
    // Or: remote_port:local_port (defaults local_host to localhost)
    let parts: Vec<&str> = spec.split(':').collect();

    match parts.len() {
        2 => {
            let remote_port: u16 = parts[0].parse()
                .context("Invalid remote port")?;
            let local_port: u16 = parts[1].parse()
                .context("Invalid local port")?;
            Ok(TunnelSpec {
                remote_bind: "localhost".to_string(),
                remote_port,
                local_host: "localhost".to_string(),
                local_port,
            })
        }
        3 => {
            let remote_port: u16 = parts[0].parse()
                .context("Invalid remote port")?;
            let local_host = parts[1].to_string();
            let local_port: u16 = parts[2].parse()
                .context("Invalid local port")?;
            Ok(TunnelSpec {
                remote_bind: "localhost".to_string(),
                remote_port,
                local_host,
                local_port,
            })
        }
        4 => {
            let remote_bind = parts[0].to_string();
            let remote_port: u16 = parts[1].parse()
                .context("Invalid remote port")?;
            let local_host = parts[2].to_string();
            let local_port: u16 = parts[3].parse()
                .context("Invalid local port")?;
            Ok(TunnelSpec {
                remote_bind,
                remote_port,
                local_host,
                local_port,
            })
        }
        _ => anyhow::bail!(
            "Invalid tunnel spec format. Use:\n  \
             remote_port:local_port\n  \
             remote_port:local_host:local_port\n  \
             remote_bind:remote_port:local_host:local_port"
        ),
    }
}

fn format_auth(auth: &AuthMethod) -> String {
    match auth {
        AuthMethod::Agent => "SSH Agent".to_string(),
        AuthMethod::KeyFile { path } => format!("Key file: {}", path),
        AuthMethod::Password => "Password".to_string(),
    }
}
