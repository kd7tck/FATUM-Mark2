#[cfg(test)]
mod tests {
    use crate::engine::SimulationSession;

    #[test]
    fn test_simulation_distribution() {
        // Entropy that favors index 1 (odd numbers)
        // Options: A, B. Index 0 = A, Index 1 = B.
        // Entropy: [1, 3, 5, 2] -> 1%2=1 (B), 3%2=1 (B), 5%2=1 (B), 2%2=0 (A)
        // Winner should be B (3 hits vs 1 hit)
        let entropy = vec![1, 3, 5, 2];
        let session = SimulationSession::new(entropy);
        let options = vec!["A".to_string(), "B".to_string()];

        let report = session.simulate_decision(&options, 4);

        assert_eq!(report.total_simulations, 4);
        assert_eq!(report.winner, "B");
        assert_eq!(report.distribution.get("A"), Some(&1));
        assert_eq!(report.distribution.get("B"), Some(&3));
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
    fn test_insufficient_entropy_cycles() {
        // Only 1 byte of entropy, but 5 simulations requested.
        // Entropy: [1]. Options: A, B.
        // Should cycle: 1 (B), 1 (B), 1 (B), 1 (B), 1 (B)
        let entropy = vec![1];
        let session = SimulationSession::new(entropy);
        let options = vec!["A".to_string(), "B".to_string()];

        let report = session.simulate_decision(&options, 5);

        assert_eq!(report.total_simulations, 5);
        assert_eq!(report.winner, "B");
        assert_eq!(report.distribution.get("B"), Some(&5));
    }
}
