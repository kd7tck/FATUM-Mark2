use std::collections::HashMap;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct SimulationSession {
    pub seed: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeStep {
    pub step_index: usize,
    pub distribution: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationReport {
    pub total_simulations: usize,
    pub winner: String,
    pub distribution: HashMap<String, usize>,
    pub anomalies: Vec<String>,
    pub time_series: Vec<TimeStep>,
}

impl SimulationSession {
    pub fn new(entropy: Vec<u8>) -> Self {
        let mut seed = [0u8; 32];
        for (i, &byte) in entropy.iter().enumerate() {
            seed[i % 32] ^= byte;
        }
        Self { seed }
    }

    /// Runs a simulation for decision making with weighted options and time-series tracking.
    pub fn simulate_decision(
        &self,
        options: &[String],
        weights: Option<&[f64]>,
        simulations: usize
    ) -> SimulationReport {
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
                time_series: vec![],
            };
        }

        let mut rng = ChaCha20Rng::from_seed(self.seed);
        let mut counts = vec![0; num_options];
        let mut time_series = Vec::new();

        // Setup weights (cumulative distribution)
        let mut cdf = Vec::with_capacity(num_options);
        if let Some(w) = weights {
            let sum: f64 = w.iter().sum();
            let mut acc = 0.0;
            for &val in w {
                acc += val / sum;
                cdf.push(acc);
            }
        } else {
            // Equal weights
            let step = 1.0 / num_options as f64;
            let mut acc = 0.0;
            for _ in 0..num_options {
                acc += step;
                cdf.push(acc);
            }
        }
        // Ensure last is exactly 1.0 to avoid float issues
        if let Some(last) = cdf.last_mut() {
            *last = 1.0;
        }

        let step_size = (simulations / 20).max(1); // Record roughly 20 data points

        for i in 1..=simulations {
            let r: f64 = rng.gen();
            // Find which option corresponds to r
            let mut choice_idx = 0;
            for (idx, &threshold) in cdf.iter().enumerate() {
                if r <= threshold {
                    choice_idx = idx;
                    break;
                }
            }
            // Fallback for float rounding errors
            if choice_idx >= num_options { choice_idx = num_options - 1; }

            counts[choice_idx] += 1;

            if i % step_size == 0 || i == simulations {
                 let mut step_dist = HashMap::new();
                 for (idx, count) in counts.iter().enumerate() {
                    if let Some(opt) = options.get(idx) {
                        step_dist.insert(opt.clone(), *count);
                    }
                 }
                 time_series.push(TimeStep {
                     step_index: i,
                     distribution: step_dist,
                 });
            }
        }

        // Populate final distribution map
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
        // Note: For weighted options, expected value changes.
        let mut anomalies = Vec::new();
        // Calculate expected counts based on weights
        for (idx, opt) in options.iter().enumerate() {
            let weight_prob = if let Some(w) = weights {
                 w[idx] / w.iter().sum::<f64>()
            } else {
                 1.0 / num_options as f64
            };

            let expected = simulations as f64 * weight_prob;
            let std_dev = (simulations as f64 * weight_prob * (1.0 - weight_prob)).sqrt();

            let count = *distribution.get(opt).unwrap_or(&0);
            let diff = count as f64 - expected;
            let z_score = if std_dev > 0.0 { diff / std_dev } else { 0.0 };

            if z_score.abs() > 3.0 {
                 let direction = if z_score > 0.0 { "high" } else { "low" };
                 anomalies.push(format!("Option '{}' is significant {} (Z={:.2})", opt, direction, z_score));
            }
        }

        SimulationReport {
            total_simulations: simulations,
            winner,
            distribution,
            anomalies,
            time_series,
        }
    }
}

#[cfg(test)]
mod tests;
