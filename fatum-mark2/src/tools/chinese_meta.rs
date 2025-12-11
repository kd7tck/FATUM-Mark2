
pub const HEAVENLY_STEMS: [&str; 10] = [
    "Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui"
];

pub const EARTHLY_BRANCHES: [&str; 12] = [
    "Zi (Rat)", "Chou (Ox)", "Yin (Tiger)", "Mao (Rabbit)", "Chen (Dragon)", "Si (Snake)",
    "Wu (Horse)", "Wei (Goat)", "Shen (Monkey)", "You (Rooster)", "Xu (Dog)", "Hai (Pig)"
];

pub fn get_stem(idx: usize) -> &'static str {
    HEAVENLY_STEMS[idx % 10]
}

pub fn get_branch(idx: usize) -> &'static str {
    EARTHLY_BRANCHES[idx % 12]
}

/// Checks for the "Six Clashes" (Liu Chong).
/// Returns true if the two branches are antagonistic (opposite each other in the zodiac).
pub fn is_six_clash(b1_idx: usize, b2_idx: usize) -> bool {
    // Clashes are always 6 positions apart (e.g., 0 vs 6)
    (b1_idx as i32 - b2_idx as i32).abs() == 6
}

/// Checks for the "Six Combinations" (Liu He).
/// Returns true if the two branches form a harmonious union.
pub fn is_six_combination(b1_idx: usize, b2_idx: usize) -> bool {
    let pairs = [
        (0, 1),   // Rat + Ox
        (2, 11),  // Tiger + Pig
        (3, 10),  // Rabbit + Dog
        (4, 9),   // Dragon + Rooster
        (5, 8),   // Snake + Monkey
        (6, 7),   // Horse + Goat
    ];
    let min = b1_idx.min(b2_idx);
    let max = b1_idx.max(b2_idx);
    pairs.contains(&(min, max))
}

/// Returns the element associated with a Stem.
pub fn get_stem_element(idx: usize) -> &'static str {
    match idx % 10 {
        0 | 1 => "Wood",
        2 | 3 => "Fire",
        4 | 5 => "Earth",
        6 | 7 => "Metal",
        8 | 9 => "Water",
        _ => "Unknown"
    }
}
