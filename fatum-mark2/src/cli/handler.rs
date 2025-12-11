use clap::Parser;

#[derive(Parser)]
#[command(name = "FATUM-MARK2")]
#[command(author = "Jules")]
#[command(version = "1.0")]
#[command(about = "Quantum Feng Shui & Divination Engine", long_about = None)]
pub struct Cli {}

pub async fn handle_cli() {
    let _cli = Cli::parse();
    // Default and only behavior: Start Web Server
    println!("Starting Web Server...");
    fatum_mark2::server::start_server().await;
}
