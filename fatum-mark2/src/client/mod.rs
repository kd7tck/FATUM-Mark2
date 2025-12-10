use anyhow::{Context, Result};
use base64::prelude::*;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct CurbyClient {
    client: Client,
    base_url: String,
    // Caching the chain ID so we don't fetch it every time
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
            client: Client::new(),
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

    /// Fetches at least `min_bytes` of quantum randomness.
    /// This may involve making multiple requests to the API, stepping backwards through rounds.
    pub async fn fetch_bulk_randomness(&mut self, min_bytes: usize) -> Result<Vec<u8>> {
        let chain_id = self.get_quantum_chain_id().await?;
        let mut buffer = Vec::with_capacity(min_bytes);

        // Fetch latest pulse info to get the start round
        let latest_url = format!("{}/api/chains/{}/pulses/latest", self.base_url, chain_id);
        let latest_resp: PulseResponse = self.client.get(&latest_url)
            .send()
            .await?
            .json()
            .await
            .context("Failed to fetch latest pulse")?;

        let mut current_round = latest_resp.data.content.payload.round;
        let mut failures = 0;
        const MAX_FAILURES: usize = 10;

        println!("Starting bulk fetch: goal {} bytes, starting round {}", min_bytes, current_round);

        while buffer.len() < min_bytes {
            let round_url = format!("{}/api/chains/{}/pulses/{}", self.base_url, chain_id, current_round);
            let resp_result = self.client.get(&round_url).send().await;

            match resp_result {
                Ok(resp) => {
                    if resp.status().is_success() {
                         match resp.json::<PulseResponse>().await {
                            Ok(pulse) => {
                                let payload = pulse.data.content.payload;
                                if payload.stage == "randomness" {
                                    if let Some(wrapper) = payload.randomness {
                                        let mut base64_string = wrapper.slash.bytes;
                                        // Pad base64 if necessary
                                        while base64_string.len() % 4 != 0 {
                                            base64_string.push('=');
                                        }
                                        match BASE64_STANDARD.decode(&base64_string) {
                                            Ok(mut bytes) => {
                                                buffer.append(&mut bytes);
                                            },
                                            Err(e) => {
                                                eprintln!("Failed to decode base64 for round {}: {}", current_round, e);
                                            }
                                        }
                                    }
                                }
                            },
                            Err(e) => eprintln!("Failed to parse pulse JSON for round {}: {}", current_round, e),
                        }
                    } else {
                        eprintln!("API returned non-success for round {}: {}", current_round, resp.status());
                        failures += 1;
                    }
                },
                Err(e) => {
                    eprintln!("Request failed for round {}: {}", current_round, e);
                    failures += 1;
                }
            }

            if failures > MAX_FAILURES {
                anyhow::bail!("Too many failures fetching randomness.");
            }

            if current_round == 0 {
                 anyhow::bail!("Reached round 0 but could not satisfy byte requirement.");
            }
            current_round -= 1;
        }

        Ok(buffer)
    }

    // Kept for backward compatibility if needed, though we will primarily use fetch_bulk_randomness
    pub async fn get_latest_quantum_randomness(&mut self) -> Result<Vec<u8>> {
        self.fetch_bulk_randomness(1).await // Minimal fetch
    }
}

impl Default for CurbyClient {
    fn default() -> Self {
        Self::new()
    }
}
