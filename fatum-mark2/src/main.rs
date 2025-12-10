mod cli;
use cli::handler::handle_cli;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    handle_cli().await;
    Ok(())
}
