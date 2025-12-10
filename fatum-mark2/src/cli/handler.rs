use clap::{Parser, Subcommand};
use fatum_mark2::tools::feng_shui::run_feng_shui_cli;
use fatum_mark2::tools::decision::run_decision_cli_interactive;

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
    /// Runs the Decision Tool
    Decision {
        /// Comma-separated options for simple mode
        #[arg(short, long)]
        options: Option<String>,

        /// Comma-separated weights (floats) corresponding to options
        #[arg(short, long)]
        weights: Option<String>,

        /// Path to JSON file defining a Decision Tree
        #[arg(short, long)]
        file: Option<String>,

        /// Number of simulations
        #[arg(long, default_value_t = 1000)]
        simulations: usize,
    },
}

pub async fn handle_cli() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::FengShui) => {
            if let Err(e) = run_feng_shui_cli().await {
                eprintln!("Error running Feng Shui tool: {}", e);
            }
        }
        Some(Commands::Decision { options, weights, file, simulations }) => {
            let opts_vec = options.as_ref().map(|s| s.split(',').map(|x| x.trim().to_string()).collect());
            let weights_vec = weights.as_ref().map(|s| {
                s.split(',')
                 .map(|x| x.trim().parse::<f64>().unwrap_or(1.0))
                 .collect()
            });

            if let Err(e) = run_decision_cli_interactive(opts_vec, weights_vec, file.clone(), *simulations).await {
                eprintln!("Error running Decision tool: {}", e);
            }
        }
        Some(Commands::Server) | None => {
            println!("Starting Web Server...");
            fatum_mark2::server::start_server().await;
        }
    }
}
