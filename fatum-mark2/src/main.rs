use fatum_mark2::CurbyClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = CurbyClient::new();
    println!("Fetching latest quantum randomness from CURBy...");

    match client.get_latest_quantum_randomness().await {
        Ok(bytes) => {
            println!("Successfully fetched {} bytes of quantum randomness.", bytes.len());
            println!("Hex: {}", hex::encode(&bytes));
        }
        Err(e) => {
            eprintln!("Error fetching randomness: {:?}", e);
        }
    }

    Ok(())
}
