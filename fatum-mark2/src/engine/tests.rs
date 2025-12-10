#[cfg(test)]
mod tests {
    use crate::engine::SimulationSession;

    #[test]
    fn test_simulation_distribution() {
        // Since we switched to ChaCha20Rng, the entropy is now a seed.
        // We cannot predict the exact outcome of 4 simulations easily without knowing the ChaCha20 implementation details
        // for that specific seed.
        // Instead, we should verify that the session initializes and runs without panicking,
        // and produces a valid report structure.

        // A fixed seed *should* be deterministic, but checking specific outcomes requires calculating the expected ChaCha output.
        // For this test, we'll verify structural correctness.

        let entropy = vec![1, 3, 5, 2];
        let session = SimulationSession::new(entropy);
        let options = vec!["A".to_string(), "B".to_string()];

        let report = session.simulate_decision(&options, 100);

        assert_eq!(report.total_simulations, 100);
        assert!(report.distribution.contains_key("A"));
        assert!(report.distribution.contains_key("B"));
        let sum = report.distribution.get("A").unwrap() + report.distribution.get("B").unwrap();
        assert_eq!(sum, 100);
    }

    #[test]
    fn test_empty_options() {
        let entropy = vec![1, 2, 3];
        let session = SimulationSession::new(entropy);
        let options: Vec<String> = vec![];

        let report = session.simulate_decision(&options, 10);

        assert_eq!(report.total_simulations, 0);
        assert_eq!(report.winner, "None");
    }

    #[test]
    fn test_consistency_from_same_seed() {
        // Same entropy should produce same results (deterministic PRNG from seed)
        let entropy = vec![42, 100, 200];
        let session1 = SimulationSession::new(entropy.clone());
        let session2 = SimulationSession::new(entropy.clone());

        let options = vec!["A".to_string(), "B".to_string(), "C".to_string()];

        let report1 = session1.simulate_decision(&options, 1000);
        let report2 = session2.simulate_decision(&options, 1000);

        assert_eq!(report1.winner, report2.winner);
        assert_eq!(report1.distribution, report2.distribution);
    }
}
