use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::engine::SimulationSession;
use std::fs;

/// Represents the metadata for a single Hexagram from `iching.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexagramData {
    pub number: u32,
    pub name: String,
    pub judgment: String,
    pub image: String,
}

/// Represents the result of a Divination cast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hexagram {
    pub number: u32,
    pub name: String,
    pub lines: Vec<u8>, // 0=Yin, 1=Yang
    pub changing_lines: Vec<usize>, // Indices 0-5 indicating which lines move
    pub transformed_hexagram: Option<Box<Hexagram>>, // The result after changing lines flip
    pub judgment: String,
    pub image: String,
}

pub struct DivinationTool;

impl DivinationTool {
    /// Casts a Hexagram using the Quantum Coin Method.
    ///
    /// Simulates tossing 3 coins 6 times.
    /// - 3 Heads (3+3+3=9) -> Old Yang (Changes to Yin)
    /// - 3 Tails (2+2+2=6) -> Old Yin (Changes to Yang)
    /// - 2 Heads + 1 Tail (3+3+2=8) -> Young Yin (Static)
    /// - 1 Head + 2 Tails (3+2+2=7) -> Young Yang (Static)
    pub fn cast_hexagram(session: &SimulationSession) -> Result<Hexagram> {
        // Load JSON data
        // Ideally cached, but reading here for stateless simplicity.
        let data_str = fs::read_to_string("static/iching.json").unwrap_or_else(|_| "[]".to_string());
        let hex_db: Vec<HexagramData> = serde_json::from_str(&data_str).unwrap_or_default();

        let mut lines = Vec::new();
        let mut changing = Vec::new();
        let mut trans_lines = Vec::new();

        // Build 6 lines (Bottom to Top)
        for i in 0..6 {
            let mut sum = 0;
            for _ in 0..3 {
                // Quantum simulation of a coin toss
                let toss = session.simulate_decision(&vec!["Head".to_string(), "Tail".to_string()], None, 10).winner;
                sum += if toss == "Head" { 3 } else { 2 };
            }

            let is_yang = sum % 2 != 0; // 7 or 9 is Yang
            let is_changing = sum == 6 || sum == 9;

            lines.push(if is_yang { 1 } else { 0 });
            if is_changing {
                changing.push(i);
                trans_lines.push(if is_yang { 0 } else { 1 }); // Flip for transformed
            } else {
                trans_lines.push(if is_yang { 1 } else { 0 }); // Keep static
            }
        }

        // Identify Primary Hexagram
        let (orig_num, orig_name) = lookup_hexagram_meta(&lines);
        let orig_data = hex_db.iter().find(|h| h.number == orig_num);
        let judgment = orig_data.map(|d| d.judgment.clone()).unwrap_or_else(|| "Unknown Judgment".to_string());
        let image = orig_data.map(|d| d.image.clone()).unwrap_or_else(|| "Unknown Image".to_string());
        let name_full = orig_data.map(|d| d.name.clone()).unwrap_or(orig_name);

        // Identify Transformed Hexagram (if any lines changed)
        let transformed = if !changing.is_empty() {
            let (t_num, t_name) = lookup_hexagram_meta(&trans_lines);
            let t_data = hex_db.iter().find(|h| h.number == t_num);
            let t_judgment = t_data.map(|d| d.judgment.clone()).unwrap_or_else(|| "Unknown Judgment".to_string());
            let t_image = t_data.map(|d| d.image.clone()).unwrap_or_else(|| "Unknown Image".to_string());
            let t_name_full = t_data.map(|d| d.name.clone()).unwrap_or(t_name);

            Some(Box::new(Hexagram {
                number: t_num,
                name: t_name_full,
                lines: trans_lines,
                changing_lines: vec![],
                transformed_hexagram: None,
                judgment: t_judgment,
                image: t_image,
            }))
        } else {
            None
        };

        Ok(Hexagram {
            number: orig_num,
            name: name_full,
            lines,
            changing_lines: changing,
            transformed_hexagram: transformed,
            judgment,
            image,
        })
    }
}

/// Converts a 6-bit array (Bottom->Top) to King Wen Hexagram Number.
fn lookup_hexagram_meta(lines: &[u8]) -> (u32, String) {
    let mut val = 0;
    // Pack bits into integer
    for (i, &bit) in lines.iter().enumerate() {
        if bit == 1 { val |= 1 << i; }
    }

    // Map binary pattern (0-63) to King Wen sequence (1-64).
    // The binary index `val` assumes LSB is bottom line.
    let king_wen_map = [
        2, 24, 7, 19, 15, 36, 46, 11,
        16, 51, 40, 54, 62, 55, 32, 34,
        8, 3, 29, 60, 39, 63, 48, 5,
        45, 17, 47, 58, 31, 49, 28, 43,
        23, 27, 4, 41, 52, 22, 18, 26,
        35, 21, 64, 38, 56, 30, 50, 14,
        20, 42, 59, 61, 53, 37, 57, 9,
        12, 25, 6, 10, 33, 13, 44, 1
    ];

    let number = if val < 64 { king_wen_map[val] } else { 0 };
    (number, format!("Hexagram {}", number))
}
