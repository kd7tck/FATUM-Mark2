use std::collections::HashMap;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

#[derive(Debug)]
pub struct SimulationSession {
    pub seed: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct SimulationReport {
    pub total_simulations: usize,
    pub winner: String,
    pub distribution: HashMap<String, usize>,
    pub anomalies: Vec<String>,
}

impl SimulationSession {
    pub fn new(entropy: Vec<u8>) -> Self {
        // Create a 32-byte seed from the entropy.
        // If entropy is < 32 bytes, we pad. If > 32, we take the first 32 (or hash it).
        // For simplicity and maximizing usage, let's mix the entropy into 32 bytes.
        let mut seed = [0u8; 32];
        for (i, &byte) in entropy.iter().enumerate() {
            seed[i % 32] ^= byte;
        }
        Self { seed }
    }

    /// Runs a simulation for decision making.
    /// Uses a CSPRNG seeded by the quantum entropy.
    pub fn simulate_decision(&self, options: &[String], simulations: usize) -> SimulationReport {
        let mut distribution: HashMap<String, usize> = HashMap::new();
        for opt in options {
            distribution.insert(opt.clone(), 0);
        }

        let num_options = options.len();
        if num_options == 0 {
             return SimulationReport {
                total_simulations: 0,
                winner: "None".to_string(),
                distribution,
                anomalies: vec![],
            };
        }

        let mut rng = ChaCha20Rng::from_seed(self.seed);
        let mut counts = vec![0; num_options];

        for _ in 0..simulations {
            let index = rng.gen_range(0..num_options);
            counts[index] += 1;
        }

        // Populate distribution map
        for (i, count) in counts.iter().enumerate() {
            if let Some(opt) = options.get(i) {
                distribution.insert(opt.clone(), *count);
            }
        }

        // Find winner
        let mut max_count = 0;
        let mut winner = options[0].clone();
        for (opt, &count) in &distribution {
            if count > max_count {
                max_count = count;
                winner = opt.clone();
            }
        }

        // Anomaly detection
        let expected = simulations as f64 / num_options as f64;
        let mut anomalies = Vec::new();

        // Statistical significance check (Z-score approx)
        let std_dev = (simulations as f64 * (1.0/num_options as f64) * (1.0 - 1.0/num_options as f64)).sqrt();

        for (opt, &count) in &distribution {
             let diff = count as f64 - expected;
             let z_score = diff / std_dev;

             // If Z-score > 3 (roughly 99.7% confidence), it's an anomaly
             if z_score.abs() > 3.0 {
                 let direction = if z_score > 0.0 { "high" } else { "low" };
                 anomalies.push(format!("Option '{}' is statistically significant {} (Z={:.2})", opt, direction, z_score));
             }
        }

        SimulationReport {
            total_simulations: simulations,
            winner,
            distribution,
            anomalies,
        }
    }
}

#[cfg(test)]
mod tests;
