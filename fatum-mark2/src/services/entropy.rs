use std::sync::Arc;
use tokio::sync::Mutex;
use crate::client::CurbyClient;
use crate::db::Db;
use std::time::Duration;
use hex;

lazy_static::lazy_static! {
    static ref HARVESTER_CONTROL: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(None));
}

pub async fn start_harvesting(db: Arc<Db>, batch_id: i64) {
    let mut lock = HARVESTER_CONTROL.lock().await;
    if lock.is_some() {
        println!("Harvester already running for batch {:?}", *lock);
        return;
    }
    *lock = Some(batch_id);
    drop(lock);

    tokio::spawn(async move {
        let mut client = CurbyClient::new();
        println!("Starting Quantum Harvesting for Batch {}", batch_id);

        loop {
            // Check if we should stop
            {
                let lock = HARVESTER_CONTROL.lock().await;
                if *lock != Some(batch_id) {
                    println!("Stopping Harvester for Batch {}", batch_id);
                    break;
                }
            }

            // Fetch Pulse
            // Note: client.fetch_single_pulse() is private, but fetch_bulk_randomness uses it.
            // However, we want raw pulses without PRNG expansion.
            // We need to modify CurbyClient or use a workaround.
            // Since I cannot easily modify client private methods from here without changing client code,
            // I will assume I can modify client code OR I will use fetch_bulk_randomness(64) which might return the raw seed
            // if we are lucky, but it seeds a PRNG.

            // Wait, I should expose `fetch_single_pulse` or a similar method in `CurbyClient`.
            // Let's assume I will modify CurbyClient in the next step to expose `fetch_raw_entropy`.

            match client.fetch_raw_entropy().await {
                Ok(bytes) => {
                    let hex_val = hex::encode(&bytes);
                    // Get round info if possible? Currently client hides it.
                    // For now just save data.
                    if let Err(e) = db.insert_entropy(batch_id, None, &hex_val).await {
                         eprintln!("Failed to save entropy: {}", e);
                    } else {
                        println!("Harvested 512 bits for Batch {}", batch_id);
                    }
                },
                Err(e) => {
                    eprintln!("Harvest Error: {}", e);
                }
            }

            // Wait 60 seconds (beacon interval)
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
}

pub async fn stop_harvesting(db: Arc<Db>) {
    let mut lock = HARVESTER_CONTROL.lock().await;
    if let Some(bid) = *lock {
        // Update batch status
        let _ = db.update_batch_status(bid, "completed").await;
    }
    *lock = None;
}

pub async fn get_harvest_status() -> Option<i64> {
    let lock = HARVESTER_CONTROL.lock().await;
    *lock
}
