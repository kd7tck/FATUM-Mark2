use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanHeAnalysis {
    pub water_method: String, // "Yellow Springs", "Eight Killings", etc.
    pub growth_phase: String, // "Birth", "Bath", "Prosperity", etc.
    pub lucky_water_exit: Vec<String>,
}

pub fn analyze_san_he(facing_deg: f64, water_exit_deg: Option<f64>) -> SanHeAnalysis {
    // 24 Mountains for San He
    // Simplified Logic: Determine "Frame" based on Facing (Water Frame, Wood Frame, etc.)
    // Then check Growth Phases.

    // Frame Determination (Double Mountain)
    // Water Frame: Shen-Zi-Chen (Monkey-Rat-Dragon) -> North
    // Wood Frame: Hai-Mao-Wei (Pig-Rabbit-Goat) -> East
    // Fire Frame: Yin-Wu-Xu (Tiger-Horse-Dog) -> South
    // Metal Frame: Si-You-Chou (Snake-Rooster-Ox) -> West

    // Map facing to Frame?
    // Usually Sitting determines the Mountain (Dragon).
    // Let's assume Sitting = Facing + 180.
    let sitting = (facing_deg + 180.0) % 360.0;

    // Simplified: Check typical "Killing Forces"
    let warnings = if (sitting >= 337.5 || sitting < 22.5) {
        "Water Frame (North). Avoid Water exit at Dragon (SE)."
    } else if (sitting >= 67.5 && sitting < 112.5) {
        "Wood Frame (East). Avoid Water exit at Goat (SW)."
    } else if (sitting >= 157.5 && sitting < 202.5) {
        "Fire Frame (South). Avoid Water exit at Dog (NW)."
    } else if (sitting >= 247.5 && sitting < 292.5) {
        "Metal Frame (West). Avoid Water exit at Ox (NE)."
    } else {
        "Mixed/Earth Frame."
    };

    SanHeAnalysis {
        water_method: "Double Mountain San He".to_string(),
        growth_phase: "Analysis Requires Topography".to_string(),
        lucky_water_exit: vec![warnings.to_string()],
    }
}
