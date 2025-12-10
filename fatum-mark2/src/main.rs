use fatum_mark2::server::start_server;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    start_server().await;
    Ok(())
}
