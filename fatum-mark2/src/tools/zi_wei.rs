use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ZiWeiConfig {
    pub birth_year: i32,
    pub birth_month: u32,
    pub birth_day: u32,
    pub birth_hour: u32,
    pub gender: String, // "M" or "F"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZiWeiChart {
    pub palaces: Vec<Palace>,
    pub life_palace_idx: usize,
    pub body_palace_idx: usize,
    pub element_phase: String, // Five Element Phase (Water 2, Wood 3, etc.)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Palace {
    pub index: usize, // 0..11 (0=Zi/Rat, 1=Chou/Ox...)
    pub branch_name: String, // "Zi", "Chou"
    pub name: String, // "Life", "Siblings", etc.
    pub major_stars: Vec<String>,
    pub minor_stars: Vec<String>,
}

pub const PALACE_NAMES: [&str; 12] = [
    "Life", "Siblings", "Spouse", "Children",
    "Wealth", "Health", "Travel", "Friends",
    "Career", "Property", "Mental", "Parents"
];

// Branches in Zi Wei usually start from Zi (Rat) at a specific position?
// Standard chart layout:
// Si(Snake)  Wu(Horse)  Wei(Goat)  Shen(Monkey) -> Top Row
// Chen(Drag)                       You(Rooster)
// Mao(Rabbit)                      Xu(Dog)
// Yin(Tiger) Chou(Ox)   Zi(Rat)    Hai(Pig)     -> Bottom Row
// Wait, the standard grid is:
// SE (Si) | S (Wu) | SW (Wei) | W (Shen)
// ...
// Actually, it's usually 12 squares around the perimeter.
// Zi (Rat) is usually North (Bottom Center in Feng Shui, but depends on chart orientation).
// In Zi Wei software, it's a 3x4 grid?
// No, it's a 4x4 grid with center empty (or used for info).
//  Si  Wu  Wei Shen
// Chen         You
// Mao          Xu
// Yin Chou  Zi Hai
// (Clockwise from bottom-left corner Yin?)
// Let's use Index 0 = Yin (Tiger) as standard start for "January"?
// No, let's stick to 0=Zi (Rat) for internal consistency with other tools.

pub fn generate_ziwei_chart(config: ZiWeiConfig) -> Result<ZiWeiChart, String> {
    // 1. Determine Life and Body Palace Positions
    // Life Palace Formula: Start at Yin (Index 2). Move Clockwise by Month, Counter-Clockwise by Hour.
    // Body Palace Formula: Start at Yin (Index 2). Move Clockwise by Month, Clockwise by Hour.
    // Note: Month here refers to Lunar Month.
    // We will use Solar Month index from 1..12 as approximation if strictly needed,
    // or assume input `birth_month` is the Lunar Month (1-12).
    // Let's assume input is Lunar Month for the algorithm to work "correctly" logically,
    // even if the user provides Gregorian. (A full conversion is out of scope).

    // Convert Hour (0-23) to Branch Index (0-11).
    // Rat (Zi) is 23:00-01:00. Index 0.
    // Hour 0 -> Index 0. Hour 1 -> Index 1 (Chou)? No.
    // 23-01 = Zi (0). 01-03 = Chou (1).
    // Formula: (h + 1) / 2 % 12.
    let hour_idx = ((config.birth_hour + 1) / 2) % 12;

    let month_num = config.birth_month as i32; // 1-12
    let hour_num = hour_idx as i32; // 0=Rat, 1=Ox...

    // Yin is Index 2 in our global system (0=Rat, 1=Ox, 2=Tiger).
    // Algorithm:
    // Start at Yin (2).
    // Add (Month - 1). (Month 1 = Tiger, stays at Tiger).
    // Subtract (Hour - 1)?
    // Standard Formula: Life = Month - Hour + 1 (in generic 'Branch steps' relative to Yin?)
    // Let's trace:
    // Born Month 1 (Tiger), Hour Zi (Rat, 0).
    // Start Yin (2). + (1-1) = 2. - (0-1)?
    // Let's check rule: "Start at Yin. Count Month clockwise. Then count Hour counter-clockwise."
    // Yin=2.
    // Month=1 -> Count 1 (Stay at 2).
    // Hour=Zi (1st branch) -> Count 1 backwards (Stay at 2).
    // Wait, usually counting is inclusive.
    // Step 1: Base = Yin + (Month - 1).
    // Step 2: Life = Base - (HourIdx). (Since Zi=0 is '1st' hour branch?)

    // Let's use indices 0..11.
    // Base = (2 + (month_num - 1)).
    // Life = (Base - hour_num).
    // If Life < 0, add 12.

    let base = 2 + (month_num - 1);
    let life_idx_raw = base - hour_num;
    let life_idx = life_idx_raw.rem_euclid(12) as usize;

    // Body Palace:
    // Start at Yin (2). Count Month clockwise. Count Hour clockwise.
    // Base = (2 + (month_num - 1)).
    // Body = Base + hour_num?
    // Wait, usually Body is Month + Hour - something.
    // Rule: "Start at Yin. Month clockwise. Hour clockwise."
    // Body = (2 + (month_num - 1) + hour_num) % 12.

    // Note: hour_num here needs to be 0 for Rat?
    // In Zi Wei, Rat is the 1st hour? Or 11th?
    // Usually Zi is associated with Index 1 in counting sequences (1..12).
    // If Zi=0 index...
    // Let's verify with an example.
    // Month 1, Hour Zi. Life should be at Yin (2).
    // Formula: 2 + 0 - 0 = 2. Correct.
    // Month 1, Hour Chou (1). Life should be Chou (1).
    // Formula: 2 + 0 - 1 = 1. Correct.
    // Month 2 (Mao), Hour Zi. Life should be Mao (3).
    // Formula: 2 + 1 - 0 = 3. Correct.

    let body_idx = (2 + (month_num - 1) + hour_num).rem_euclid(12) as usize;

    // 2. Assign Palaces
    // The Life Palace is the anchor. The other 11 palaces follow Counter-Clockwise.
    // Sequence: Life, Siblings, Spouse, Children, Wealth, Health, Travel, Friends, Career, Property, Mental, Parents.

    let mut palaces = Vec::new();
    for i in 0..12 {
        // Current Palace Position in the rotation
        // Life is at `life_idx`.
        // Next (Siblings) is at `life_idx - 1` (Counter-Clockwise).
        let p_idx = (life_idx as i32 - i as i32).rem_euclid(12) as usize;

        let mut major = Vec::new();
        // Placeholder for stars
        if p_idx == life_idx { major.push("Emperor (Zi Wei)".to_string()); }

        palaces.push(Palace {
            index: p_idx,
            branch_name: crate::tools::chinese_meta::get_branch(p_idx).to_string(),
            name: PALACE_NAMES[i].to_string(),
            major_stars: major,
            minor_stars: Vec::new(),
        });
    }

    // Sort palaces by index 0..11 for easier rendering in a grid
    palaces.sort_by_key(|p| p.index);

    Ok(ZiWeiChart {
        palaces,
        life_palace_idx: life_idx,
        body_palace_idx: body_idx,
        element_phase: "Wood 3 (Mock)".to_string(),
    })
}
