use crate::engine::SimulationSession;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineState {
    pub step_index: usize, // 0 to duration
    pub score: f64,        // 0.0 to 100.0
    pub dominant_element: String,
    pub elemental_values: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelinePath {
    pub id: usize,
    pub steps: Vec<TimelineState>,
    pub final_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateStep {
    pub step_index: usize,
    pub avg_score: f64,
    pub variance: f64,
    pub element_distribution: HashMap<String, usize>, // Count of dominant elements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManyWorldsResult {
    pub paths: Vec<TimelinePath>, // Subset of paths
    pub aggregate_stats: Vec<AggregateStep>, // Average/Min/Max per year
}

pub struct TimelineSimulator<'a> {
    session: &'a mut SimulationSession,
}

impl<'a> TimelineSimulator<'a> {
    pub fn new(session: &'a mut SimulationSession) -> Self {
        Self { session }
    }

    /// Simulates branching timelines.
    ///
    /// * `start_elements`: Initial elemental balance (Wood, Fire, Earth, Metal, Water).
    /// * `duration`: Number of steps (e.g., years) to simulate.
    /// * `num_worlds`: Number of timelines to generate.
    pub fn simulate(
        &mut self,
        start_elements: HashMap<String, f64>,
        duration: usize,
        num_worlds: usize,
    ) -> ManyWorldsResult {
        let mut all_paths = Vec::with_capacity(num_worlds);
        let mut rng = ChaCha20Rng::from_seed(self.session.seed);

        for i in 0..num_worlds {
            let mut current_elements = start_elements.clone();
            let mut steps = Vec::with_capacity(duration);

            // Initial score calculation
            let mut current_score = self.calculate_score(&current_elements);

            for step in 0..duration {
                // Evolve elements based on Entropy
                let entropy_flux = self.session.next_f64(&mut rng);

                // Determine which element gets boosted/drained
                // 0.0-0.2: Wood, 0.2-0.4: Fire, etc.
                let element_idx = (entropy_flux * 5.0) as usize;
                let boosted_element = match element_idx {
                    0 => "Wood",
                    1 => "Fire",
                    2 => "Earth",
                    3 => "Metal",
                    _ => "Water",
                };

                // Apply flux
                // A second random number determines magnitude
                let magnitude = self.session.next_f64(&mut rng) * 10.0 - 2.0; // -2 to +8 range

                if let Some(val) = current_elements.get_mut(boosted_element) {
                    *val = (*val + magnitude).max(0.0);
                }

                // Normalization (optional, to keep values sane)
                // But let's just let them drift for now to see "extreme" timelines.

                // Calculate Dominant Element
                let mut max_val = -1.0;
                let mut dom = "Unknown".to_string();
                for (k, v) in &current_elements {
                    if *v > max_val {
                        max_val = *v;
                        dom = k.clone();
                    }
                }

                current_score = self.calculate_score(&current_elements);

                steps.push(TimelineState {
                    step_index: step,
                    score: current_score,
                    dominant_element: dom,
                    elemental_values: current_elements.clone(),
                });
            }

            all_paths.push(TimelinePath {
                id: i,
                final_score: current_score,
                steps,
            });
        }

        // Calculate Aggregates
        let mut aggregates = Vec::new();
        for step in 0..duration {
            let mut total_score = 0.0;
            let mut score_sq_sum = 0.0;
            let mut elem_dist = HashMap::new();

            for path in &all_paths {
                if let Some(s) = path.steps.get(step) {
                    total_score += s.score;
                    score_sq_sum += s.score * s.score;
                    *elem_dist.entry(s.dominant_element.clone()).or_insert(0) += 1;
                }
            }

            let avg = total_score / num_worlds as f64;
            let variance = (score_sq_sum / num_worlds as f64) - (avg * avg);

            aggregates.push(AggregateStep {
                step_index: step,
                avg_score: avg,
                variance,
                element_distribution: elem_dist,
            });
        }

        // Return top 50 paths to avoid massive JSON payload
        let paths_to_return = all_paths.into_iter().take(50).collect();

        ManyWorldsResult {
            paths: paths_to_return,
            aggregate_stats: aggregates,
        }
    }

    fn calculate_score(&self, elements: &HashMap<String, f64>) -> f64 {
        // Simple scoring: Balance is better? Or just sum?
        // Let's assume a "Flow" score where standard deviation is low (balanced) is higher score?
        // Or maybe just the sum of energy.
        // Let's go with Sum of Energy for now.
        elements.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::engine::SimulationSession;

    #[test]
    fn test_many_worlds_simulation() {
        let entropy = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Weak entropy but enough for test
        let mut session = SimulationSession::new(entropy);
        let mut simulator = TimelineSimulator::new(&mut session);

        let mut start_elements = HashMap::new();
        start_elements.insert("Wood".to_string(), 20.0);
        start_elements.insert("Fire".to_string(), 20.0);

        let result = simulator.simulate(start_elements, 10, 5);

        assert_eq!(result.paths.len(), 5);
        assert_eq!(result.paths[0].steps.len(), 10);
        assert_eq!(result.aggregate_stats.len(), 10);
    }
}
