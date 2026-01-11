use anyhow::Result;
use clap::Subcommand;
use inquire::{Text, Confirm};
use reverse_ssh_core::{
    config::load,
    types::profile::{Profile, ForwardRule},
};

#[derive(Subcommand, Debug, Clone)]
pub enum ProfileAction {
    /// Add a new profile interactively
    Add,
    /// List all profiles
    List,
}

pub async fn run(action: ProfileAction) -> Result<()> {
    match action {
        ProfileAction::Add => add_profile().await,
        ProfileAction::List => list_profiles().await,
    }
}

async fn list_profiles() -> Result<()> {
    let config = load::load_config().await?;
    if config.profiles.is_empty() {
        println!("No profiles found. Use 'add' to create one.");
        return Ok(());
    }
    
    println!("Found {} profiles:", config.profiles.len());
    for (id, profile) in config.profiles {
        println!("- [{}] {}@{} (Port: {})", id, profile.user, profile.host, profile.port);
        for fwd in profile.forwards {
            println!("    R: {}", fwd.to_arg_string());
        }
    }
    Ok(())
}

async fn add_profile() -> Result<()> {
    println!("Creating a new Reverse SSH Profile...");
    
    let id = Text::new("Profile Name (ID):").with_help_message("Unique name for this connection").prompt()?;
    let host = Text::new("SSH Host IP/Domain:").prompt()?;
    let user = Text::new("SSH User:").prompt()?;
    let port_str = Text::new("SSH Port:").with_default("22").prompt()?;
    let port = port_str.parse::<u16>().unwrap_or(22);
    
    let mut profile = Profile::new(&id, &host, &user);
    profile.port = port;

    // Optional Key Path
    if Confirm::new("Specify a private key path?").with_default(false).prompt()? {
        let key = Text::new("Path to private key:").prompt()?;
        profile.key_path = Some(key);
    }

    // Add forwards
    loop {
        if !Confirm::new("Add a reverse forward rule (-R)?").with_default(true).prompt()? {
            break;
        }
        
        let remote_port_str = Text::new("Remote Port (server port to open):").prompt()?;
        let remote_port = remote_port_str.parse::<u16>().unwrap_or(8080);
        
        let local_port_str = Text::new("Local Port (device port to forward):").prompt()?;
        let local_port = local_port_str.parse::<u16>().unwrap_or(8080);
        
        profile.forwards.push(ForwardRule {
            remote_port,
            remote_bind: "127.0.0.1".to_string(), // Default safe bind
            local_host: "localhost".to_string(),
            local_port,
        });
        
        println!("Added forward: {} -> {}", remote_port, local_port);
    }

    let mut config = load::load_config().await?;
    config.add_profile(profile);
    load::save_config(&config).await?;
    
    println!("Profile '{}' saved successfully!", id);
    Ok(())
}
