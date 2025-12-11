use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::engine::SimulationSession;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexagramData {
    pub number: u32,
    pub name: String,
    pub judgment: String,
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hexagram {
    pub number: u32,
    pub name: String,
    pub lines: Vec<u8>, // 0=Yin, 1=Yang
    pub changing_lines: Vec<usize>, // Indices 0-5
    pub transformed_hexagram: Option<Box<Hexagram>>,
    pub judgment: String,
    pub image: String,
}

pub struct DivinationTool;

impl DivinationTool {
    pub fn cast_hexagram(session: &SimulationSession) -> Result<Hexagram> {
        // Load JSON data
        // In a production env, this should be cached in AppState, but reading from disk is fine here for simplicity.
        let data_str = fs::read_to_string("static/iching.json").unwrap_or_else(|_| "[]".to_string());
        let hex_db: Vec<HexagramData> = serde_json::from_str(&data_str).unwrap_or_default();

        let mut lines = Vec::new();
        let mut changing = Vec::new();
        let mut trans_lines = Vec::new();

        for i in 0..6 {
            let mut sum = 0;
            for _ in 0..3 {
                let toss = session.simulate_decision(&vec!["Head".to_string(), "Tail".to_string()], None, 10).winner;
                sum += if toss == "Head" { 3 } else { 2 };
            }

            let is_yang = sum % 2 != 0;
            let is_changing = sum == 6 || sum == 9;

            lines.push(if is_yang { 1 } else { 0 });
            if is_changing {
                changing.push(i);
                trans_lines.push(if is_yang { 0 } else { 1 });
            } else {
                trans_lines.push(if is_yang { 1 } else { 0 });
            }
        }

        let (orig_num, orig_name) = lookup_hexagram_meta(&lines);
        let orig_data = hex_db.iter().find(|h| h.number == orig_num);
        let judgment = orig_data.map(|d| d.judgment.clone()).unwrap_or_else(|| "Unknown Judgment".to_string());
        let image = orig_data.map(|d| d.image.clone()).unwrap_or_else(|| "Unknown Image".to_string());
        let name_full = orig_data.map(|d| d.name.clone()).unwrap_or(orig_name);

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

// Helper to get number and default name from bits.
// The King Wen sequence mapping from binary is specific.
// We need a map.
fn lookup_hexagram_meta(lines: &[u8]) -> (u32, String) {
    let mut val = 0;
    // Standard binary reading usually bottom-up
    for (i, &bit) in lines.iter().enumerate() {
        if bit == 1 { val |= 1 << i; }
    }

    // Map binary value (0-63) to King Wen Number (1-64)
    // This mapping table converts the binary representation (bottom line = LSB) to King Wen order.
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
