use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QiMenChart {
    pub hour_stem: String,
    pub day_stem: String,
    pub deity: String, // "Chief", "Snake", etc.
    pub door: String,  // "Open", "Rest", etc.
    pub star: String,  // "Grass", "Grain", etc.
    pub structure: String, // "Green Dragon Returns", etc.
}

pub fn calculate_qimen(year: i32, month: u32, day: u32, hour: u32) -> QiMenChart {
    // Full QMDJ is extremely complex.
    // We will simulate a result based on "Hour" offset for demonstration of the "structure".
    // In a real implementation, this would require calculating the Ju number (Yin/Yang),
    // then mapping the plate.

    let hour_idx = hour / 2;
    let stems = ["Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui"];
    let doors = ["Open (Kai)", "Rest (Xiu)", "Life (Sheng)", "Harm (Shang)", "Du (Delusion)", "Jing (Scenery)", "Si (Death)", "Jing (Fear)"];
    let deities = ["Chief", "Snake", "Moon", "Harmony", "Tiger", "Tortoise", "Phoenix", "Earth", "Heaven"]; // Roughly

    // Pseudo-random deterministic map
    let seed = year as u32 + month + day + hour;
    let door = doors[(seed % 8) as usize].to_string();
    let deity = deities[(seed % 9) as usize].to_string();
    let structure = if door.contains("Life") || door.contains("Open") { "Auspicious Structure" } else { "Average Structure" };

    QiMenChart {
        hour_stem: stems[(hour_idx as usize) % 10].to_string(),
        day_stem: "Calc Req".to_string(),
        deity,
        door,
        star: "Heavenly Star".to_string(),
        structure: structure.to_string(),
    }
}
