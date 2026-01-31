use clap::{Parser, Subcommand};
use anyhow::Result;

pub mod cmd;
pub mod output;

use output::OutputFormat;

#[derive(Parser)]
#[command(name = "rssh")]
#[command(about = "Manage reverse SSH connections", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a reverse SSH tunnel
    Up {
        /// Profile name
        profile: String,
    },
    /// Stop a reverse SSH tunnel
    Down {
        /// Session ID (UUID)
        session_id: String,
    },
    /// Show status of tunnels
    Status {
        /// Session ID (optional)
        #[arg(short, long)]
        session: Option<String>,
        
        /// Output format
        #[arg(short, long, value_enum, default_value = "human")]
        format: OutputFormat,
    },
    /// View logs for a session
    Logs {
        /// Session ID (optional, lists all if not provided)
        session: Option<String>,
        
        /// Follow logs
        #[arg(short, long)]
        follow: bool,
        
        /// Number of lines to show
        #[arg(short = 'n', long, default_value = "50")]
        lines: usize,
    },
    /// Manage profiles
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
}

#[derive(Subcommand)]
enum ProfileAction {
    /// List all profiles
    List {
        /// Output format
        #[arg(short, long, value_enum, default_value = "human")]
        format: OutputFormat,
    },
    /// Show details of a profile
    Show {
        /// Profile name
        name: String,
        
        /// Output format
        #[arg(short, long, value_enum, default_value = "human")]
        format: OutputFormat,
    },
    /// Add a new profile
    Add {
        /// Profile name
        name: String,
        
        /// Remote host
        #[arg(short = 'H', long)]
        host: String,
        
        /// Remote user
        #[arg(short, long)]
        user: String,
        
        /// Remote port
        #[arg(short, long)]
        port: Option<u16>,
        
        /// Tunnel specifications (format: remote_port:local_host:local_port)
        #[arg(short, long)]
        tunnel: Vec<String>,
        
        /// Path to SSH key file
        #[arg(short, long)]
        key: Option<String>,
    },
    /// Remove a profile
    Remove {
        /// Profile name
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Up { profile } => {
            cmd::up::run(profile).await?;
        }
        Commands::Down { session_id } => {
            cmd::down::run(session_id).await?;
        }
        Commands::Status { session, format } => {
            cmd::status::run(session, format).await?;
        }
        Commands::Logs { session, follow, lines } => {
            cmd::logs::run(session, follow, lines).await?;
        }
        Commands::Profile { action } => {
            match action {
                ProfileAction::List { format } => {
                    cmd::profile::run_list(format).await?;
                }
                ProfileAction::Show { name, format } => {
                    cmd::profile::run_show(name, format).await?;
                }
                ProfileAction::Add { name, host, user, port, tunnel, key } => {
                    cmd::profile::run_add(name, host, user, port, tunnel, key).await?;
                }
                ProfileAction::Remove { name } => {
                    cmd::profile::run_remove(name).await?;
                }
            }
        }
    }
    Ok(())
}
