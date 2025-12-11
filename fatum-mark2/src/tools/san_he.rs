use serde::{Deserialize, Serialize};

/// Analysis report for the San He (Three Harmony) Water Method.
///
/// San He focuses on the relationship between the Mountain (Sitting), Water (Facing/Exit),
/// and the 12 Growth Phases of Qi.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanHeAnalysis {
    pub water_method: String, // e.g. "Double Mountain San He"
    pub growth_phase: String, // Current phase of the water exit (e.g. "Death", "Grave")
    pub lucky_water_exit: Vec<String>, // Recommended exit directions
}

/// Analyzes the environment using San He Logic.
///
/// Requires the Facing Degree (to determine the Sitting/Mountain) and optionally
/// the degree where water exits the property.
pub fn analyze_san_he(facing_deg: f64, _water_exit_deg: Option<f64>) -> SanHeAnalysis {
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
    // Yellow Springs (Huang Quan) are specific directions that are harmful to specific mountains.
    // Eight Killings (Ba Sha) are similar conflict points.
    let warnings = if sitting >= 337.5 || sitting < 22.5 {
        "Water Frame (North). Avoid Water exit at Dragon (SE) - Yellow Springs."
    } else if sitting >= 67.5 && sitting < 112.5 {
        "Wood Frame (East). Avoid Water exit at Goat (SW) - Yellow Springs."
    } else if sitting >= 157.5 && sitting < 202.5 {
        "Fire Frame (South). Avoid Water exit at Dog (NW) - Yellow Springs."
    } else if sitting >= 247.5 && sitting < 292.5 {
        "Metal Frame (West). Avoid Water exit at Ox (NE) - Yellow Springs."
    } else {
        "Mixed/Earth Frame. Check individual mountain affiliations."
    };

    SanHeAnalysis {
        water_method: "Double Mountain San He".to_string(),
        growth_phase: "Analysis Requires Topography (Water Exit Degree)".to_string(),
        lucky_water_exit: vec![warnings.to_string()],
    }
}
