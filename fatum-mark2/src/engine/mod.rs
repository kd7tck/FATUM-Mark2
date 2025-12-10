use std::collections::HashMap;

#[derive(Debug)]
pub struct SimulationSession {
    entropy: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct SimulationReport {
    pub total_simulations: usize,
    pub winner: String,
    pub distribution: HashMap<String, usize>,
    pub anomalies: Vec<String>, // Description of any anomalies found
}

impl SimulationSession {
    pub fn new(entropy: Vec<u8>) -> Self {
        Self { entropy }
    }

    /// Runs a simulation for decision making.
    /// It treats the entropy as a stream of choices.
    /// `options`: The available choices.
    /// `simulations`: How many simulations to run.
    pub fn simulate_decision(&self, options: &[String], simulations: usize) -> SimulationReport {
        let mut distribution: HashMap<String, usize> = HashMap::new();
        for opt in options {
            distribution.insert(opt.clone(), 0);
        }

        // Simple strategy: modulo mapping.
        // We walk through the entropy bytes. Each byte (or set of bytes) maps to an index.

        let num_options = options.len();
        if num_options == 0 {
             return SimulationReport {
                total_simulations: 0,
                winner: "None".to_string(),
                distribution,
                anomalies: vec![],
            };
        }

        let mut counts = vec![0; num_options];
        let mut entropy_iter = self.entropy.iter().cycle(); // Cycle if we run out of entropy (though we should have enough if fetched correctly)

        for _ in 0..simulations {
            // In a real high-fidelity simulation, we might combine multiple bytes or look for specific bit patterns.
            // For now, we take one byte.
            if let Some(&byte) = entropy_iter.next() {
                let index = byte as usize % num_options;
                counts[index] += 1;
            }
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

        // Anomaly detection (very basic for now)
        // Check if any option is significantly deviating from expected distribution (uniform).
        let expected = simulations as f64 / num_options as f64;
        let mut anomalies = Vec::new();
        for (opt, &count) in &distribution {
             let deviation = (count as f64 - expected).abs();
             if deviation > (expected * 0.2) { // >20% deviation
                 anomalies.push(format!("Option '{}' deviated by {:.1}% from expected mean.", opt, (deviation/expected)*100.0));
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
