use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::engine::SimulationSession;

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

        let original = lookup_hexagram(&lines);
        let transformed = if !changing.is_empty() {
            let (t_num, t_name) = lookup_hexagram(&trans_lines);
            Some(Box::new(Hexagram {
                number: t_num,
                name: t_name,
                lines: trans_lines,
                changing_lines: vec![],
                transformed_hexagram: None,
                judgment: "Transformed Judgement...".to_string(),
                image: "Transformed Image...".to_string(),
            }))
        } else {
            None
        };

        Ok(Hexagram {
            number: original.0,
            name: original.1,
            lines,
            changing_lines: changing,
            transformed_hexagram: transformed,
            judgment: "Judgement text placeholder...".to_string(),
            image: "Image text placeholder...".to_string(),
        })
    }
}

fn lookup_hexagram(lines: &[u8]) -> (u32, String) {
    let mut val = 0;
    for (i, &bit) in lines.iter().enumerate() {
        if bit == 1 { val |= 1 << i; }
    }
    match val {
        0 => (2, "Kun (The Receptive)".to_string()),
        63 => (1, "Qian (The Creative)".to_string()),
        // In a full implementation, all 64 would be here.
        // For now, we return a generic placeholder for others to compile.
        _ => (0, format!("Hexagram Value {}", val)),
    }
}
