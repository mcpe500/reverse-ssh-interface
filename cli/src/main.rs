use clap::{Parser, Subcommand};
use anyhow::Result;

pub mod cmd;
pub mod output;

use cmd::profile::ProfileAction;

#[derive(Parser)]
#[command(name = "reverse-ssh-cli")]
#[command(about = "Manage reverse SSH connections", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a reverse SSH tunnel
    Up {
        /// Profile ID
        id: String,
    },
    /// Stop a reverse SSH tunnel
    Down {
        /// Profile ID
        id: String,
    },
    /// Show status of tunnels
    Status,
    /// View logs for a session
    Logs {
        /// Profile ID
        id: String,
        /// Follow logs
        #[arg(short, long)]
        follow: bool,
    },
    /// Manage profiles
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Basic logging to stderr
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Up { id } => {
            cmd::up::run(id).await?;
        }
        Commands::Down { id } => {
            cmd::down::run(id).await?;
        }
        Commands::Status => {
            cmd::status::run().await?;
        }
        Commands::Logs { id, follow } => {
            cmd::logs::run(id, follow).await?;
        }
        Commands::Profile { action } => {
            cmd::profile::run(action).await?;
        }
    }
    Ok(())
}
