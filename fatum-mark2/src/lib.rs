use anyhow::{Context, Result};
use base64::prelude::*;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct CurbyClient {
    client: Client,
    base_url: String,
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
        }
    }

    async fn get_quantum_chain_id(&self) -> Result<String> {
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
                    return Ok(chain.cid.slash);
                }
            }
        }

        anyhow::bail!("CURBy-Q chain not found");
    }

    pub async fn get_latest_quantum_randomness(&self) -> Result<Vec<u8>> {
        let chain_id = self.get_quantum_chain_id().await?;

        // Fetch latest pulse info to get the current round number
        let latest_url = format!("{}/api/chains/{}/pulses/latest", self.base_url, chain_id);
        let latest_resp: PulseResponse = self.client.get(&latest_url)
            .send()
            .await?
            .json()
            .await
            .context("Failed to fetch latest pulse")?;

        let mut current_round = latest_resp.data.content.payload.round;

        // Try up to 5 times to find a finalized round
        for _ in 0..5 {
            let round_url = format!("{}/api/chains/{}/pulses/{}", self.base_url, chain_id, current_round);
            let resp_result = self.client.get(&round_url).send().await;

            if let Ok(resp) = resp_result {
                if resp.status().is_success() {
                    let pulse: PulseResponse = resp.json().await.context("Failed to parse pulse")?;
                    let payload = pulse.data.content.payload;

                    if payload.stage == "randomness" {
                        if let Some(wrapper) = payload.randomness {
                            let mut base64_string = wrapper.slash.bytes;
                            while base64_string.len() % 4 != 0 {
                                base64_string.push('=');
                            }
                            let bytes = BASE64_STANDARD.decode(&base64_string)
                                .context("Failed to decode base64 randomness")?;
                            return Ok(bytes);
                        }
                    }
                }
            }

            if current_round == 0 {
                break;
            }
            current_round -= 1;
        }

        anyhow::bail!("Could not find a finalized quantum randomness pulse in the last few rounds");
    }
}

impl Default for CurbyClient {
    fn default() -> Self {
        Self::new()
    }
}
