use chrono::{NaiveDate, Datelike};
use crate::tools::chinese_meta::{is_six_clash, is_six_combination, get_branch};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSelectionConfig {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub intention: Option<String>,
    pub user_birth_year: Option<i32>, // Personalized Mode
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
        let (score, summary, collision) = evaluate_day(current, &config.intention, config.user_birth_year);

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

fn evaluate_day(date: NaiveDate, _intention: &Option<String>, user_year: Option<i32>) -> (i32, String, Option<String>) {
    let y_branch = get_year_branch_idx(date.year());
    let d_branch = get_day_branch_idx(date);

    let mut score = 50; // Base score
    let mut notes = Vec::new();
    let mut collision = None;

    // 1. General Mode Checks
    // Check Year Breaker (Sui Po)
    if is_six_clash(y_branch, d_branch) {
        score -= 50;
        collision = Some("Year Breaker (Sui Po)".to_string());
        notes.push("Clash with Year Pillar".to_string());
    }

    // 2. Personalized Mode Checks
    if let Some(uy) = user_year {
        let user_branch = get_year_branch_idx(uy);

        // Personal Breaker (Clash with User Year)
        if is_six_clash(user_branch, d_branch) {
            score -= 40;
            let msg = format!("Clash with your Year ({} vs {})", get_branch(user_branch), get_branch(d_branch));
            notes.push(msg.clone());
            if collision.is_none() { collision = Some(msg); }
        }

        // Personal Combination (Harmony with User Year)
        if is_six_combination(user_branch, d_branch) {
            score += 20;
            notes.push(format!("Harmony with your Year ({} + {})", get_branch(user_branch), get_branch(d_branch)));
        }
    }

    // Intention Check (Mock)
    if let Some(intent) = _intention {
        if intent.to_lowercase().contains("wealth") && (d_branch == 2 || d_branch == 8) { // Arbitrary lucky days
            score += 20;
            notes.push("Good for Wealth".to_string());
        }
    }

    (score, notes.join(", "), collision)
}

fn get_year_branch_idx(year: i32) -> usize {
    // 1924 = Rat (0).
    let offset = (year - 1924).rem_euclid(12);
    offset as usize
}

fn get_day_branch_idx(date: NaiveDate) -> usize {
    // Reference: Jan 1 2000 was Saturday.
    // We need Stem/Branch day cycle.
    // Jan 1 2000 was Wu Wu (Earth Horse). Horse = 6.
    let base2000 = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let days = (date - base2000).num_days();
    let idx = (6 + days).rem_euclid(12); // 6 is Horse index
    idx as usize
}
