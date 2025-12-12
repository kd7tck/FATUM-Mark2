use chrono::{NaiveDate, Datelike};
use crate::tools::chinese_meta::{is_six_clash, is_six_combination, get_branch};
use crate::tools::astronomy::get_solar_term;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DateSelectionConfig {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub intention: Option<String>,
    pub activities: Option<Vec<String>>, // List of desired activities
    pub user_birth_year: Option<i32>, // Personalized Mode
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuspiciousDate {
    pub date: NaiveDate,
    pub score: i32,
    pub summary: String,
    pub officer: String, // 12 Day Officer
    pub suitable_activities: Vec<String>,
    pub collision: Option<String>, // e.g. "Year Breaker"
}

// 12 Day Officers (Jian Chu)
const OFFICERS: [&str; 12] = [
    "Jian (Establish)", "Chu (Remove)", "Man (Full)", "Ping (Balance)",
    "Ding (Stable)", "Zhi (Initiate)", "Po (Destruction)", "Wei (Danger)",
    "Cheng (Success)", "Shou (Receive)", "Kai (Open)", "Bi (Close)"
];

pub fn calculate_auspiciousness(config: DateSelectionConfig) -> Result<Vec<AuspiciousDate>, String> {
    let mut results = Vec::new();
    let mut current = config.start_date;

    while current <= config.end_date {
        let (score, summary, collision, officer, suitable) = evaluate_day(
            current,
            &config.intention,
            &config.activities,
            config.user_birth_year
        );

        // Filter: only show days with neutral or positive score, unless it's a critical clash
        // OR if the user asked for a specific activity that is suitable
        if score >= 0 {
            results.push(AuspiciousDate {
                date: current,
                score,
                summary,
                officer,
                suitable_activities: suitable,
                collision,
            });
        }

        current = current.succ_opt().ok_or("Date out of range")?;
    }

    Ok(results)
}

fn evaluate_day(
    date: NaiveDate,
    _intention: &Option<String>,
    activities: &Option<Vec<String>>,
    user_year: Option<i32>
) -> (i32, String, Option<String>, String, Vec<String>) {
    let y_branch = get_year_branch_idx(date.year());
    let d_branch = get_day_branch_idx(date);
    let m_branch = get_month_branch_idx(date);

    let mut score = 50; // Base score
    let mut notes = Vec::new();
    let mut collision = None;
    let mut suitable_acts = Vec::new();

    // 1. General Mode Checks (Year Breaker)
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

    // 3. 12 Day Officers (Jian Chu)
    // Formula: Jian is day branch == month branch.
    // Offset = (day_branch - month_branch) mod 12
    let officer_idx = (d_branch as i32 - m_branch as i32).rem_euclid(12) as usize;
    let officer_name = OFFICERS[officer_idx].to_string();

    // Officer Scoringa & Activities
    match officer_idx {
        0 => { // Jian (Establish) - Neutral/Mixed
            notes.push("Jian: Good for starting, bad for burial/travel".to_string());
            suitable_acts.push("Business Opening".to_string());
            suitable_acts.push("Proposal".to_string());
        },
        1 => { // Chu (Remove) - Good for cleansing
            score += 10;
            notes.push("Chu: Good for cleansing/repairs".to_string());
            suitable_acts.push("Medical Procedure".to_string());
            suitable_acts.push("Cleaning".to_string());
            suitable_acts.push("Repairs".to_string());
        },
        2 => { // Man (Full) - Good
            score += 20;
            notes.push("Man: Abundance".to_string());
            suitable_acts.push("Marriage".to_string());
            suitable_acts.push("Opening Business".to_string());
            suitable_acts.push("Signing Contracts".to_string());
        },
        3 => { // Ping (Balance) - Neutral
            notes.push("Ping: Balanced outcomes".to_string());
            suitable_acts.push("Negotiation".to_string());
            suitable_acts.push("Travel".to_string());
        },
        4 => { // Ding (Stable) - Good
            score += 15;
            notes.push("Ding: Long-lasting stability".to_string());
            suitable_acts.push("Marriage".to_string());
            suitable_acts.push("Moving House".to_string());
            suitable_acts.push("Medical Procedure".to_string());
        },
        5 => { // Zhi (Initiate/Hold) - Mixed
            notes.push("Zhi: Good for internal affairs".to_string());
            suitable_acts.push("Internal Meetings".to_string());
        },
        6 => { // Po (Destruction) - Bad
            score -= 30;
            notes.push("Po: Depleting Energy".to_string());
            suitable_acts.push("Demolition".to_string());
            if collision.is_none() { collision = Some("Destruction Day".to_string()); }
        },
        7 => { // Wei (Danger) - Bad
            score -= 20;
            notes.push("Wei: Unstable".to_string());
            suitable_acts.push("Worship".to_string());
        },
        8 => { // Cheng (Success) - Very Good
            score += 30;
            notes.push("Cheng: Success in all things".to_string());
            suitable_acts.push("Marriage".to_string());
            suitable_acts.push("Opening Business".to_string());
            suitable_acts.push("Travel".to_string());
            suitable_acts.push("Moving House".to_string());
        },
        9 => { // Shou (Receive) - Good
            score += 15;
            notes.push("Shou: Good for receiving rewards".to_string());
            suitable_acts.push("Trading".to_string());
            suitable_acts.push("Signing Contracts".to_string());
        },
        10 => { // Kai (Open) - Very Good
            score += 25;
            notes.push("Kai: Open opportunities".to_string());
            suitable_acts.push("Opening Business".to_string());
            suitable_acts.push("Marriage".to_string());
            suitable_acts.push("Travel".to_string());
        },
        11 => { // Bi (Close) - Mixed/Bad
            score -= 10;
            notes.push("Bi: Obstruction".to_string());
            suitable_acts.push("Burial".to_string());
            suitable_acts.push("Closing Deal".to_string());
        },
        _ => {}
    }

    // Intention/Activity Matching
    if let Some(user_acts) = activities {
        for act in user_acts {
            if suitable_acts.iter().any(|s| s.to_lowercase().contains(&act.to_lowercase())) {
                score += 15;
                notes.push(format!("Good for {}", act));
            } else if officer_idx == 6 || officer_idx == 7 { // Po or Wei
                score -= 20;
                notes.push(format!("Avoid {} today", act));
            }
        }
    }

    // Legacy Intention Check
    if let Some(intent) = _intention {
        if !intent.is_empty() {
             if suitable_acts.iter().any(|s| s.to_lowercase().contains(&intent.to_lowercase())) {
                score += 15;
                notes.push("Matches Intention".to_string());
            }
        }
    }

    (score, notes.join(", "), collision, officer_name, suitable_acts)
}

fn get_year_branch_idx(year: i32) -> usize {
    // 1924 = Rat (0).
    let offset = (year - 1924).rem_euclid(12);
    offset as usize
}

fn get_day_branch_idx(date: NaiveDate) -> usize {
    // Reference: Jan 1 2000 was Saturday.
    // Jan 1 2000 was Wu Wu (Earth Horse). Horse = 6.
    let base2000 = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let days = (date - base2000).num_days();
    let idx = (6 + days).rem_euclid(12);
    idx as usize
}

fn get_month_branch_idx(date: NaiveDate) -> usize {
    // Use Astronomy tool to get Solar Term
    let term = get_solar_term(date.year(), date.month(), date.day());

    // Logic derived during planning:
    // Term 23 (Start Rabbit) -> Rabbit (3)
    // Term 0 (Mid Rabbit) -> Rabbit (3)
    // Term 1 (Start Dragon) -> Dragon (4)
    // Formula: ((term + 1) / 2 + 3) % 12

    let idx = ((term + 1) / 2 + 3) % 12;
    idx as usize
}
