use clap::{Parser, Subcommand};
use fatum_mark2::tools::feng_shui::run_feng_shui_cli;

#[derive(Parser)]
#[command(name = "FATUM-MARK2")]
#[command(author = "Jules")]
#[command(version = "1.0")]
#[command(about = "Quantum Randomness Simulation Engine & Decision Tools", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Starts the Web Server (Default)
    Server,
    /// Runs the Feng Shui Tool
    FengShui,
}

pub async fn handle_cli() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::FengShui) => {
            if let Err(e) = run_feng_shui_cli().await {
                eprintln!("Error running Feng Shui tool: {}", e);
            }
        }
        Some(Commands::Server) | None => {
            println!("Starting Web Server...");
            fatum_mark2::server::start_server().await;
        }
    }
}
