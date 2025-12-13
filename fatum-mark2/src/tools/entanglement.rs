use serde::{Deserialize, Serialize};
use std::fmt::Write;
// use crate::tools::chinese_meta::{is_six_clash, is_six_combination, get_stem_element};

#[derive(Deserialize)]
pub struct EntanglementRequest {
    pub profile1_data: String, // e.g., JSON string or raw text
    pub profile2_data: String,
    pub mode: EntanglementMode,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum EntanglementMode {
    SeedHash,
    EntropyStream,
}

#[derive(Serialize)]
pub struct EntanglementReport {
    pub mode: String,
    pub resonance_score: f64, // 0.0 to 1.0 (or higher)
    pub compatibility_factors: Vec<String>,
    pub narrative: String,
    pub shared_hexagram: Option<u8>, // 1-64
}

pub fn calculate_entanglement(req: &EntanglementRequest) -> anyhow::Result<EntanglementReport> {
    match req.mode {
        EntanglementMode::SeedHash => calculate_seed_hash(req),
        EntanglementMode::EntropyStream => calculate_entropy_stream(req),
    }
}

#[cfg(test)]
#[path = "entanglement_tests.rs"]
mod tests;

// === MODE A: SEED HASH (Deterministic) ===
// Combines birth data to form a seed, then derives compatibility.
// This is like "Synastry" where the chart interaction is fixed, but we add a crypto-flavor.
fn calculate_seed_hash(req: &EntanglementRequest) -> anyhow::Result<EntanglementReport> {
    use sha2::{Sha256, Digest};

    // 1. Concatenate Data
    let combined = format!("{}{}", req.profile1_data, req.profile2_data);

    // 2. Hash
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize(); // 32 bytes

    // 3. Extract Metrics
    // Byte 0: Base Resonance (0-255)
    // Byte 1: Karma Link (0-255)
    // Byte 2: Friction (0-255)
    // Byte 3: Hexagram (0-63)

    let base_res = result[0] as f64 / 2.55; // 0-100
    let karma = result[1] as f64 / 2.55;
    let friction = result[2] as f64 / 2.55;
    let hex_idx = (result[3] % 64) + 1;

    // Adjust score based on "Metaphysical" logic if we can parse the profiles
    // For MVP, we assume profiles are passed as JSON strings that might contain {birth_year, etc.}
    // But since the request just takes strings to be generic, we'll try to parse them if possible.
    // If parsing fails, we rely purely on the hash.

    let mut factors = Vec::new();
    factors.push(format!("Base Resonance: {:.1}%", base_res));
    factors.push(format!("Karmic Link: {:.1}%", karma));
    factors.push(format!("Friction Potential: {:.1}%", friction));

    // Calculate final score
    let score = (base_res + karma - (friction * 0.5)).clamp(0.0, 100.0);

    let mut narrative = String::new();
    write!(narrative, "Deterministic Seed Analysis complete. The combined waveform of these two entities generates a stable resonance pattern. ")?;
    if score > 80.0 {
        write!(narrative, "Extremely high compatibility detected. A 'Soul Bond' configuration.")?;
    } else if score > 50.0 {
        write!(narrative, "Moderate compatibility. Requires conscious energy management to maintain coherence.")?;
    } else {
        write!(narrative, "Low resonance. High potential for destructive interference.")?;
    }

    Ok(EntanglementReport {
        mode: "Seed Hash".to_string(),
        resonance_score: score,
        compatibility_factors: factors,
        narrative,
        shared_hexagram: Some(hex_idx),
    })
}

// === MODE B: ENTROPY STREAM (Probabilistic) ===
// Fetches entropy and simulates how two entities 'ride the wave' together.
// Does their luck correlate?
fn calculate_entropy_stream(req: &EntanglementRequest) -> anyhow::Result<EntanglementReport> {
    // For simulation, we ideally need the actual BaZi charts to see if they like the same elements.
    // Since we don't have full BaZi logic exposed easily here without full profile parsing,
    // we will simulate "Abstract Resonance" using the hash of their data as a "seed" for their
    // individual reaction functions.

    // 1. Derive a "Reaction Seed" for each profile
    let seed1 = derive_reaction_seed(&req.profile1_data);
    let seed2 = derive_reaction_seed(&req.profile2_data);

    // 2. Simulate 100 "Time Steps" of Entropy
    // In a real scenario, we'd fetch from CURBy. Here we use a local RNG seeded by system time for the "Stream"
    // to simulate a live flux if we don't have a batch passed.
    // (Ideally the controller passes entropy, but for this tool we'll self-generate for now).

    let mut rng = rand::thread_rng();
    use rand::Rng;

    let mut correlation_sum: f64 = 0.0;

    for _ in 0..100 {
        // "Event" is a value -1.0 to 1.0 representing some energy shift
        let event_val: f64 = rng.gen_range(-1.0..1.0);

        // Entity Reaction: sin(seed * event)
        // This is a pseudo-scientific placeholder for "how this person reacts to this energy"
        // Cast u64 seed to f64 for sin calc.
        let r1 = ((seed1 as f64) * event_val).sin();
        let r2 = ((seed2 as f64) * event_val).sin();

        // If signs match, they are in sync.
        if r1.signum() == r2.signum() {
            correlation_sum += 1.0;
        } else {
            correlation_sum -= 0.5; // Penalty for discord
        }
    }

    // Normalize
    let score = correlation_sum.clamp(0.0, 100.0);

    let factors = vec![
        format!("Quantum Synchronization: {:.1}%", score),
        "Simulated 100 Entropy Events".to_string()
    ];

    let mut narrative = String::new();
    write!(narrative, "Entropy Stream Simulation complete. ")?;
    if score > 70.0 {
        write!(narrative, "Entities exhibit 'Phase Locking'. They tend to react similarly to external chaotic stimuli.")?;
    } else {
        write!(narrative, "Entities are 'Phase Shifted'. External pressure often drives them in opposite emotional directions.")?;
    }

    Ok(EntanglementReport {
        mode: "Entropy Stream".to_string(),
        resonance_score: score,
        compatibility_factors: factors,
        narrative,
        shared_hexagram: None,
    })
}

fn derive_reaction_seed(data: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut s = DefaultHasher::new();
    data.hash(&mut s);
    s.finish()
}
