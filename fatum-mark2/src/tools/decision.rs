use crate::client::CurbyClient;
use crate::engine::SimulationSession;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DecisionInput {
    pub options: Vec<String>,
    pub simulation_count: usize,
}

#[derive(Debug, Serialize)]
pub struct DecisionOutput {
    pub winner: String,
    pub report: String,
    pub distribution: std::collections::HashMap<String, usize>,
    pub anomalies: Vec<String>,
}

pub struct DecisionTool;

impl DecisionTool {
    pub async fn run(input: DecisionInput) -> Result<DecisionOutput> {
        let mut client = CurbyClient::new();

        // 1. Estimate needed entropy.
        // For simple modulo choice, we need 1 byte per simulation.
        let needed_bytes = input.simulation_count;

        // 2. Fetch entropy
        println!("DecisionTool: Fetching {} bytes of quantum data...", needed_bytes);
        let entropy = client.fetch_bulk_randomness(needed_bytes).await?;

        // 3. Create Session
        let session = SimulationSession::new(entropy);

        // 4. Run Simulation
        let report = session.simulate_decision(&input.options, input.simulation_count);

        // 5. Format Output
        let report_text = format!(
            "Ran {} simulations. The quantum noise patterns favored '{}' with {} hits.",
            report.total_simulations,
            report.winner,
            report.distribution.get(&report.winner).unwrap_or(&0)
        );

        Ok(DecisionOutput {
            winner: report.winner,
            report: report_text,
            distribution: report.distribution,
            anomalies: report.anomalies,
        })
    }
}
