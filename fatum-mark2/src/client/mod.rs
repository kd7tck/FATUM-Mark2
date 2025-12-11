use anyhow::{Context, Result};
use base64::prelude::*;
use reqwest::Client;
use serde::Deserialize;
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand::rngs::OsRng;

#[derive(Debug, Clone)]
pub struct CurbyClient {
    client: Client,
    base_url: String,
    chain_id_cache: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChainResponse {
    cid: Cid,
    data: ChainData,
}

#[derive(Debug, Deserialize)]
struct ChainData {
    content: ChainContent,
}

#[derive(Debug, Deserialize)]
struct ChainContent {
    meta: ChainMeta,
}

#[derive(Debug, Deserialize)]
struct ChainMeta {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Cid {
    #[serde(rename = "/")]
    slash: String,
}

#[derive(Debug, Deserialize)]
struct PulseResponse {
    data: PulseData,
}

#[derive(Debug, Deserialize)]
struct PulseData {
    content: PulseContent,
}

#[derive(Debug, Deserialize)]
struct PulseContent {
    payload: PulsePayload,
}

#[derive(Debug, Deserialize)]
struct PulsePayload {
    stage: String,
    round: u64,
    #[serde(default)]
    randomness: Option<RandomnessWrapper>,
}

#[derive(Debug, Deserialize)]
struct RandomnessWrapper {
    #[serde(rename = "/")]
    slash: RandomnessBytes,
}

#[derive(Debug, Deserialize)]
struct RandomnessBytes {
    bytes: String,
}

impl CurbyClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder().timeout(std::time::Duration::from_secs(5)).build().unwrap(),
            base_url: "https://random.colorado.edu".to_string(),
            chain_id_cache: None,
        }
    }

    async fn get_quantum_chain_id(&mut self) -> Result<String> {
        if let Some(id) = &self.chain_id_cache {
            return Ok(id.clone());
        }

        let url = format!("{}/api/chains", self.base_url);
        let response_text = self.client.get(&url)
            .send()
            .await?
            .text()
            .await?;

        let chains: Vec<ChainResponse> = serde_json::from_str(&response_text)
            .context("Failed to parse chains list")?;

        for chain in chains {
            if let Some(name) = &chain.data.content.meta.name {
                if name == "CURBy-Q" {
                    let id = chain.cid.slash;
                    self.chain_id_cache = Some(id.clone());
                    return Ok(id);
                }
            }
        }

        anyhow::bail!("CURBy-Q chain not found");
    }

    /// Fetches a seed from Quantum source, then expands it via CSPRNG (ChaCha20).
    /// Fallback to OS RNG if network fails.
    pub async fn fetch_bulk_randomness(&mut self, min_bytes: usize) -> Result<Vec<u8>> {
        let seed = match self.fetch_single_pulse().await {
            Ok(s) => {
                println!("Successfully seeded with Quantum Entropy.");
                s
            },
            Err(e) => {
                eprintln!("Quantum Fetch Failed ({}), falling back to OS Entropy.", e);
                let mut os_seed = [0u8; 32];
                OsRng.fill_bytes(&mut os_seed);
                os_seed.to_vec()
            }
        };

        // Seed must be 32 bytes for ChaCha20
        let mut key = [0u8; 32];
        for (i, &b) in seed.iter().enumerate().take(32) {
            key[i] = b;
        }

        let mut rng = ChaCha20Rng::from_seed(key);
        let mut buffer = vec![0u8; min_bytes];
        rng.fill_bytes(&mut buffer);

        Ok(buffer)
    }

    async fn fetch_single_pulse(&mut self) -> Result<Vec<u8>> {
        let chain_id = self.get_quantum_chain_id().await?;
        let latest_url = format!("{}/api/chains/{}/pulses/latest", self.base_url, chain_id);

        let latest_resp: PulseResponse = self.client.get(&latest_url)
            .send()
            .await?
            .json()
            .await?;

        let mut current_round = latest_resp.data.content.payload.round;

        // Try up to 5 rounds backwards to find valid randomness
        for _ in 0..5 {
            let round_url = format!("{}/api/chains/{}/pulses/{}", self.base_url, chain_id, current_round);
            let resp = self.client.get(&round_url).send().await?;
            if resp.status().is_success() {
                if let Ok(pulse) = resp.json::<PulseResponse>().await {
                     let payload = pulse.data.content.payload;
                     if payload.stage == "randomness" {
                         if let Some(wrapper) = payload.randomness {
                             let mut base64_string = wrapper.slash.bytes;
                             while base64_string.len() % 4 != 0 { base64_string.push('='); }
                             return Ok(BASE64_STANDARD.decode(&base64_string)?);
                         }
                     }
                }
            }
            if current_round == 0 { break; }
            current_round -= 1;
        }
        anyhow::bail!("No valid randomness found in recent pulses");
    }
}

impl Default for CurbyClient {
    fn default() -> Self {
        Self::new()
    }
}
