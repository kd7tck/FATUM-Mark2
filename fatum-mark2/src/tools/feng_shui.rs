use std::io::{self, Write};
use anyhow::Result;
use crate::client::CurbyClient;
use crate::engine::SimulationSession;

pub async fn run_feng_shui_cli() -> Result<()> {
    println!("=== QUANTUM FENG SHUI CALCULATOR ===");
    println!("Powered by CURBy Quantum Entropy");
    println!("------------------------------------");

    // 1. Gather User Input
    print!("Enter your Birth Year (YYYY) or press Enter to skip: ");
    io::stdout().flush()?;
    let mut year_input = String::new();
    io::stdin().read_line(&mut year_input)?;
    let year_input = year_input.trim();

    print!("Enter your Gender (M/F) [Default: Skip]: ");
    io::stdout().flush()?;
    let mut gender_input = String::new();
    io::stdin().read_line(&mut gender_input)?;
    let gender_input = gender_input.trim().to_uppercase();

    // 2. Fetch Entropy
    println!("\nFetching Quantum Entropy from CURBy...");
    let mut client = CurbyClient::new();
    // Fetch enough bytes for a solid simulation
    let entropy = client.fetch_bulk_randomness(1024).await?;
    let session = SimulationSession::new(entropy);

    // 3. Determine Kua Number (if data provided) or Quantum Kua
    let kua = if !year_input.is_empty() && (gender_input == "M" || gender_input == "F") {
        calculate_kua(year_input.parse().unwrap_or(2000), &gender_input)
    } else {
        println!("Insufficient data for Traditional Kua. Generating QUANTUM KUA...");
        // 1-9
        let report = session.simulate_decision(
            &["1", "2", "3", "4", "5", "6", "7", "8", "9"].iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            1000
        );
        report.winner.parse().unwrap_or(1)
    };

    println!("\nYour Kua Number: {}", kua);
    let group = if [1, 3, 4, 9].contains(&kua) { "East Group" } else { "West Group" };
    println!("Group: {}", group);

    // 4. Quantum Bagua Scan
    println!("\nRunning Quantum Bagua Scan (1,000,000 simulations)...");
    let sectors = vec![
        "North (Career)", "North-East (Knowledge)", "East (Family)",
        "South-East (Wealth)", "South (Fame)", "South-West (Love)",
        "West (Children)", "North-West (Helpful People)", "Center (Health)"
    ];

    // Using the session (seeded PRNG approach or raw cycle depending on engine implementation)
    // To support "millions", we need to ensure the engine handles it.
    let report = session.simulate_decision(
        &sectors.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        1_000_000
    );

    println!("\n=== RESULTS ===");
    println!("Focus Sector (Winner): {}", report.winner);
    println!("Anomaly Detected: {:?}", report.anomalies.first().unwrap_or(&"None".to_string()));

    // Provide advice based on winner
    let advice = get_feng_shui_advice(&report.winner);
    println!("\nAdvice: {}", advice);

    Ok(())
}

pub fn calculate_kua(year: i32, gender: &str) -> i32 {
    let mut sum = 0;
    let digits: Vec<u32> = year.to_string().chars().filter_map(|c| c.to_digit(10)).collect();
    for d in digits { sum += d as i32; }

    // Reduce to single digit
    while sum > 9 {
        let s_str = sum.to_string();
        sum = 0;
        for c in s_str.chars() {
            sum += c.to_digit(10).unwrap() as i32;
        }
    }

    if gender == "M" {
        let mut k = 11 - sum;
        while k > 9 { k -= 9; } // simple reduction
        if k == 0 { k = 9; } // unlikely but safety
        if k == 5 { 2 } else { k } // Male 5 becomes 2
    } else {
        let mut k = 4 + sum;
        while k > 9 { k -= 9; }
        if k == 5 { 8 } else { k } // Female 5 becomes 8
    }
}

pub fn get_feng_shui_advice(sector: &str) -> &'static str {
    if sector.contains("North (Career)") { return "Place a water feature or blue items here to boost career flow."; }
    if sector.contains("North-East") { return "Ideal for meditation or a library. Use earth tones."; }
    if sector.contains("East (Family)") { return "Add healthy plants here for family harmony."; }
    if sector.contains("South-East") { return "The Wealth corner. Place symbols of abundance or a small fountain."; }
    if sector.contains("South (Fame)") { return "Bright lights, red colors, and awards belong here."; }
    if sector.contains("South-West") { return "Pairs of objects (e.g., two candles) promote relationship harmony."; }
    if sector.contains("West (Children)") { return "Metal objects or white colors enhance creativity."; }
    if sector.contains("North-West") { return "Metal wind chimes can attract helpful people."; }
    "Keep the center open and clutter-free for overall health."
}
