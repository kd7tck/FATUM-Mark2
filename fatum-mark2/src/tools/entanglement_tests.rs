#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::entanglement::{EntanglementRequest, EntanglementMode, calculate_entanglement};

    #[test]
    fn test_seed_hash_determinism() {
        let req1 = EntanglementRequest {
            profile1_data: "UserA".to_string(),
            profile2_data: "UserB".to_string(),
            mode: EntanglementMode::SeedHash,
        };

        let report1 = calculate_entanglement(&req1).unwrap();

        // Run again
        let req2 = EntanglementRequest {
            profile1_data: "UserA".to_string(),
            profile2_data: "UserB".to_string(),
            mode: EntanglementMode::SeedHash,
        };
        let report2 = calculate_entanglement(&req2).unwrap();

        assert_eq!(report1.resonance_score, report2.resonance_score);
        assert_eq!(report1.compatibility_factors.len(), 3);
    }

    #[test]
    fn test_seed_hash_commutativity_check() {
        // Hash(A+B) is NOT same as Hash(B+A) usually.
        // Synastry is directional (A to B vs B to A).
        // Let's verify they are different.

        let req1 = EntanglementRequest {
            profile1_data: "UserA".to_string(),
            profile2_data: "UserB".to_string(),
            mode: EntanglementMode::SeedHash,
        };
        let r1 = calculate_entanglement(&req1).unwrap();

        let req2 = EntanglementRequest {
            profile1_data: "UserB".to_string(),
            profile2_data: "UserA".to_string(),
            mode: EntanglementMode::SeedHash,
        };
        let r2 = calculate_entanglement(&req2).unwrap();

        assert_ne!(r1.resonance_score, r2.resonance_score);
    }

    #[test]
    fn test_entropy_stream_runs() {
        let req = EntanglementRequest {
            profile1_data: "UserA".to_string(),
            profile2_data: "UserB".to_string(),
            mode: EntanglementMode::EntropyStream,
        };
        let r = calculate_entanglement(&req).unwrap();
        // Just check it returns a score 0-100
        assert!(r.resonance_score >= 0.0 && r.resonance_score <= 100.0);
    }
}
