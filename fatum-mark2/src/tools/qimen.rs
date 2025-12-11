use serde::{Deserialize, Serialize};
use crate::tools::astronomy::get_solar_term;

/// Represents a full Qi Men Dun Jia Chart (Hour School).
///
/// Contains the configuration of the Earth, Heaven, Door, and Deity plates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QiMenChart {
    pub time_label: String, // e.g. "Hour: Jia Zi"
    pub solar_term: String, // e.g. "Winter Solstice"
    pub dun_type: String, // "Yang Dun" or "Yin Dun"
    pub ju_number: i32, // The Bureau Number (1-9)
    pub duty_star: String, // Zhi Fu (The lead star)
    pub duty_door: String, // Zhi Shi (The lead door)
    pub palaces: Vec<QiMenPalace>,
}

/// A single sector (Palace) in the Qi Men grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QiMenPalace {
    pub index: usize, // 1-9
    pub position: String, // "SE", "S", etc.
    pub earth_plate: String, // Earth Stem (Di Pan)
    pub heaven_plate: String, // Heaven Stem (Tian Pan)
    pub door: String, // Men (Door)
    pub star: String, // Xing (Star)
    pub deity: String, // Shen (Deity)
    pub structure: String, // Special structures (e.g. "Green Dragon Returns")
}

// Stems: 0=Jia, 1=Yi, ... 9=Gui
// Branches: 0=Zi, ... 11=Hai

/// Calculates the Qi Men Dun Jia chart using the Chai Bu (Intersecting) Method.
///
/// This method relies on the Solar Term to determine the Yin/Yang nature and the Ju number,
/// but aligns the chart strictly to the specific hour pillar.
pub fn calculate_qimen(year: i32, month: u32, day: u32, hour: u32) -> QiMenChart {
    // 1. Determine Solar Term
    // The solar term dictates the Ju (Bureau) Number.
    let term_idx = get_solar_term(year, month, day); // 0-23
    let term_name = get_term_name(term_idx as usize);

    // 2. Determine Yin/Yang Dun and Ju Number
    // Calculate Day Stem/Branch to find the "Yuan" (Upper/Middle/Lower cycle).
    let (day_stem, _day_branch_idx) = get_gan_zhi_day(year, month, day);
    let (hour_stem, hour_branch) = get_gan_zhi_hour(day_stem, hour);

    let day_idx = get_day_gan_zhi_idx(year, month, day);
    // Cycle repeats every 15 days (5 Upper + 5 Middle + 5 Lower)
    let yuan_mod = day_idx % 15;
    let yuan = if yuan_mod < 5 { 0 } else if yuan_mod < 10 { 1 } else { 2 }; // 0=Upper, 1=Middle, 2=Lower

    let (dun_type, ju_num) = get_ju_number(term_idx as usize, yuan);

    // 3. Layout Earth Plate
    // Stems are placed in the palaces following the Ju number pattern.
    let earth_plate = layout_earth_plate(dun_type, ju_num);

    // 4. Find Duty Star (Zhi Fu) and Duty Door (Zhi Shi)
    // Determined by the Hour Stem location on the Earth Plate.
    let h_idx = get_gan_zhi_idx_hour(day_stem, hour);
    let palaces = generate_palaces(dun_type, ju_num, h_idx, &earth_plate);

    QiMenChart {
        time_label: format!("Hour: {} {}", hour_stem, hour_branch),
        solar_term: term_name.to_string(),
        dun_type: if dun_type { "Yang Dun".to_string() } else { "Yin Dun".to_string() },
        ju_number: ju_num,
        duty_star: palaces[0].star.clone(), // Simplified: Just taking first sector's star as representative
        duty_door: palaces[0].door.clone(),
        palaces,
    }
}

// === HELPERS ===

fn get_term_name(idx: usize) -> &'static str {
    let names = [
        "Little Cold", "Great Cold", "Start of Spring", "Rain Water", "Awakening of Insects", "Spring Equinox",
        "Pure Brightness", "Grain Rain", "Start of Summer", "Grain Full", "Grain in Ear", "Summer Solstice",
        "Minor Heat", "Major Heat", "Start of Autumn", "Limit of Heat", "White Dew", "Autumn Equinox",
        "Cold Dew", "Frost Descent", "Start of Winter", "Minor Snow", "Major Snow", "Winter Solstice"
    ];
    names[idx % 24]
}

/// Returns (Is_Yang, Ju_Number) based on Solar Term and Yuan.
fn get_ju_number(term: usize, yuan: usize) -> (bool, i32) {
    // Chai Bu constants map
    // Format: [Upper, Middle, Lower]

    // Winter Solstice to Summer Solstice is Yang.
    let is_yang = match term {
        22 | 23 | 0..=9 => true,
        _ => false,
    };

    // Specific Ju numbers for each solar term
    let nums = match term {
         0 => [2,8,5],  1 => [3,9,6],  2 => [8,5,2],  3 => [9,6,3],  4 => [1,7,4],  5 => [3,9,6],
         6 => [4,1,7],  7 => [5,2,8],  8 => [4,1,7],  9 => [5,2,8], 10 => [9,3,6], 11 => [9,3,6],
        12 => [8,2,5], 13 => [7,1,4], 14 => [2,5,8], 15 => [1,4,7], 16 => [9,3,6], 17 => [7,1,4],
        18 => [6,9,3], 19 => [5,8,2], 20 => [4,7,1], 21 => [3,6,9], 22 => [1,7,4], 23 => [2,8,5],
        _ => [1,1,1]
    };

    (is_yang, nums[yuan])
}

/// Arranges the Earth Plate Stems.
///
/// Pattern: Wu, Ji, Geng, Xin, Ren, Gui, Ding, Bing, Yi.
fn layout_earth_plate(yang: bool, ju: i32) -> [String; 9] {
    let mut plate = vec![""; 9];
    let stems = ["Wu", "Ji", "Geng", "Xin", "Ren", "Gui", "Ding", "Bing", "Yi"];

    let mut current_palace = ju;
    for stem in stems {
        let idx = (current_palace - 1) as usize;
        // Safety check for 1-9 range
        if idx < 9 {
             plate[idx] = stem;
        }

        // Yang moves forward (1->2...), Yin moves backward (9->8...)
        if yang {
            current_palace += 1;
            if current_palace > 9 { current_palace = 1; }
        } else {
            current_palace -= 1;
            if current_palace < 1 { current_palace = 9; }
        }
    }

    let mut arr = ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
    for i in 0..9 { arr[i] = plate[i].to_string(); }
    arr
}

/// Generates the full palace content (Heaven, Star, Door, Deity).
fn generate_palaces(yang: bool, _ju: i32, h_idx: usize, earth: &[String; 9]) -> Vec<QiMenPalace> {
    let doors = ["Rest", "Life", "Harm", "Du", "Jing", "Death", "Fear", "Open"];
    let stars = ["Peng", "Ren", "Chong", "Fu", "Ying", "Rui", "Zhu", "Xin", "Qin"];
    let deities = ["Chief", "Snake", "Moon", "Harmony", "Tiger", "Tortoise", "Phoenix", "Earth", "Heaven"];

    // Rotation logic based on Hour Stem index
    let shift = h_idx % 9;

    let mut palaces = Vec::new();
    let sectors = ["Kan (N)", "Kun (SW)", "Zhen (E)", "Xun (SE)", "Center", "Qian (NW)", "Dui (W)", "Gen (NE)", "Li (S)"];

    for i in 0..9 {
        let star_idx = (i + shift) % 9;
        let door_idx = (i + shift) % 8; // Doors skip center usually, simplified here
        let deity_idx = if yang { (i + (h_idx % 9)) % 8 } else { (i + 8 - (h_idx % 9)) % 8 };

        // Heaven stem rotates with the stars
        let heaven_stem = earth[(i + shift) % 9].clone();

        palaces.push(QiMenPalace {
            index: i + 1,
            position: sectors[i].to_string(),
            earth_plate: earth[i].clone(),
            heaven_plate: heaven_stem,
            door: doors[door_idx].to_string(),
            star: stars[star_idx].to_string(),
            deity: deities[deity_idx].to_string(),
            structure: "Normal".to_string(),
        });
    }

    palaces
}

// === DATE UTILS ===

fn get_day_gan_zhi_idx(y: i32, m: u32, d: u32) -> usize {
    let offset = (y * 365 + m as i32 * 30 + d as i32) as usize;
    offset % 60
}

fn get_gan_zhi_idx_hour(day_stem: &str, hour: u32) -> usize {
    let h_branch = (hour as usize + 1) / 2 % 12;
    let d_stem_idx = get_stem_idx(day_stem);
    let h_stem_idx = (d_stem_idx % 5 * 2 + h_branch) % 10;
    (h_stem_idx * 10 + h_branch) % 60
}

fn get_gan_zhi_day(y: i32, m: u32, d: u32) -> (&'static str, usize) {
    let stems = ["Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui"];
    let idx = get_day_gan_zhi_idx(y, m, d);
    let stem = stems[idx % 10];
    let branch = idx % 12;
    (stem, branch)
}

fn get_gan_zhi_hour(day_stem: &str, hour: u32) -> (&'static str, &'static str) {
    let stems = ["Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui"];
    let branches = ["Zi", "Chou", "Yin", "Mao", "Chen", "Si", "Wu", "Wei", "Shen", "You", "Xu", "Hai"];

    let h_branch_idx = (hour as usize + 1) / 2 % 12;
    let d_stem_idx = get_stem_idx(day_stem);
    let h_stem_idx = (d_stem_idx % 5 * 2 + h_branch_idx) % 10;

    (stems[h_stem_idx], branches[h_branch_idx])
}

fn get_stem_idx(s: &str) -> usize {
    match s {
        "Jia" => 0, "Yi" => 1, "Bing" => 2, "Ding" => 3, "Wu" => 4,
        "Ji" => 5, "Geng" => 6, "Xin" => 7, "Ren" => 8, "Gui" => 9, _ => 0
    }
}
