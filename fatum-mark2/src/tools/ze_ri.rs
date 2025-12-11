use chrono::{NaiveDate, Datelike};
use crate::tools::astronomy; // Assuming astronomy handles solar terms/stems/branches
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSelectionConfig {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub intention: Option<String>,
    pub user_bazi: Option<String>, // Placeholder for now
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuspiciousDate {
    pub date: NaiveDate,
    pub score: i32,
    pub summary: String,
    pub collision: Option<String>, // e.g. "Year Breaker"
}

pub fn calculate_auspiciousness(config: DateSelectionConfig) -> Result<Vec<AuspiciousDate>, String> {
    let mut results = Vec::new();
    let mut current = config.start_date;

    while current <= config.end_date {
        let (score, summary, collision) = evaluate_day(current, &config.intention);

        // Filter: only show days with neutral or positive score, unless it's a critical clash
        if score >= 0 {
            results.push(AuspiciousDate {
                date: current,
                score,
                summary,
                collision,
            });
        }

        current = current.succ_opt().ok_or("Date out of range")?;
    }

    Ok(results)
}

fn evaluate_day(date: NaiveDate, _intention: &Option<String>) -> (i32, String, Option<String>) {
    // Basic "Mode A" Logic: Check for Year Breaker (Sui Po) and Month Breaker (Yue Po)

    // 1. Get Lunar/Solar Info (Mocking this interaction for now as astronomy integration depends on existing functions)
    // We need the Stem/Branch of the Year and Day.

    // Let's assume a simplified cycle for the "Prototype":
    // 2024 is Jia Chen (Dragon).
    // Dragon (Chen) clashes with Dog (Xu).

    let (_y_stem, y_branch) = get_year_pillar(date.year());
    let (_d_stem, d_branch) = get_day_pillar_simple(date);

    let mut score = 50; // Base score
    let mut notes = Vec::new();
    let mut collision = None;

    // Check Year Breaker (Sui Po)
    if is_clash(y_branch, d_branch) {
        score -= 50;
        collision = Some("Year Breaker (Sui Po)".to_string());
        notes.push("Clash with Year Pillar".to_string());
    }

    // Check Month Breaker (Yue Po) - (omitted for brevity in MVP, needs precise solar term month)

    // Intention Check (Mock)
    if let Some(intent) = _intention {
        if intent.to_lowercase().contains("wealth") && (d_branch == 2 || d_branch == 8) { // Arbitrary lucky days
            score += 20;
            notes.push("Good for Wealth".to_string());
        }
    }

    (score, notes.join(", "), collision)
}

// Helpers (Simplified for MVP, ideally import from astronomy.rs)
// Branches: 0=Rat, 1=Ox, 2=Tiger, ... 11=Pig
fn get_year_pillar(year: i32) -> (u8, u8) {
    // 1984 = Wood Rat (Jia Zi). 1984 % 60 = 4.
    // This is rough.
    // 2024 = Dragon. (2024 - 4) % 12 = 0? No.
    // Rat=0. 2020=Rat.
    // (year - 2020) % 12.
    let offset = (year - 4) % 12; // 2024-4=2020%12 = 4 (Dragon) ?
    // 0=Rat(1984, 1996, 2008, 2020)
    // 1=Ox, 2=Tiger, 3=Rabbit, 4=Dragon.
    // So (year - 4) % 12 for standard mapping is messy.
    // Let's use (year - 1900) % 12. 1900=Rat.
    let branch = ((year - 1900).rem_euclid(12)) as u8;
    (0, branch) // Stem ignored
}

fn get_day_pillar_simple(date: NaiveDate) -> (u8, u8) {
    // Epoch: Jan 1, 1900 was a certain day.
    // Approx calculation for MVP.
    // Actually, let's just use a random seed based on date hash if we don't have full calendar?
    // No, let's allow `astronomy.rs` to handle this if possible.
    // Checking astronomy.rs...

    // For now, return a dummy based on day of year
    let day_idx = date.ordinal0() as u8;
    (0, day_idx % 12)
}

fn is_clash(b1: u8, b2: u8) -> bool {
    // Clashes are 6 positions apart. 0(Rat) vs 6(Horse).
    (b1 as i32 - b2 as i32).abs() == 6
}
