use std::io::{self, Write};
use anyhow::Result;
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use crate::client::CurbyClient;
use crate::engine::SimulationSession;

/// Configuration for a Feng Shui analysis session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FengShuiConfig {
    pub birth_year: Option<i32>,
    pub gender: Option<String>,
    pub construction_year: i32,
    pub facing_degrees: f64,
    pub current_year: Option<i32>, // Defaults to system year if None
    pub intention: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FengShuiReport {
    pub kua: Option<KuaProfile>,
    pub chart: FlyingStarChart,
    pub quantum: QuantumAnalysis,
    pub advice: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KuaProfile {
    pub number: i32,
    pub group: String,
    pub element: String,
    pub lucky_directions: Vec<(String, String)>, // (Direction, Meaning)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlyingStarChart {
    pub period: i32,
    pub facing_mountain: String,
    pub sitting_mountain: String,
    pub palaces: Vec<Palace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palace {
    pub sector: String, // e.g., "North", "South-East"
    pub base_star: i32,
    pub mountain_star: i32,
    pub water_star: i32,
    pub annual_star: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumAnalysis {
    pub volatility_index: f64, // 0.0 to 1.0
    pub focus_sector: String,
    pub anomalies: Vec<String>,
    pub intention_resonance: Option<String>,
}

// === MAIN ENTRY POINTS ===

/// CLI Entry Point
pub async fn run_feng_shui_cli() -> Result<()> {
    println!("=== QUANTUM FENG SHUI & FLYING STARS SYSTEM ===");
    println!("Powered by CURBy Quantum Entropy");
    println!("-----------------------------------------------");

    // 1. Gather Inputs
    let birth_year = prompt_input("Enter Birth Year (YYYY) [Optional]: ");
    let birth_year = if birth_year.is_empty() { None } else { birth_year.parse().ok() };

    let gender = if birth_year.is_some() {
        let g = prompt_input("Enter Gender (M/F): ").to_uppercase();
        if g == "M" || g == "F" { Some(g) } else { None }
    } else {
        None
    };

    let construction_year = prompt_input("Enter Building Construction Year (YYYY) [Default: 2024]: ");
    let construction_year = construction_year.parse().unwrap_or(2024);

    let facing_input = prompt_input("Enter Facing Direction in Degrees (0-360): ");
    let facing_degrees = facing_input.parse().unwrap_or(180.0); // Default South

    let intention = prompt_input("Enter specific intention or question [Optional]: ");
    let intention = if intention.is_empty() { None } else { Some(intention) };

    let config = FengShuiConfig {
        birth_year,
        gender,
        construction_year,
        facing_degrees,
        current_year: None,
        intention,
    };

    // 2. Run Analysis
    println!("\nInitializing Quantum Simulation & Calculation...");
    let report = generate_report(config).await?;

    // 3. Render Output
    println!("\n================ REPORT ================");

    if let Some(kua) = &report.kua {
        println!("\n[ USER PROFILE ]");
        println!("Kua Number: {}", kua.number);
        println!("Element: {}", kua.element);
        println!("Group: {}", kua.group);
        println!("Lucky Directions:");
        for (dir, meaning) in &kua.lucky_directions {
            println!("  - {}: {}", dir, meaning);
        }
    }

    println!("\n[ FLYING STAR CHART (Period {}) ]", report.chart.period);
    println!("Facing: {} | Sitting: {}", report.chart.facing_mountain, report.chart.sitting_mountain);
    println!("Format: [Base | Mtn | Wtr | Ann]");

    for palace in &report.chart.palaces {
        println!("  {:<12}: [ {} | {} | {} | {} ]",
            palace.sector, palace.base_star, palace.mountain_star, palace.water_star, palace.annual_star);
    }

    println!("\n[ QUANTUM ANALYSIS ]");
    println!("Qi Volatility Index: {:.2}", report.quantum.volatility_index);
    println!("Quantum Focus Sector: {}", report.quantum.focus_sector);
    if let Some(res) = &report.quantum.intention_resonance {
        println!("Intention Resonance: {}", res);
    }
    if !report.quantum.anomalies.is_empty() {
        println!("Anomalies Detected: {:?}", report.quantum.anomalies);
    }

    println!("\n[ ADVICE ]");
    for tip in &report.advice {
        println!("* {}", tip);
    }

    Ok(())
}

/// Core Logic Handler (Shared by CLI and Server)
pub async fn generate_report(config: FengShuiConfig) -> Result<FengShuiReport> {
    // 1. Fetch Entropy
    let mut client = CurbyClient::new();
    // Fetch a good chunk for deep simulation
    let entropy = client.fetch_bulk_randomness(2048).await?;
    let session = SimulationSession::new(entropy);

    // 2. Calculate Kua
    let kua_profile = if let (Some(y), Some(g)) = (config.birth_year, &config.gender) {
        Some(calculate_kua_profile(y, g))
    } else {
        None
    };

    // 3. Calculate Flying Stars
    let current_year = config.current_year.unwrap_or_else(|| chrono::Local::now().year());
    let chart = calculate_flying_star_chart(config.construction_year, config.facing_degrees, current_year);

    // 4. Quantum Simulation
    let quantum = run_quantum_analysis(&session, &chart, config.intention.as_deref());

    // 5. Generate Advice
    let advice = generate_advice(&chart, &kua_profile, &quantum);

    Ok(FengShuiReport {
        kua: kua_profile,
        chart,
        quantum,
        advice,
    })
}

// === HELPERS & LOGIC ===

fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap_or(());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap_or(0);
    buffer.trim().to_string()
}

// --- KUA LOGIC ---

pub fn calculate_kua_profile(year: i32, gender: &str) -> KuaProfile {
    let mut sum = 0;
    let digits: Vec<u32> = year.to_string().chars().filter_map(|c| c.to_digit(10)).collect();
    for d in digits { sum += d as i32; }

    while sum > 9 {
        let s_str = sum.to_string();
        sum = 0;
        for c in s_str.chars() {
            sum += c.to_digit(10).unwrap() as i32;
        }
    }

    let k = if gender == "M" {
        let mut val = 11 - sum;
        while val > 9 { val -= 9; }
        if val == 0 { 9 } else if val == 5 { 2 } else { val }
    } else {
        let mut val = 4 + sum;
        while val > 9 { val -= 9; }
        if val == 5 { 8 } else { val }
    };

    let group = if [1, 3, 4, 9].contains(&k) { "East Group".to_string() } else { "West Group".to_string() };

    let element = match k {
        1 => "Water",
        2 | 8 => "Earth",
        3 | 4 => "Wood",
        6 | 7 => "Metal",
        9 => "Fire",
        _ => "Earth",
    }.to_string();

    // Directions
    let dirs = match k {
        1 => vec![("SE", "Sheng Chi (Wealth)"), ("E", "Tian Yi (Health)"), ("S", "Yan Nian (Love)"), ("N", "Fu Wei (Growth)")],
        2 => vec![("NE", "Sheng Chi (Wealth)"), ("W", "Tian Yi (Health)"), ("NW", "Yan Nian (Love)"), ("SW", "Fu Wei (Growth)")],
        3 => vec![("S", "Sheng Chi (Wealth)"), ("N", "Tian Yi (Health)"), ("SE", "Yan Nian (Love)"), ("E", "Fu Wei (Growth)")],
        4 => vec![("N", "Sheng Chi (Wealth)"), ("S", "Tian Yi (Health)"), ("E", "Yan Nian (Love)"), ("SE", "Fu Wei (Growth)")],
        6 => vec![("W", "Sheng Chi (Wealth)"), ("NE", "Tian Yi (Health)"), ("SW", "Yan Nian (Love)"), ("NW", "Fu Wei (Growth)")],
        7 => vec![("NW", "Sheng Chi (Wealth)"), ("SW", "Tian Yi (Health)"), ("NE", "Yan Nian (Love)"), ("W", "Fu Wei (Growth)")],
        8 => vec![("SW", "Sheng Chi (Wealth)"), ("NW", "Tian Yi (Health)"), ("W", "Yan Nian (Love)"), ("NE", "Fu Wei (Growth)")],
        9 => vec![("E", "Sheng Chi (Wealth)"), ("SE", "Tian Yi (Health)"), ("N", "Yan Nian (Love)"), ("S", "Fu Wei (Growth)")],
        _ => vec![],
    };

    KuaProfile {
        number: k,
        group,
        element,
        lucky_directions: dirs.into_iter().map(|(a,b)| (a.to_string(), b.to_string())).collect(),
    }
}

// --- FLYING STARS LOGIC ---

pub fn calculate_flying_star_chart(construction_year: i32, degrees: f64, current_year: i32) -> FlyingStarChart {
    // 1. Period
    let period = match construction_year {
        y if y < 1864 => 1, // Fallback
        y if y <= 1883 => 1,
        y if y <= 1903 => 2,
        y if y <= 1923 => 3,
        y if y <= 1943 => 4,
        y if y <= 1963 => 5,
        y if y <= 1983 => 6,
        y if y <= 2003 => 7,
        y if y <= 2023 => 8,
        _ => 9,
    };

    // 2. Mountains & Facing
    let (facing_sector, facing_mountain_idx, _facing_pol) = get_24_mountain(degrees);
    // Sitting is 180 opposite
    let (sitting_sector, sitting_mountain_idx, _sitting_pol) = get_24_mountain((degrees + 180.0) % 360.0);

    let facing_label = format!("{} ({})", facing_sector, get_mountain_name(&facing_sector, facing_mountain_idx));
    let sitting_label = format!("{} ({})", sitting_sector, get_mountain_name(&sitting_sector, sitting_mountain_idx));

    // 3. Base Chart (Period Star in Center)
    let base_chart = fly_stars(period, true); // Period stars always fly forward

    // 4. Mountain & Water Stars
    // Center stars for M/W are determined by the base star at Sitting (Mountain) and Facing (Water) sectors.
    // We need to find WHICH star is at the Sitting/Facing sector in the base chart.

    // Map sector names to indices in the 9-cell grid (standard Luo Shu order: 0=Center? No. Let's define order.)
    // Order: Center, NW, W, NE, S, N, SW, E, SE
    let sector_map = |s: &str| match s {
        "Center" => 0, "NW" => 1, "W" => 2, "NE" => 3, "S" => 4,
        "N" => 5, "SW" => 6, "E" => 7, "SE" => 8, _ => 0
    };

    let sit_base_star = base_chart[sector_map(&sitting_sector)];
    let face_base_star = base_chart[sector_map(&facing_sector)];

    // Determine flight direction for Mountain Star
    // 1. Look at the original home of the `sit_base_star`.
    // 2. Identify the specific mountain index (1, 2, or 3) from the house's Sitting Mountain Index.
    // 3. Check polarity of that specific mountain in the star's original home.
    let mtn_flight_pol = get_flight_polarity(sit_base_star, sitting_mountain_idx);
    let mtn_chart = fly_stars(sit_base_star, mtn_flight_pol);

    // Determine flight direction for Water Star
    // Same logic but using `face_base_star` and Facing Mountain Index
    let wtr_flight_pol = get_flight_polarity(face_base_star, facing_mountain_idx);
    let wtr_chart = fly_stars(face_base_star, wtr_flight_pol);

    // 5. Annual Star
    // Simple calculation: Sum digits of year, subtract from 11 (similar to Kua M?)
    // Actually, Annual Star formula:
    // 2024 -> 3. (11 - (2+0+2+4) = 3).
    // wait, 2024 sum is 8. 11-8=3. Correct.
    // 2000 -> 11 - 2 = 9. Correct.
    let mut sum = 0;
    let digits: Vec<u32> = current_year.to_string().chars().filter_map(|c| c.to_digit(10)).collect();
    for d in digits { sum += d as i32; }
    while sum > 9 {
        let s_str = sum.to_string();
        sum = 0;
        for c in s_str.chars() {
             sum += c.to_digit(10).unwrap() as i32;
        }
    }
    let mut annual_star = 11 - sum;
    while annual_star > 9 { annual_star -= 9; }
    if annual_star == 0 { annual_star = 9; }
    // Annual stars always fly forward? Yes.
    let annual_chart = fly_stars(annual_star, true);

    // Assemble Palaces
    let sectors_ordered = vec!["Center", "NW", "W", "NE", "S", "N", "SW", "E", "SE"];
    let mut palaces = Vec::new();
    for i in 0..9 {
        palaces.push(Palace {
            sector: sectors_ordered[i].to_string(),
            base_star: base_chart[i],
            mountain_star: mtn_chart[i],
            water_star: wtr_chart[i],
            annual_star: annual_chart[i],
        });
    }

    FlyingStarChart {
        period,
        facing_mountain: facing_label,
        sitting_mountain: sitting_label,
        palaces,
    }
}

fn fly_stars(center_star: i32, forward: bool) -> Vec<i32> {
    // Path: Center(0) -> NW(1) -> W(2) -> NE(3) -> S(4) -> N(5) -> SW(6) -> E(7) -> SE(8)
    // This path corresponds to Luo Shu numbers: 5 -> 6 -> 7 -> 8 -> 9 -> 1 -> 2 -> 3 -> 4
    // We fill the array [Center, NW, W, NE, S, N, SW, E, SE]

    let mut chart = vec![0; 9];
    let mut current = center_star;

    // The indices in `chart` corresponding to the sequence of flight
    let path = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

    for &idx in &path {
        chart[idx] = current;
        if forward {
            current += 1;
            if current > 9 { current = 1; }
        } else {
            current -= 1;
            if current < 1 { current = 9; }
        }
    }
    chart
}

fn get_flight_polarity(star: i32, house_mtn_idx: usize) -> bool {
    // star: 1..9
    // house_mtn_idx: 1, 2, 3 (Mapped to 0, 1, 2 for array access)
    // Returns true for Forward (+), false for Reverse (-)

    // Each star corresponds to a "Home" sector in Luo Shu:
    // 1:N, 2:SW, 3:E, 4:SE, 5:Center, 6:NW, 7:W, 8:NE, 9:S
    // 5 usually borrows from the period or has special rules.
    // Standard rule for 5: It takes polarity of the original "Base Star" of the house?
    // Or it follows the "Gender" of the Star?
    // Simplified: 5 borrows the sector of the Facing/Sitting?
    // Let's assume 5 behaves like the palace it is currently in (base chart).

    // Polarity Table for the 24 Mountains (1=Yang, 0=Yin)
    // Index 0, 1, 2 for each sector.
    // N:  [+, -, -]
    // SW: [-, +, +]
    // E:  [+, -, -]
    // SE: [-, +, +]
    // NW: [-, +, +]
    // W:  [+, -, -]
    // NE: [-, +, +]
    // S:  [+, -, -]

    // Wait, let's verify standard chart:
    // Jiang Da Hong Ring:
    // N (1):  Ren(+), Zi(-), Gui(-)
    // SW (2): Wei(-), Kun(+), Shen(+)
    // E (3):  Jia(+), Mao(-), Yi(-)
    // SE (4): Chen(-), Xun(+), Si(+)
    // Center (5): No standard mountains. Usually follows the "ruling" star?
    //    If Star is 5, it adopts the polarity of the star matching the Period?
    //    Or, if 5 is the star, look at the original palace of the *Period*?
    //    Let's use a standard convention: 5 uses the polarity of the palace it replaces.

    // NW (6): Xu(-), Qian(+), Hai(+)
    // W (7):  Geng(+), You(-), Xin(-)
    // NE (8): Chou(-), Gen(+), Yin(+)
    // S (9):  Bing(+), Wu(-), Ding(-)

    // Even numbers (2,4,6,8) -> Yin-Yang-Yang [- + +]
    // Odd numbers (1,3,7,9)  -> Yang-Yin-Yin [+ - -]
    // Note: 5 is special.

    let pattern_odd = [true, false, false];  // + - -
    let pattern_even = [false, true, true];  // - + +

    if star == 5 {
        // Special case for 5. It aligns with the original mountain star of the *Sector* it is currently occupying?
        // No, it aligns with the *Period* star?
        // Simplification: Treat 5 as the Period Star for polarity lookup?
        // Actually, most sources say if 5, look at the Period Star.
        // Let's defer 5 to "true" (Forward) for safety or map it to period.
        return true;
    }

    let is_even = star % 2 == 0;
    if is_even {
        pattern_even[house_mtn_idx - 1]
    } else {
        pattern_odd[house_mtn_idx - 1]
    }
}

fn get_24_mountain(deg: f64) -> (String, usize, bool) {
    // deg 0-360
    // Returns (Sector, Index 1-3, Polarity)
    // Note: This Polarity is just for info, not the Flight Polarity calculated above.

    // Normalized to 0-360
    let d = (deg % 360.0 + 360.0) % 360.0;

    // Each mountain is 15 degrees.
    // N2 is center of North (0 deg / 360 deg). Range: 352.5 - 7.5.

    // Helper to find sector index 0-23
    // 0 = N2 (352.5 - 7.5) -> Wait, standard order usually starts N1 (Ren) at 337.5
    // Let's use standard offset. 337.5 is start of N1.

    // let _offset_deg = (d + 22.5) % 360.0; // Rotate so N1 starts at 0? No.

    // Let's easier map:
    // N: 337.5 - 22.5
    //   N1 (Ren): 337.5 - 352.5
    //   N2 (Zi):  352.5 - 7.5
    //   N3 (Gui): 7.5 - 22.5

    if d >= 337.5 && d < 352.5 { return ("N".to_string(), 1, true); }
    if d >= 352.5 || d < 7.5   { return ("N".to_string(), 2, false); }
    if d >= 7.5 && d < 22.5    { return ("N".to_string(), 3, false); }

    if d >= 22.5 && d < 37.5   { return ("NE".to_string(), 1, false); }
    if d >= 37.5 && d < 52.5   { return ("NE".to_string(), 2, true); }
    if d >= 52.5 && d < 67.5   { return ("NE".to_string(), 3, true); }

    if d >= 67.5 && d < 82.5   { return ("E".to_string(), 1, true); }
    if d >= 82.5 && d < 97.5   { return ("E".to_string(), 2, false); }
    if d >= 97.5 && d < 112.5  { return ("E".to_string(), 3, false); }

    if d >= 112.5 && d < 127.5 { return ("SE".to_string(), 1, false); }
    if d >= 127.5 && d < 142.5 { return ("SE".to_string(), 2, true); }
    if d >= 142.5 && d < 157.5 { return ("SE".to_string(), 3, true); }

    if d >= 157.5 && d < 172.5 { return ("S".to_string(), 1, true); }
    if d >= 172.5 && d < 187.5 { return ("S".to_string(), 2, false); }
    if d >= 187.5 && d < 202.5 { return ("S".to_string(), 3, false); }

    if d >= 202.5 && d < 217.5 { return ("SW".to_string(), 1, false); }
    if d >= 217.5 && d < 232.5 { return ("SW".to_string(), 2, true); }
    if d >= 232.5 && d < 247.5 { return ("SW".to_string(), 3, true); }

    if d >= 247.5 && d < 262.5 { return ("W".to_string(), 1, true); }
    if d >= 262.5 && d < 277.5 { return ("W".to_string(), 2, false); }
    if d >= 277.5 && d < 292.5 { return ("W".to_string(), 3, false); }

    if d >= 292.5 && d < 307.5 { return ("NW".to_string(), 1, false); }
    if d >= 307.5 && d < 322.5 { return ("NW".to_string(), 2, true); }
    if d >= 322.5 && d < 337.5 { return ("NW".to_string(), 3, true); }

    ("N".to_string(), 2, false) // Fallback
}

fn get_mountain_name(sector: &str, idx: usize) -> &'static str {
    match (sector, idx) {
        ("N", 1) => "Ren", ("N", 2) => "Zi", ("N", 3) => "Gui",
        ("NE", 1) => "Chou", ("NE", 2) => "Gen", ("NE", 3) => "Yin",
        ("E", 1) => "Jia", ("E", 2) => "Mao", ("E", 3) => "Yi",
        ("SE", 1) => "Chen", ("SE", 2) => "Xun", ("SE", 3) => "Si",
        ("S", 1) => "Bing", ("S", 2) => "Wu", ("S", 3) => "Ding",
        ("SW", 1) => "Wei", ("SW", 2) => "Kun", ("SW", 3) => "Shen",
        ("W", 1) => "Geng", ("W", 2) => "You", ("W", 3) => "Xin",
        ("NW", 1) => "Xu", ("NW", 2) => "Qian", ("NW", 3) => "Hai",
        _ => "Unknown"
    }
}

// --- QUANTUM SIMULATION ---

fn run_quantum_analysis(session: &SimulationSession, _chart: &FlyingStarChart, intention: Option<&str>) -> QuantumAnalysis {
    let sectors = vec![
        "North", "North-East", "East", "South-East", "South",
        "South-West", "West", "North-West", "Center"
    ];

    // 1. Determine Focus Sector (Where the energy gathers)
    let report = session.simulate_decision(
        &sectors.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        5000 // Deep scan
    );

    // 2. Volatility (Simulated by variance in win counts - simplified here as just normalized entropy check)
    // We'll use the "winning_percentage" spread from the report if available, or generate a random factor seeded by quantum.
    // Since `SimulationReport` might not expose variance, we use the anomalies count as a proxy.
    let volatility = (report.anomalies.len() as f64 * 0.1).min(1.0);

    // 3. Intention Resonance
    let resonance = if let Some(_intent) = intention {
        // Run a specific check: "Does intention align with X?"
        // We simulate yes/no for each sector.
        // For now, simple: Pick a sector that 'likes' the intention.
        let res_report = session.simulate_decision(
            &sectors.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            1000
        );
        Some(res_report.winner)
    } else {
        None
    };

    QuantumAnalysis {
        volatility_index: volatility,
        focus_sector: report.winner,
        anomalies: report.anomalies,
        intention_resonance: resonance,
    }
}

fn generate_advice(chart: &FlyingStarChart, kua: &Option<KuaProfile>, quantum: &QuantumAnalysis) -> Vec<String> {
    let mut advice = Vec::new();

    // 1. Base Flying Star Advice
    // Find the Wealth Star (usually 8, 9, 1 in current periods)
    // Period 9 (2024+): 9 is Primary Wealth, 1 is Future Wealth, 2 is distant.
    let wealth_star = if chart.period == 9 { 9 } else { 8 };

    // Find where the Water Star == Wealth Star
    for p in &chart.palaces {
        if p.water_star == wealth_star {
            advice.push(format!("Sector {} contains the Water Star {}, activating Wealth Luck. Place a water feature here.", p.sector, wealth_star));
        }
        if p.mountain_star == wealth_star {
            advice.push(format!("Sector {} contains the Mountain Star {}, activating Health/Relations. good for a bedroom.", p.sector, wealth_star));
        }
    }

    // 2. Kua Advice
    if let Some(k) = kua {
        advice.push(format!("Your Life Gua (Kua) is {}. You are {}.", k.number, k.group));
        if let Some((dir, _)) = k.lucky_directions.first() {
            advice.push(format!("Your strongest direction is {}. Face this way when working.", dir));
        }
    }

    // 3. Quantum Advice
    advice.push(format!("Quantum Scan indicates high energy in {}. Pay attention to events here.", quantum.focus_sector));
    if quantum.volatility_index > 0.5 {
        advice.push("Energy is currently volatile. Avoid renovations in the Focus Sector today.".to_string());
    }

    advice
}
