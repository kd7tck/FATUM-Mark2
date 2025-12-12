use serde::{Deserialize, Serialize};
use crate::tools::chinese_meta::{get_branch};

#[derive(Debug, Serialize, Deserialize)]
pub struct DaLiuRenConfig {
    pub day_stem_idx: usize, // 0-9
    pub day_branch_idx: usize, // 0-11
    pub hour_branch_idx: usize, // 0-11
    pub solar_term_idx: usize, // 0-23
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DaLiuRenChart {
    pub earth_plate: Vec<String>, // Fixed 12
    pub heaven_plate: Vec<String>, // Rotated 12 (Branch names)
    pub four_lessons: Vec<Lesson>,
    pub three_transmissions: Vec<String>, // The 3 Branches
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lesson {
    pub bottom: String, // Earth Position (or Stem Parasite)
    pub top: String, // Heaven Branch
    pub bottom_idx: usize,
    pub top_idx: usize,
}

pub fn generate_da_liu_ren(config: DaLiuRenConfig) -> Result<DaLiuRenChart, String> {
    // 1. Determine Monthly General (Yue Jiang)
    // Formula: In Term T, Jiang is J.
    // Standard Mapping:
    // Terms 22-23 (Winter Solstice/Lesser Cold) -> Jiang Zi (Rat, 0) ? No, Jiang is Chou (1) usually?
    // Let's use the Table:
    // Yin Month (Terms 1,2) -> Jiang Hai (11)
    // Mao Month (Terms 3,4) -> Jiang Xu (10)
    // Chen Month (Terms 5,6) -> Jiang You (9)
    // Si Month (Terms 7,8) -> Jiang Shen (8)
    // Wu Month (Terms 9,10) -> Jiang Wei (7)
    // Wei Month (Terms 11,12) -> Jiang Wu (6)
    // Shen Month (Terms 13,14) -> Jiang Si (5)
    // You Month (Terms 15,16) -> Jiang Chen (4)
    // Xu Month (Terms 17,18) -> Jiang Mao (3)
    // Hai Month (Terms 19,20) -> Jiang Yin (2)
    // Zi Month (Terms 21,22) -> Jiang Chou (1)
    // Chou Month (Terms 23,0) -> Jiang Zi (0)

    // Solar Term Index 0 = Start of Spring (Tiger Month)?
    // Usually Term 0 = Li Chun (Start Spring).
    // If Term 0 is Li Chun:
    // 0,1 -> Yin Month -> Hai (11).
    // Mapping: Jiang = (11 - (term / 2)).
    // Example: Term 0 -> 11 - 0 = 11 (Hai). Correct.
    // Example: Term 2 (Jing Zhe) -> 11 - 1 = 10 (Xu). Correct.
    // Example: Term 23 (Da Han) -> 11 - 11 = 0 (Zi). Correct.
    // Handle wrap around? No, 11-11=0.

    let month_idx = config.solar_term_idx / 2;
    let jiang_idx = (11i32 - month_idx as i32).rem_euclid(12) as usize;

    // 2. Heaven Plate
    // Place Jiang on Hour Branch.
    // Shift = (Jiang - Hour).
    // If Hour is 0 (Zi), Jiang is at 0. Shift = Jiang.
    // Heaven[i] = (i + Shift) % 12?
    // Let's verify:
    // Hour = Zi (0). Jiang = Hai (11).
    // Heaven at Zi (0) should be Hai (11).
    // Formula: Val = (Pos + Shift) -> 11 = (0 + S) -> S = 11.
    // Shift = (Jiang - Hour + 12) % 12.
    // Check: Hour = Wu (6). Jiang = Hai (11).
    // Shift = (11 - 6 + 12) % 12 = 5.
    // Heaven at Wu (6) -> (6 + 5) = 11 (Hai). Correct.

    let shift = (jiang_idx as i32 - config.hour_branch_idx as i32).rem_euclid(12);

    let mut heaven_plate = Vec::new();
    let mut heaven_map = [0usize; 12]; // Map[Earth_Idx] = Heaven_Idx

    for i in 0..12 {
        let val = (i as i32 + shift).rem_euclid(12) as usize;
        heaven_plate.push(get_branch(val).to_string());
        heaven_map[i] = val;
    }

    let earth_plate: Vec<String> = (0..12).map(|i| get_branch(i).to_string()).collect();

    // 3. Four Lessons (Si Ke)
    // Determine Parasitic Branch for Day Stem (Gan Ji)
    // Jia(0)->Yin(2), Yi(1)->Chen(4), Bing(2)->Si(5), Ding(3)->Wei(7), Wu(4)->Si(5), Ji(5)->Wei(7), Geng(6)->Shen(8), Xin(7)->Xu(10), Ren(8)->Hai(11), Gui(9)->Chou(1)
    let gan_ji_map = [2, 4, 5, 7, 5, 7, 8, 10, 11, 1];
    let gan_ji = gan_ji_map[config.day_stem_idx];

    // Lesson 1: Day Stem (Gan) -> Heaven of Parasite
    // Bottom is conceptually the Stem, but we look at the Earth position of its parasite.
    let l1_bottom = gan_ji;
    let l1_top = heaven_map[l1_bottom];

    // Lesson 2: L1 Top as Bottom
    let l2_bottom = l1_top;
    let l2_top = heaven_map[l2_bottom];

    // Lesson 3: Day Branch (Zhi)
    let l3_bottom = config.day_branch_idx;
    let l3_top = heaven_map[l3_bottom];

    // Lesson 4: L3 Top as Bottom
    let l4_bottom = l3_top;
    let l4_top = heaven_map[l4_bottom];

    let lessons_raw = vec![
        (l1_bottom, l1_top),
        (l2_bottom, l2_top),
        (l3_bottom, l3_top),
        (l4_bottom, l4_top)
    ];

    let lessons: Vec<Lesson> = lessons_raw.iter().map(|(b, t)| {
        Lesson {
            bottom_idx: *b,
            top_idx: *t,
            bottom: get_branch(*b).to_string(),
            top: get_branch(*t).to_string(),
        }
    }).collect();

    // 4. Three Transmissions (San Chuan) - Algorithm "Ze Ke" (Simplified)
    // Determine Element relationships (0=Water, 1=Earth, 2=Wood, 3=Wood, 4=Earth, 5=Fire, 6=Fire, 7=Earth, 8=Metal, 9=Metal, 10=Earth, 11=Water)
    // Actually, let's use a helper for element index (0=Wood, 1=Fire, 2=Earth, 3=Metal, 4=Water) to simplify comparisons.
    // Branch Elements:
    // Hai(11), Zi(0) -> Water (4)
    // Yin(2), Mao(3) -> Wood (0)
    // Si(5), Wu(6) -> Fire (1)
    // Shen(8), You(9) -> Metal (3)
    // Chen(4), Xu(10), Chou(1), Wei(7) -> Earth (2)

    let get_el = |b: usize| -> usize {
        match b {
            11 | 0 => 4, // Water
            2 | 3 => 0, // Wood
            5 | 6 => 1, // Fire
            8 | 9 => 3, // Metal
            _ => 2, // Earth
        }
    };

    // Overcomes? (A overcomes B) -> Metal(3)>Wood(0), Wood(0)>Earth(2), Earth(2)>Water(4), Water(4)>Fire(1), Fire(1)>Metal(3)
    let overcomes = |a: usize, b: usize| -> bool {
        let ea = get_el(a);
        let eb = get_el(b);
        match (ea, eb) {
            (3, 0) => true,
            (0, 2) => true,
            (2, 4) => true,
            (4, 1) => true,
            (1, 3) => true,
            _ => false
        }
    };

    let mut candidates_lower_destroys_upper = Vec::new(); // Ze (Rebellion) - Priority
    let mut candidates_upper_destroys_lower = Vec::new(); // Ke (Control)

    for (i, lesson) in lessons.iter().enumerate() {
        if overcomes(lesson.bottom_idx, lesson.top_idx) {
            candidates_lower_destroys_upper.push((i, lesson.top_idx));
        }
        if overcomes(lesson.top_idx, lesson.bottom_idx) {
            candidates_upper_destroys_lower.push((i, lesson.top_idx));
        }
    }

    let mut first_transmission = None;
    let day_is_yang = config.day_stem_idx % 2 == 0; // Jia(0) is Yang

    // Rule 1: Ze (Lower > Upper)
    if !candidates_lower_destroys_upper.is_empty() {
        if candidates_lower_destroys_upper.len() == 1 {
            first_transmission = Some(candidates_lower_destroys_upper[0].1);
        } else {
            // Bi Yong (Compare with Day)
            // Yang Day -> Pick Yang Branch (Top).
            // Yin Day -> Pick Yin Branch.
            // Branch Yin/Yang:
            // Yang: Zi(0), Yin(2), Chen(4), Wu(6), Shen(8), Xu(10) ??
            // Standard: Odd indices in list? No.
            // Zi(0) is Yang. Chou(1) is Yin.
            // So Even Index = Yang, Odd Index = Yin.
            for (_, branch_idx) in &candidates_lower_destroys_upper {
                let branch_is_yang = branch_idx % 2 == 0;
                if branch_is_yang == day_is_yang {
                    first_transmission = Some(*branch_idx);
                    break;
                }
            }
            // If still none (e.g. Day Yang, but all candidates Yin), pick first?
            if first_transmission.is_none() {
                first_transmission = Some(candidates_lower_destroys_upper[0].1);
            }
        }
    }
    // Rule 2: Ke (Upper > Lower)
    else if !candidates_upper_destroys_lower.is_empty() {
        if candidates_upper_destroys_lower.len() == 1 {
            first_transmission = Some(candidates_upper_destroys_lower[0].1);
        } else {
            // Bi Yong
            for (_, branch_idx) in &candidates_upper_destroys_lower {
                let branch_is_yang = branch_idx % 2 == 0;
                if branch_is_yang == day_is_yang {
                    first_transmission = Some(*branch_idx);
                    break;
                }
            }
            if first_transmission.is_none() {
                first_transmission = Some(candidates_upper_destroys_lower[0].1);
            }
        }
    }

    // Rule 3: Yao Ke (Remote) - Simplified Fallback
    // If no direct clashes, check Day Stem vs Heaven Plates of lessons.
    if first_transmission.is_none() {
        // Fallback: Just pick Lesson 1 Top (Yuan Shou / Chief)
        // This is a gross simplification but ensures a result for MVP.
        first_transmission = Some(lessons[0].top_idx);
    }

    let t1 = first_transmission.unwrap();
    let t2 = heaven_map[t1]; // Heaven atop T1
    let t3 = heaven_map[t2]; // Heaven atop T2

    let transmissions = vec![
        get_branch(t1).to_string(),
        get_branch(t2).to_string(),
        get_branch(t3).to_string()
    ];

    Ok(DaLiuRenChart {
        earth_plate,
        heaven_plate,
        four_lessons: lessons,
        three_transmissions: transmissions,
        description: "Standard Yuan Shou / Ze Ke Calculation".to_string(),
    })
}
