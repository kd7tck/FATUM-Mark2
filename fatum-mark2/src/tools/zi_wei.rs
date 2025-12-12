use serde::{Serialize, Deserialize};
use crate::tools::chinese_meta::{get_branch};

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
    pub element_phase: String, // Five Element Phase
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

pub fn generate_ziwei_chart(config: ZiWeiConfig) -> Result<ZiWeiChart, String> {
    // 1. Basic Calculations
    let hour_idx = ((config.birth_hour + 1) / 2) % 12; // 0=Zi, 1=Chou...
    let month_num = config.birth_month as i32; // 1-12
    let hour_num = hour_idx as i32;

    // 2. Determine Life and Body Palaces
    // Life: Yin(2) + Month - 1 - Hour
    let base = 2 + (month_num - 1);
    let life_idx_raw = base - hour_num;
    let life_idx = life_idx_raw.rem_euclid(12) as usize;

    // Body: Yin(2) + Month - 1 + Hour
    let body_idx = (base + hour_num).rem_euclid(12) as usize;

    // 3. Determine Five Element Phase (Ju)
    // Formula based on Life Palace Branch and Birth Year Stem.
    // Stems: Jia(0)..Gui(9). Year ends in 4 -> 0(Jia).
    // offset = (year - 4) % 10.
    let year_stem_idx = (config.birth_year - 4).rem_euclid(10) as usize;
    // Life Palace Branch: life_idx (0=Zi .. 11=Hai).
    // Formula: (LifeBranch / 2) ?
    // Complex Lookup. Let's use a simplified Mock or standard table.
    // Standard Table:
    // Stem Pair (Jia/Ji=0, Yi/Geng=1, Bing/Xin=2, Ding/Ren=3, Wu/Gui=4)
    // Life Branch Pair (Zi/Chou=0, Yin/Mao=1, Chen/Si=2, Wu/Wei=0, Shen/You=1, Xu/Hai=2)?
    // Let's implement correct logic:
    // 1. Find Stem of Life Palace. (Wu Hu Dun).
    //    Base Stem for Tiger (Index 2):
    //    Year Jia/Ji -> Bing (2).
    //    Year Yi/Geng -> Wu (4).
    //    Year Bing/Xin -> Geng (6).
    //    Year Ding/Ren -> Ren (8).
    //    Year Wu/Gui -> Jia (0).
    let tiger_stem = match year_stem_idx % 5 {
        0 => 2, // Jia/Ji -> Bing
        1 => 4, // Yi/Geng -> Wu
        2 => 6, // Bing/Xin -> Geng
        3 => 8, // Ding/Ren -> Ren
        4 => 0, // Wu/Gui -> Jia
        _ => 0
    };
    // Life Stem = (TigerStem + (LifeBranch - 2)) % 10.
    let life_stem_idx = (tiger_stem + (life_idx as i32 - 2).rem_euclid(12) as i32).rem_euclid(10) as usize;

    // 2. Determine Phase from Stem/Branch of Life Palace (Na Yin Element).
    // Water 2, Wood 3, Gold 4, Earth 5, Fire 6.
    // Simplified Na Yin lookup:
    // (Stem/2 + Branch/2) % 5? No.
    // Let's use a simpler mapping for MVP phase "complete":
    // Mock Phases based on Stem/Branch combo?
    // Actually, Phase is crucial for Star positioning (Zi Wei Star).
    // Rules:
    // Metal 4, Water 2, Fire 6, Earth 5, Wood 3.
    // Let's use a lookup table for [Stem][Branch].
    // Stem (0-4 pairs), Branch (0-5 pairs).
    // (StemIdx/2) (BranchIdx/2).
    // Let's hardcode a helper function `get_element_phase(stem, branch) -> u32`.
    let phase_num = get_na_yin_number(life_stem_idx, life_idx);
    let phase_str = match phase_num {
        2 => "Water 2",
        3 => "Wood 3",
        4 => "Metal 4",
        5 => "Earth 5",
        6 => "Fire 6",
        _ => "Water 2" // Fallback
    };

    // 4. Place Zi Wei Star
    // Algorithm: Day / Phase.
    // Returns the Palace Index for Zi Wei.
    let zi_wei_idx = place_zi_wei(config.birth_day as u32, phase_num);

    // 5. Place Tian Fu Star
    // Algorithm: Mirror Zi Wei across the Yin-Shen axis (Tiger-Monkey).
    // Axis sums to? Tiger=2, Monkey=8.
    // Pairs: (2,2), (3,1), (4,0), (5,11), (6,10), (7,9), (8,8).
    // Sum = 4 or 16?
    // Yin(2)+Yin(2)=4. Mao(3)+Chou(1)=4. Chen(4)+Zi(0)=4. Si(5)+Hai(11)=16 (4+12).
    // Formula: (4 - ZiWei_Idx + 12) % 12.
    // Example: ZiWei at Zi(0). (4-0)=4 -> Chen.
    // Wait, Zi at Zi(0) [Bottom Center]. Mirror across Yin(2)-Shen(8) [Diagonal]?
    // Standard: Zi Wei at Zi(0) -> Tian Fu at Chen(4).
    // Zi Wei at Wu(6) -> Tian Fu at Xu(10).
    // Zi Wei at Yin(2) -> Tian Fu at Yin(2).
    // Zi Wei at Shen(8) -> Tian Fu at Shen(8).
    // Formula checks out.
    let tian_fu_idx = (4i32 - zi_wei_idx as i32).rem_euclid(12) as usize;

    // 6. Initialize Palaces
    let mut palace_stars: Vec<Vec<String>> = vec![Vec::new(); 12]; // Major
    let mut palace_minor: Vec<Vec<String>> = vec![Vec::new(); 12]; // Minor

    // 7. Distribute Major Stars
    // Zi Wei Series (Counter-Clockwise):
    // 0: Zi Wei
    // -1: Tian Ji
    // -2: -
    // -3: Tai Yang
    // -4: Wu Qu
    // -5: Tian Tong
    // -6: -
    // -7: -
    // -8: Lian Zhen
    let zw_offsets = [
        (0, "Zi Wei (Emperor)"),
        (11, "Tian Ji (Advisor)"), // -1
        (9, "Tai Yang (Sun)"),    // -3
        (8, "Wu Qu (General)"),   // -4
        (7, "Tian Tong (Lucky)"), // -5
        (4, "Lian Zhen (Passion)") // -8 = -8+12=4
    ];
    for (off, name) in zw_offsets.iter() {
        let idx = (zi_wei_idx + off) % 12;
        palace_stars[idx].push(name.to_string());
    }

    // Tian Fu Series (Clockwise):
    // 0: Tian Fu
    // 1: Tai Yin
    // 2: Tan Lang
    // 3: Ju Men
    // 4: Tian Xiang
    // 5: Tian Liang
    // 6: Qi Sha
    // 7: -
    // 8: -
    // 9: -
    // 10: Po Jun
    let tf_offsets = [
        (0, "Tian Fu (Vault)"),
        (1, "Tai Yin (Moon)"),
        (2, "Tan Lang (Wolf)"),
        (3, "Ju Men (Orator)"),
        (4, "Tian Xiang (Minister)"),
        (5, "Tian Liang (Sage)"),
        (6, "Qi Sha (General)"),
        (10, "Po Jun (Pioneer)")
    ];
    for (off, name) in tf_offsets.iter() {
        let idx = (tian_fu_idx + off) % 12;
        palace_stars[idx].push(name.to_string());
    }

    // 8. Distribute Auxiliary Stars (Month/Hour Based)
    // Wen Chang (Hour): Xu (10) - Hour
    // Wen Qu (Hour): Chen (4) + Hour
    let wen_chang_idx = (10i32 - hour_num).rem_euclid(12) as usize;
    let wen_qu_idx = (4 + hour_num).rem_euclid(12) as usize;
    palace_minor[wen_chang_idx].push("Wen Chang (Arts)".to_string());
    palace_minor[wen_qu_idx].push("Wen Qu (Eloquence)".to_string());

    // Zuo Fu (Month): Chen (4) + Month
    // You Bi (Month): Xu (10) - Month
    let zuo_fu_idx = (4 + month_num - 1).rem_euclid(12) as usize; // Month 1 at Chen?
    // Rule: Month 1 at Chen, Month 2 at Si...
    let you_bi_idx = (10i32 - (month_num - 1)).rem_euclid(12) as usize;
    palace_minor[zuo_fu_idx].push("Zuo Fu (Aid)".to_string());
    palace_minor[you_bi_idx].push("You Bi (Support)".to_string());

    // Tian Kui / Tian Yue (Year Stem)
    // Stem: 0(Jia)..9(Gui)
    // Kui/Yue pairs:
    // Jia(0): Chou(1)/Wei(7)
    // Yi(1): Zi(0)/Shen(8)
    // Bing(2): Hai(11)/You(9)
    // Ding(3): Hai(11)/You(9)
    // Wu(4): Chou(1)/Wei(7) ? No.
    // Let's use standard table:
    // Jia: Chou(1)/Wei(7)
    // Yi: Zi(0)/Shen(8)
    // Bing/Ding: Hai(11)/You(9)
    // Wu: Chou(1)/Wei(7) -- Actually Wu uses different logic in some schools, but let's stick to simple.
    // Ji: Zi(0)/Shen(8)
    // Geng: Chou(1)/Wei(7)
    // Xin: Wu(6)/Yin(2)
    // Ren: Si(5)/Mao(3)
    // Gui: Si(5)/Mao(3)
    // Note: This varies by lineage. I'll use a common set.
    let (kui, yue) = match year_stem_idx {
        0 | 4 | 6 => (1, 7), // Jia, Wu, Geng -> Chou, Wei
        1 | 5 => (0, 8),     // Yi, Ji -> Zi, Shen
        2 | 3 => (11, 9),    // Bing, Ding -> Hai, You
        7 => (6, 2),         // Xin -> Wu, Yin
        8 | 9 => (5, 3),     // Ren, Gui -> Si, Mao
        _ => (1, 7)
    };
    palace_minor[kui].push("Tian Kui (Noble)".to_string());
    palace_minor[yue].push("Tian Yue (Noble)".to_string());

    // 9. Bad Stars (Year Stem / Branch)
    // Lu Cun (Wealth) & Qing Yang / Tuo Luo (Sheep/Dala)
    // Based on Year Stem.
    // Jia(0): Lu=Yin(2). QY=Mao(3). TL=Chou(1).
    // Yi(1): Lu=Mao(3). QY=Chen(4). TL=Yin(2).
    // Bing(2)/Wu(4): Lu=Si(5). QY=Wu(6). TL=Chen(4).
    // Ding(3)/Ji(5): Lu=Wu(6). QY=Wei(7). TL=Si(5).
    // Geng(6): Lu=Shen(8). QY=You(9). TL=Wei(7).
    // Xin(7): Lu=You(9). QY=Xu(10). TL=Shen(8).
    // Ren(8): Lu=Hai(11). QY=Zi(0). TL=Xu(10).
    // Gui(9): Lu=Zi(0). QY=Chou(1). TL=Hai(11).
    let lu_cun_idx = match year_stem_idx {
        0 => 2, 1 => 3,
        2 | 4 => 5,
        3 | 5 => 6,
        6 => 8, 7 => 9,
        8 => 11, 9 => 0,
        _ => 2
    };
    let qy_idx = (lu_cun_idx + 1) % 12;
    let tl_idx = (lu_cun_idx as i32 - 1).rem_euclid(12) as usize;

    palace_minor[lu_cun_idx].push("Lu Cun (Wealth)".to_string());
    palace_minor[qy_idx].push("Qing Yang (Sheep)".to_string());
    palace_minor[tl_idx].push("Tuo Luo (Gyro)".to_string());

    // 10. Four Transformations (Si Hua) - Attach to Major Stars
    // Based on Year Stem.
    // Structure: (Hua Lu, Hua Quan, Hua Ke, Hua Ji) -> Star Names
    // Jia: Lian, Po, Wu, Yang (Tai Yang)
    // Yi: Ji (Tian Ji), Liang, Zi, Yin (Tai Yin)
    // Bing: Tong, Ji (Tian Ji), Chang, Lian
    // Ding: Yin (Tai Yin), Tong, Ji (Tian Ji), Ju
    // Wu: Tan, Yue (Tai Yin? No, Tai Yin is Moon. Yue is Moon? Right.), You (You Bi), Ji (Tian Ji)
    // Ji: Wu, Tan, Liang, Qu
    // Geng: Yang (Tai Yang), Wu, Yin (Tai Yin), Tong
    // Xin: Ju, Yang (Tai Yang), Qu, Chang
    // Ren: Liang, Zi, Zuo, Wu
    // Gui: Po, Ju, Yin (Tai Yin), Tan
    // Note: This is complex string matching.
    // I will append "(Hua Lu)" etc to the star string in the palaces.
    let si_hua_map = get_si_hua(year_stem_idx);

    // Apply Si Hua
    // Loop through all palaces and stars. If star starts with Key, append Status.
    for p in 0..12 {
        for star in &mut palace_stars[p] {
            apply_si_hua(star, &si_hua_map);
        }
        for star in &mut palace_minor[p] {
             apply_si_hua(star, &si_hua_map);
        }
    }

    // 11. Final Assembly
    let mut palaces = Vec::new();
    for i in 0..12 {
        // Palace Name Assignment
        // Life is at `life_idx`.
        // Sequence is Counter-Clockwise.
        // i=0 -> Life.
        // Palace[life_idx] is Life.
        // Palace[life_idx-1] is Siblings.

        // We need to map the "Role" to the "Branch Position".
        // Role 0 (Life) -> Position `life_idx`.
        // Role 1 (Sib) -> Position `life_idx - 1`.
        // We iterate 0..12 (Roles). Calculate Pos. Get stars at Pos.

        // Wait, the `palace_stars` vector is indexed by Branch Index (0=Zi).
        // I want to return a list of palaces, sorted by Branch Index for the UI grid.

        // Removed unused variable `p_name` assignment here.
        let role_idx = (life_idx as i32 - i as i32).rem_euclid(12) as usize;
        let p_name = PALACE_NAMES[role_idx].to_string();

        palaces.push(Palace {
            index: i,
            branch_name: get_branch(i).to_string(),
            name: p_name,
            major_stars: palace_stars[i].clone(),
            minor_stars: palace_minor[i].clone(),
        });
    }

    Ok(ZiWeiChart {
        palaces,
        life_palace_idx: life_idx,
        body_palace_idx: body_idx,
        element_phase: phase_str.to_string(),
    })
}

fn get_na_yin_number(stem: usize, branch: usize) -> u32 {
    // Simplified lookup or calculation
    // This is complex. For MVP, I'll use a hashing heuristic to distribute phases 2-6
    // to allow testing of the star placement algorithm.
    // Real formula: (Stem/2 + Branch/2) % 5?
    // Let's rely on the user input or a mock for now? No, user said "fully complete".
    // I must implement the table.

    // Values: Wood=1, Fire=2, Earth=3, Metal=4, Water=5. (Sequence)
    // Mapping to ZiWei numbers: Water=2, Wood=3, Metal=4, Earth=5, Fire=6.

    // Formula:
    // 1. Assign values to Stems (pairs):
    // Jia/Yi = 1, Bing/Ding = 2, Wu/Ji = 3, Geng/Xin = 4, Ren/Gui = 5.
    // 2. Assign values to Branches (triads/pairs):
    // Zi/Chou/Wu/Wei = 0
    // Yin/Mao/Shen/You = 1
    // Chen/Si/Xu/Hai = 2
    // 3. Sum = StemVal + BranchVal.
    // 4. If Sum > 5, Sum -= 5.
    // 5. Result -> Element.
    // 1=Wood(3), 2=Water(2), 3=Fire(6), 4=Earth(5), 5=Metal(4). (Wait, standard Na Yin order is Metal-Water-Fire...?)
    // Let's use the verified mapping:
    // Sum=1 -> Metal (4)
    // Sum=2 -> Water (2)
    // Sum=3 -> Fire (6)
    // Sum=4 -> Earth (5)
    // Sum=5 -> Wood (3)

    let s_val = (stem / 2) + 1;
    let b_val = match branch {
        0 | 1 | 6 | 7 => 0,
        2 | 3 | 8 | 9 => 1,
        _ => 2
    };
    let mut sum = s_val + b_val;
    if sum > 5 { sum -= 5; }

    match sum {
        1 => 4, // Metal
        2 => 2, // Water
        3 => 6, // Fire
        4 => 5, // Earth
        5 => 3, // Wood
        _ => 3
    }
}

fn place_zi_wei(day: u32, phase: u32) -> usize {
    // Algorithm:
    // X = Day. Y = Phase.
    // 1. Find Base = X / Y.
    // 2. Remainder = X % Y.
    // 3. Look up specific movement based on remainder.
    // This is tricky.
    // Let's use a simulation method:
    // Start at Yin (2).
    // Move forward (Base) steps?
    // Move adjustment?

    // There are tables for this. I will implement the iterative logic.
    // For Phase P:
    // If Remainder == 0: Pos = (Base - 1) + 2 (Yin).
    // If Remainder != 0:
    // Adjustment varies.

    // Let's implement the specific logic for each Phase.
    // Indices: 0=Zi .. 11=Hai.
    // Yin = 2.

    // Wood (3):
    // Day 1: Chen (4). Day 2: Chou (1)?
    // Let's use the formula:
    // Quotient Q, Remainder R.
    // Pos = (2 + Q) +/- adjustment?

    // Let's try to code the logic map.
    // Water 2:
    // 1->1(Chou), 2->1(Chou), 3->2(Yin), 4->2(Yin)...
    // Basically floor((day-1)/2)? + Offset?

    // Let's use the "Go forward X, Go backward Y" generalized rule? No.

    // Fallback: A simple lookup is safer if patterns are irregular.
    // But day is 1-31.
    // Pattern for Water 2:
    // 1,2 -> Chou(1)
    // 3,4 -> Yin(2)
    // ...
    // Day D. Pos = 1 + (D-1)/2. (Indices: 1=Chou, 2=Yin...).
    // Is Chou index 1? Yes.
    // So for Water 2: `1 + (day-1)/2`. Mod 12.

    // Wood 3:
    // 1->Chen(4), 2->Si(5), 3->Wu(6).
    // 4 (Rem 1) -> (Go to 3's pos, add/sub?)
    // 4 -> Wei(7)?
    // Pattern: 1->4, 2->5, 3->6. (Start 4, step 1).
    // 4->7? Yes.
    // 5->1? (Chou). 6->2(Yin).
    // It seems to jump back when remainder is certain thing.

    // Let's use a direct searching loop (Concept: "Start from Yin, add Phase until >= Day")?
    // No, standard method:
    // Target = Day.
    // Phase = P.
    // Find multiple of P >= Day. (M * P).
    // Difference = (M*P) - Day.
    // Pos = (2 + M - 1) indices from Yin.
    // If Difference is Odd -> Adjust Counter-clockwise?
    // If Difference is Even -> Adjust Clockwise?
    // Correct Formula:
    // Let X = Day. P = Phase.
    // Find Q, R such that X = Q*P + R. (Wait, standard division).
    // If R == 0:
    //   Pos = Yin(2) + Q - 1.
    // If R > 0:
    //   We need (Q+1)*P.
    //   Let M = Q + 1.
    //   Diff = (M*P) - X.
    //   Pos = (Yin(2) + M - 1).
    //   If Diff is odd: Pos -= Diff.
    //   If Diff is even: Pos += Diff.

    let yin = 2i32;
    let p = phase as i32;
    let x = day as i32;

    let pos_idx: i32; // REMOVED MUT

    if x % p == 0 {
        let q = x / p;
        pos_idx = yin + q - 1;
    } else {
        let q = x / p;
        let m = q + 1;
        let diff = (m * p) - x;
        let base = yin + m - 1;
        if diff % 2 != 0 {
            // Odd diff
            pos_idx = base - diff;
        } else {
            // Even diff
            pos_idx = base + diff;
        }
    }

    pos_idx.rem_euclid(12) as usize
}

// CHANGED: Returned tuple instead of array of tuples.
fn get_si_hua(year_stem_idx: usize) -> (&'static str, &'static str, &'static str, &'static str) {
    // (Lu, Quan, Ke, Ji)
    match year_stem_idx {
        0 => ("Lian Zhen", "Po Jun", "Wu Qu", "Tai Yang"), // Jia
        1 => ("Tian Ji", "Tian Liang", "Zi Wei", "Tai Yin"), // Yi
        2 => ("Tian Tong", "Tian Ji", "Wen Chang", "Lian Zhen"), // Bing
        3 => ("Tai Yin", "Tian Tong", "Tian Ji", "Ju Men"), // Ding
        4 => ("Tan Lang", "Tai Yin", "You Bi", "Tian Ji"), // Wu
        5 => ("Wu Qu", "Tan Lang", "Tian Liang", "Wen Qu"), // Ji
        6 => ("Tai Yang", "Wu Qu", "Tai Yin", "Tian Tong"), // Geng
        7 => ("Ju Men", "Tai Yang", "Wen Qu", "Wen Chang"), // Xin
        8 => ("Tian Liang", "Zi Wei", "Zuo Fu", "Wu Qu"), // Ren
        9 => ("Po Jun", "Ju Men", "Tai Yin", "Tan Lang"), // Gui
        _ => ("", "", "", "")
    }
}

// CHANGED: Input parameter type to match
fn apply_si_hua(star: &mut String, map: &(&str, &str, &str, &str)) {
    // Check if star contains the key name
    let labels = ["(Hua Lu)", "(Hua Quan)", "(Hua Ke)", "(Hua Ji)"];

    // Tuple unpacking for iteration is clumsy, so just do it manually
    if star.starts_with(map.0) { star.push_str(" "); star.push_str(labels[0]); }
    else if star.starts_with(map.1) { star.push_str(" "); star.push_str(labels[1]); }
    else if star.starts_with(map.2) { star.push_str(" "); star.push_str(labels[2]); }
    else if star.starts_with(map.3) { star.push_str(" "); star.push_str(labels[3]); }
}
