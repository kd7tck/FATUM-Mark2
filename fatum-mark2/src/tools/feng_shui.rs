use std::io::{self, Write};
use anyhow::Result;
use chrono::{Datelike, NaiveDate, TimeZone, Local};
use serde::{Deserialize, Serialize};
use crate::client::CurbyClient;
use crate::engine::SimulationSession;
use std::collections::HashMap;

/// Configuration for a Feng Shui analysis session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FengShuiConfig {
    pub birth_year: Option<i32>,
    pub gender: Option<String>,
    pub construction_year: i32,
    pub facing_degrees: f64,
    pub current_year: Option<i32>, // Defaults to system year if None
    pub current_month: Option<u32>, // Defaults to system month
    pub current_day: Option<u32>,   // Defaults to system day
    pub intention: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FengShuiReport {
    pub kua: Option<KuaProfile>,
    pub annual_chart: FlyingStarChart,
    pub monthly_chart: Option<FlyingStarChart>,
    pub daily_chart: Option<FlyingStarChart>,
    pub formations: Vec<String>,
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
    pub period: i32, // For monthly/daily, this represents the ruling star
    pub label: String, // "Period 9", "Month 5", "Day 2"
    pub facing_mountain: String,
    pub sitting_mountain: String,
    pub palaces: Vec<Palace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palace {
    pub sector: String, // e.g., "North", "South-East"
    pub base_star: i32,
    pub mountain_star: i32,
    pub water_star: i32, // Usually only relevant for Base Chart, but we keep structure
    pub visiting_star: i32, // The Annual/Monthly/Daily star visiting this sector
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumAnalysis {
    pub volatility_index: f64, // 0.0 to 1.0
    pub focus_sector: String,
    pub anomalies: Vec<String>,
    pub intention_resonance: Option<String>,
    pub suggested_cures: Vec<CureSuggestion>,
    pub qi_flow: Option<QiFlowAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CureSuggestion {
    pub sector: String,
    pub affliction: String, // e.g. "Star 2 (Sickness)"
    pub cure_name: String, // e.g. "6 Coins"
    pub placement_location: String, // "South-West corner, high up"
    pub success_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QiFlowAnalysis {
    pub flow_path: Vec<String>,
    pub blockages: Vec<String>,
}

// === MAIN ENTRY POINTS ===

/// CLI Entry Point
pub async fn run_feng_shui_cli() -> Result<()> {
    println!("=== QUANTUM FENG SHUI & FLYING STARS SYSTEM (EXPANDED) ===");
    println!("Powered by CURBy Quantum Entropy");
    println!("----------------------------------------------------------");

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

    // Get current date for deep analysis
    let now = chrono::Local::now();

    let config = FengShuiConfig {
        birth_year,
        gender,
        construction_year,
        facing_degrees,
        current_year: Some(now.year()),
        current_month: Some(now.month()),
        current_day: Some(now.day()),
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

    println!("\n[ ANNUAL FLYING STAR CHART ({}) ]", report.annual_chart.label);
    println!("Facing: {} | Sitting: {}", report.annual_chart.facing_mountain, report.annual_chart.sitting_mountain);
    println!("Format: [Base | Mtn | Wtr | Ann]");
    print_chart(&report.annual_chart);

    if let Some(m_chart) = &report.monthly_chart {
        println!("\n[ MONTHLY FLYING STARS ({}) ]", m_chart.label);
        println!("Format: [Base(Month) | Annual | Month]");
        print_visiting_chart(m_chart);
    }

    if let Some(d_chart) = &report.daily_chart {
        println!("\n[ DAILY FLYING STARS ({}) ]", d_chart.label);
        println!("Format: [Base(Day) | Month | Day]");
        print_visiting_chart(d_chart);
    }

    println!("\n[ SPECIAL FORMATIONS ]");
    if report.formations.is_empty() {
        println!("None detected.");
    } else {
        for f in &report.formations {
            println!("* {}", f);
        }
    }

    println!("\n[ QUANTUM ANALYSIS ]");
    println!("Qi Volatility Index: {:.2}", report.quantum.volatility_index);
    println!("Quantum Focus Sector: {}", report.quantum.focus_sector);

    if let Some(flow) = &report.quantum.qi_flow {
        println!("Qi Flow Path: {}", flow.flow_path.join(" -> "));
        if !flow.blockages.is_empty() {
            println!("Blockages Detected: {:?}", flow.blockages);
        }
    }

    if !report.quantum.anomalies.is_empty() {
        println!("Anomalies Detected: {:?}", report.quantum.anomalies);
    }

    println!("\n[ QUANTUM CURE SUGGESTIONS ]");
    for cure in &report.quantum.suggested_cures {
        println!("* Sector: {}", cure.sector);
        println!("  Affliction: {}", cure.affliction);
        println!("  Suggested Cure: {}", cure.cure_name);
        println!("  Placement: {}", cure.placement_location);
        println!("  Simulated Efficacy: {:.1}%", cure.success_probability * 100.0);
        println!("  ---");
    }

    println!("\n[ GENERAL ADVICE ]");
    for tip in &report.advice {
        println!("* {}", tip);
    }

    Ok(())
}

fn print_chart(chart: &FlyingStarChart) {
    for palace in &chart.palaces {
        println!("  {:<12}: [ {} | {} | {} | {} ]",
            palace.sector, palace.base_star, palace.mountain_star, palace.water_star, palace.visiting_star);
    }
}

fn print_visiting_chart(chart: &FlyingStarChart) {
    for palace in &chart.palaces {
         // visiting_star here is the Month/Day star.
         // base_star is likely the period star (or annual for month chart).
         // We can simplify:
         println!("  {:<12}: [ Vis: {} ]", palace.sector, palace.visiting_star);
    }
}

/// Core Logic Handler (Shared by CLI and Server)
pub async fn generate_report(config: FengShuiConfig) -> Result<FengShuiReport> {
    // 1. Fetch Entropy
    let mut client = CurbyClient::new();
    let entropy = client.fetch_bulk_randomness(4096).await?;
    let session = SimulationSession::new(entropy);

    // 2. Calculate Kua
    let kua_profile = if let (Some(y), Some(g)) = (config.birth_year, &config.gender) {
        Some(calculate_kua_profile(y, g))
    } else {
        None
    };

    // 3. Calculate Charts
    let current_year = config.current_year.unwrap_or_else(|| chrono::Local::now().year());
    let current_month = config.current_month.unwrap_or_else(|| chrono::Local::now().month());
    let current_day = config.current_day.unwrap_or_else(|| chrono::Local::now().day());

    // Annual (Base Chart + Annual Star)
    let annual_chart = calculate_flying_star_chart(config.construction_year, config.facing_degrees, current_year);

    // Monthly
    let monthly_chart = calculate_monthly_chart(current_year, current_month);

    // Daily
    let daily_chart = calculate_daily_chart(current_year, current_month, current_day);

    // 4. Analyze Formations
    let formations = analyze_formations(&annual_chart);

    // 5. Quantum Simulation
    let quantum = run_quantum_analysis(&session, &annual_chart, monthly_chart.as_ref(), config.intention.as_deref());

    // 6. Generate Advice
    let advice = generate_advice(&annual_chart, &kua_profile, &quantum, &formations);

    Ok(FengShuiReport {
        kua: kua_profile,
        annual_chart,
        monthly_chart,
        daily_chart,
        formations,
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
// (Kept as is, verified correct)
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

    let dirs = match k {
        1 => vec![("SE", "Sheng Chi"), ("E", "Tian Yi"), ("S", "Yan Nian"), ("N", "Fu Wei")],
        2 => vec![("NE", "Sheng Chi"), ("W", "Tian Yi"), ("NW", "Yan Nian"), ("SW", "Fu Wei")],
        3 => vec![("S", "Sheng Chi"), ("N", "Tian Yi"), ("SE", "Yan Nian"), ("E", "Fu Wei")],
        4 => vec![("N", "Sheng Chi"), ("S", "Tian Yi"), ("E", "Yan Nian"), ("SE", "Fu Wei")],
        6 => vec![("W", "Sheng Chi"), ("NE", "Tian Yi"), ("SW", "Yan Nian"), ("NW", "Fu Wei")],
        7 => vec![("NW", "Sheng Chi"), ("SW", "Tian Yi"), ("NE", "Yan Nian"), ("W", "Fu Wei")],
        8 => vec![("SW", "Sheng Chi"), ("NW", "Tian Yi"), ("W", "Yan Nian"), ("NE", "Fu Wei")],
        9 => vec![("E", "Sheng Chi"), ("SE", "Tian Yi"), ("N", "Yan Nian"), ("S", "Fu Wei")],
        _ => vec![],
    };

    KuaProfile {
        number: k,
        group,
        element,
        lucky_directions: dirs.into_iter().map(|(a,b)| (a.to_string(), b.to_string())).collect(),
    }
}

// --- FLYING STARS LOGIC (ANNUAL/BASE) ---

pub fn calculate_flying_star_chart(construction_year: i32, degrees: f64, current_year: i32) -> FlyingStarChart {
    let period = get_period(construction_year);
    let (facing_sector, facing_mountain_idx, _) = get_24_mountain(degrees);
    let (sitting_sector, sitting_mountain_idx, _) = get_24_mountain((degrees + 180.0) % 360.0);

    let facing_label = format!("{} ({})", facing_sector, get_mountain_name(&facing_sector, facing_mountain_idx));
    let sitting_label = format!("{} ({})", sitting_sector, get_mountain_name(&sitting_sector, sitting_mountain_idx));

    let base_chart = fly_stars(period, true);

    let sector_map = |s: &str| match s {
        "Center" => 0, "NW" => 1, "W" => 2, "NE" => 3, "S" => 4,
        "N" => 5, "SW" => 6, "E" => 7, "SE" => 8, _ => 0
    };

    let sit_base_star = base_chart[sector_map(&sitting_sector)];
    let face_base_star = base_chart[sector_map(&facing_sector)];

    let mtn_flight_pol = get_flight_polarity(sit_base_star, sitting_mountain_idx);
    let mtn_chart = fly_stars(sit_base_star, mtn_flight_pol);

    let wtr_flight_pol = get_flight_polarity(face_base_star, facing_mountain_idx);
    let wtr_chart = fly_stars(face_base_star, wtr_flight_pol);

    // Annual Star Calculation
    let annual_star = calculate_annual_star(current_year);
    let annual_chart = fly_stars(annual_star, true);

    let sectors_ordered = vec!["Center", "NW", "W", "NE", "S", "N", "SW", "E", "SE"];
    let mut palaces = Vec::new();
    for i in 0..9 {
        palaces.push(Palace {
            sector: sectors_ordered[i].to_string(),
            base_star: base_chart[i],
            mountain_star: mtn_chart[i],
            water_star: wtr_chart[i],
            visiting_star: annual_chart[i],
        });
    }

    FlyingStarChart {
        period,
        label: format!("Period {} / Annual {}", period, annual_star),
        facing_mountain: facing_label,
        sitting_mountain: sitting_label,
        palaces,
    }
}

// --- NEW MONTHLY/DAILY LOGIC ---

pub fn calculate_monthly_chart(year: i32, month: u32) -> Option<FlyingStarChart> {
    // 1. Determine Year Branch (Zodiac) to find monthly starting star
    // Simplified Zodiac: 2024 is Dragon. 2024 % 12 -> 8 (Dragon is 4? No. Rat is 0/1900/1984/2020)
    // 1900: Rat.
    // (year - 1900) % 12: 0=Rat, 1=Ox, 2=Tiger, 3=Rabbit, 4=Dragon, 5=Snake, 6=Horse, 7=Goat, 8=Monkey, 9=Rooster, 10=Dog, 11=Pig

    let offset = (year - 1900).rem_euclid(12);
    // Groups:
    // A: Rat(0), Horse(6), Rabbit(3), Rooster(9) -> Start Star 8 (for Month 1 - Tiger Month/Feb)
    // B: Ox(1), Goat(7), Dragon(4), Dog(10) -> Start Star 5
    // C: Tiger(2), Monkey(8), Snake(5), Pig(11) -> Start Star 2

    let start_star = if [0, 6, 3, 9].contains(&offset) { 8 }
    else if [1, 7, 4, 10].contains(&offset) { 5 }
    else { 2 };

    // Chinese Month usually starts Feb 4 (Tiger Month).
    // If we are in Gregorian month M, the Chinese Month is roughly M-1 (if M>1) or 12 (if M=1).
    // Very simplified offset: Month index = (month + 10) % 12?
    // Feb -> Month 1. Jan -> Month 12 (of prev year).
    // Let's assume standard Gregorian -> Chinese Month approximation for "Start of Month".
    // Month 1 starts in Feb.
    let chinese_month_idx = if month == 1 { 12 } else { month - 1 };

    // Monthly Stars fly backwards? No, usually forward annually, but monthly?
    // Standard rule: Monthly stars fly FORWARD (Ascending) in Yang years?
    // Actually, simple rule: Just count backwards from the Start Star?
    // Formula: Star = StartStar - (MonthIdx - 1).
    // 8 -> 7 -> 6... (Descending)
    // Wait, let's verify.
    // Rat Year (1984): Month 1 (Tiger) -> Star 8. Month 2 -> Star 7.
    // So Monthly stars fly REVERSE (Descending) from the starting star.

    // Except... is it descending?
    // Sources say: Rat/Horse/Rabbit/Rooster years -> Feb is 8. March 7. April 6. YES, Descending.

    let mut ruling_star = start_star - (chinese_month_idx as i32 - 1);
    while ruling_star < 1 { ruling_star += 9; }
    while ruling_star > 9 { ruling_star -= 9; }

    let chart_nums = fly_stars(ruling_star, true); // The stars INSIDE the chart fly forward usually.
    // Wait, the "Movement" of the ruling star changes, but once ruling star is in Center, the flight path is standard forward?
    // YES. Luo Shu path is always used.

    let sectors_ordered = vec!["Center", "NW", "W", "NE", "S", "N", "SW", "E", "SE"];
    let mut palaces = Vec::new();
    for i in 0..9 {
        palaces.push(Palace {
            sector: sectors_ordered[i].to_string(),
            base_star: 0,
            mountain_star: 0,
            water_star: 0,
            visiting_star: chart_nums[i],
        });
    }

    Some(FlyingStarChart {
        period: ruling_star,
        label: format!("Month {}", month),
        facing_mountain: "-".to_string(),
        sitting_mountain: "-".to_string(),
        palaces,
    })
}

pub fn calculate_daily_chart(year: i32, month: u32, day: u32) -> Option<FlyingStarChart> {
    // Simplified Solstice Logic
    // Winter Solstice (~Dec 21) -> Ascending.
    // Summer Solstice (~Jun 21) -> Descending.

    let d = NaiveDate::from_ymd_opt(year, month, day)?;
    let winter_solstice = NaiveDate::from_ymd_opt(year, 12, 21)?;
    let summer_solstice = NaiveDate::from_ymd_opt(year, 6, 21)?;

    // Determine cycle.
    // If date is between Summer and Winter -> Yin (Descending).
    // Else -> Yang (Ascending).

    let is_yin = (d >= summer_solstice) && (d < winter_solstice);

    // Calculate days since Solstice
    let days_diff = if is_yin {
        (d - summer_solstice).num_days()
    } else {
        // Handle year wrap for Winter Solstice
        let ws_prev = NaiveDate::from_ymd_opt(if month < 6 { year - 1 } else { year }, 12, 21)?;
        (d - ws_prev).num_days()
    };

    let base_star = if is_yin {
        // Summer Solstice starts at 9, Descending.
        // Star = 9 - (days % 9)
        let mut s = 9 - (days_diff % 9);
        if s < 1 { s += 9; }
        s as i32
    } else {
        // Winter Solstice starts at 1, Ascending.
        // Star = 1 + (days % 9)
        let mut s = 1 + (days_diff % 9);
        if s > 9 { s -= 9; }
        s as i32
    };

    let chart_nums = fly_stars(base_star, true);

    let sectors_ordered = vec!["Center", "NW", "W", "NE", "S", "N", "SW", "E", "SE"];
    let mut palaces = Vec::new();
    for i in 0..9 {
        palaces.push(Palace {
            sector: sectors_ordered[i].to_string(),
            base_star: 0,
            mountain_star: 0,
            water_star: 0,
            visiting_star: chart_nums[i],
        });
    }

    Some(FlyingStarChart {
        period: base_star,
        label: format!("Day {} ({})", day, if is_yin { "Yin" } else { "Yang" }),
        facing_mountain: "-".to_string(),
        sitting_mountain: "-".to_string(),
        palaces,
    })
}

// --- FORMATION ANALYSIS ---

pub fn analyze_formations(chart: &FlyingStarChart) -> Vec<String> {
    let mut formations = Vec::new();

    // Check Sum of Ten
    // Sum of Base + Facing (Water) = 10 (Health/Relationship luck)
    // Sum of Base + Sitting (Mountain) = 10 (Wealth luck)
    // Actually, usually it's Base+Water=10 across ALL sectors (Sum of Ten Chart).

    let mut sum_ten_water = true;
    let mut sum_ten_mountain = true;
    let mut pearl_string = true;
    let mut parent_string = true;

    for p in &chart.palaces {
        if p.base_star + p.water_star != 10 { sum_ten_water = false; }
        if p.base_star + p.mountain_star != 10 { sum_ten_mountain = false; }

        // Parent String: Stars are (1,4,7), (2,5,8), or (3,6,9) in each palace
        let stars = vec![p.base_star, p.mountain_star, p.water_star];
        let mods: Vec<i32> = stars.iter().map(|s| s % 3).collect();
        // If all match mod 3 (0->3,6,9; 1->1,4,7; 2->2,5,8)
        if !(mods[0] == mods[1] && mods[1] == mods[2]) {
            parent_string = false;
        }

        // Pearl String: Consecutive numbers (e.g., 2,3,4)
        let mut sorted = stars.clone();
        sorted.sort();
        if !((sorted[1] == sorted[0] + 1) && (sorted[2] == sorted[1] + 1)) {
            pearl_string = false;
        }
    }

    if sum_ten_water { formations.push("Sum of Ten (Water): Great wealth potential.".to_string()); }
    if sum_ten_mountain { formations.push("Sum of Ten (Mountain): Great health/relationship potential.".to_string()); }
    if parent_string { formations.push("Parent String: Auspicious connectivity across all sectors.".to_string()); }
    if pearl_string { formations.push("Pearl String: Smooth Qi flow.".to_string()); }

    // Seven Star Robbery (Simplified check: Double Stars at Facing or Sitting)
    // if facing/sitting palace has mountain=water?

    formations
}

// --- QUANTUM SIMULATION ---

fn run_quantum_analysis(session: &SimulationSession, chart: &FlyingStarChart, monthly: Option<&FlyingStarChart>, intention: Option<&str>) -> QuantumAnalysis {
    let sectors = vec![
        "North", "North-East", "East", "South-East", "South",
        "South-West", "West", "North-West", "Center"
    ];

    // 1. Focus Sector
    let report = session.simulate_decision(
        &sectors.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        5000 // Deep scan
    );
    let volatility = (report.anomalies.len() as f64 * 0.1).min(1.0);

    // 2. Intention Resonance
    let resonance = if let Some(_intent) = intention {
        let res_report = session.simulate_decision(
            &sectors.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            1000
        );
        Some(res_report.winner)
    } else {
        None
    };

    // 3. Cure Suggestions
    let mut cures = Vec::new();
    for p in &chart.palaces {
        // Identify bad stars (2 and 5)
        let bad_stars = [2, 5];
        let mut affliction = String::new();

        if bad_stars.contains(&p.mountain_star) { affliction.push_str(&format!("Mtn Star {}, ", p.mountain_star)); }
        if bad_stars.contains(&p.water_star) { affliction.push_str(&format!("Wtr Star {}, ", p.water_star)); }
        if bad_stars.contains(&p.visiting_star) { affliction.push_str(&format!("Ann Star {}, ", p.visiting_star)); }

        if let Some(m) = monthly {
            // Find visiting star in monthly chart for this sector
            if let Some(mp) = m.palaces.iter().find(|xp| xp.sector == p.sector) {
                if bad_stars.contains(&mp.visiting_star) { affliction.push_str(&format!("Mth Star {}", mp.visiting_star)); }
            }
        }

        if !affliction.is_empty() {
            // Use entropy to select cure and placement
            let cure_options = match 5 { // Generic metal cures for 2/5
                _ => vec!["6 Coins", "Metal Windchime", "Salt Water Cure", "Brass Wu Lou", "Heavy Metal Object"],
            };

            // Random selection via entropy (simulated by just picking from entropy buffer ideally, but here using session decision)
            let cure_sim = session.simulate_decision(
                &cure_options.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                100
            );

            // Random placement
            let placements = vec!["Corner", "Center of Wall", "Hidden", "Visible", "High Up", "Low Down"];
             let place_sim = session.simulate_decision(
                &placements.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                50
            );

            // Simulate Efficacy (0.0 - 1.0)
            // Use the winning percentage or just a derived value from anomalies
            let efficacy = 0.5 + (report.anomalies.len() % 50) as f64 * 0.01;

            cures.push(CureSuggestion {
                sector: p.sector.clone(),
                affliction: affliction.trim_end_matches(", ").to_string(),
                cure_name: cure_sim.winner,
                placement_location: place_sim.winner,
                success_probability: efficacy,
            });
        }
    }

    // 4. Qi Flow
    // Simulate a path 1 -> 9 through the chart sectors
    let mut flow_path = Vec::new();
    let mut blockages = Vec::new();
    let star_locs: HashMap<i32, String> = chart.palaces.iter().map(|p| (p.base_star, p.sector.clone())).collect();

    for i in 1..=9 {
        if let Some(sec) = star_locs.get(&i) {
            flow_path.push(format!("{}({})", sec, i));
            // Check for blockage using entropy simulation
            // If the sector is the "loser" of a simulation, maybe it's blocked?
            // Or just random check.
             if session.simulate_decision(&vec!["Flow", "Block"].iter().map(|s| s.to_string()).collect::<Vec<_>>(), 10).winner == "Block" {
                 blockages.push(sec.clone());
             }
        }
    }

    QuantumAnalysis {
        volatility_index: volatility,
        focus_sector: report.winner,
        anomalies: report.anomalies,
        intention_resonance: resonance,
        suggested_cures: cures,
        qi_flow: Some(QiFlowAnalysis { flow_path, blockages }),
    }
}

fn generate_advice(chart: &FlyingStarChart, kua: &Option<KuaProfile>, quantum: &QuantumAnalysis, formations: &Vec<String>) -> Vec<String> {
    let mut advice = Vec::new();

    let wealth_star = if chart.period == 9 { 9 } else { 8 };

    for p in &chart.palaces {
        if p.water_star == wealth_star {
            advice.push(format!("Sector {} contains the Water Star {}, activating Wealth Luck.", p.sector, wealth_star));
        }
        if p.mountain_star == wealth_star {
            advice.push(format!("Sector {} contains the Mountain Star {}, good for Health/Relations.", p.sector, wealth_star));
        }
    }

    if let Some(k) = kua {
        advice.push(format!("Your Life Gua is {}. Strongest direction: {}.", k.number, k.lucky_directions[0].0));
    }

    advice.push(format!("Quantum Focus: {}. Volatility: {:.2}", quantum.focus_sector, quantum.volatility_index));

    if !formations.is_empty() {
        advice.push("Special Auspicious Formations detected! See report details.".to_string());
    }

    advice
}

// === UTILS ===

fn get_period(year: i32) -> i32 {
    match year {
        y if y < 1864 => 1,
        y if y <= 1883 => 1,
        y if y <= 1903 => 2,
        y if y <= 1923 => 3,
        y if y <= 1943 => 4,
        y if y <= 1963 => 5,
        y if y <= 1983 => 6,
        y if y <= 2003 => 7,
        y if y <= 2023 => 8,
        _ => 9,
    }
}

fn calculate_annual_star(year: i32) -> i32 {
    let mut sum = 0;
    let digits: Vec<u32> = year.to_string().chars().filter_map(|c| c.to_digit(10)).collect();
    for d in digits { sum += d as i32; }
    while sum > 9 {
        let s_str = sum.to_string();
        sum = 0;
        for c in s_str.chars() { sum += c.to_digit(10).unwrap() as i32; }
    }
    let mut star = 11 - sum;
    while star > 9 { star -= 9; }
    if star == 0 { star = 9; }
    star
}

fn fly_stars(center_star: i32, forward: bool) -> Vec<i32> {
    let mut chart = vec![0; 9];
    let mut current = center_star;
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
    if star == 5 { return true; }
    let pattern_odd = [true, false, false];  // + - -
    let pattern_even = [false, true, true];  // - + +
    let is_even = star % 2 == 0;
    if is_even { pattern_even[house_mtn_idx - 1] } else { pattern_odd[house_mtn_idx - 1] }
}

fn get_24_mountain(deg: f64) -> (String, usize, bool) {
    let d = (deg % 360.0 + 360.0) % 360.0;
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
    ("N".to_string(), 2, false)
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
