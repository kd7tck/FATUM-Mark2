use crate::client::CurbyClient;
use crate::engine::{SimulationSession, TimeStep};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};
use std::fs;

// --- Data Structures ---

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DecisionNode {
    pub id: String,
    pub question: String,
    pub options: Vec<DecisionOption>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DecisionOption {
    pub text: String,
    pub weight: Option<f64>,
    pub next_node_id: Option<String>, // If None, this is a leaf/outcome
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DecisionTree {
    pub root_node_id: String,
    pub nodes: HashMap<String, DecisionNode>,
}

#[derive(Debug, Deserialize)]
pub struct DecisionInput {
    // Simple Mode
    pub options: Option<Vec<String>>,
    pub weights: Option<Vec<f64>>,
    // Tree Mode
    pub tree: Option<DecisionTree>,
    // Common
    pub simulation_count: usize,
}

#[derive(Debug, Serialize)]
pub struct DecisionOutput {
    pub mode: String, // "Simple" or "Tree"
    pub winner: String,
    pub report: String,
    pub distribution: HashMap<String, usize>, // For Simple Mode
    pub anomalies: Vec<String>,
    pub time_series: Vec<TimeStep>, // For Simple Mode

    // Tree Specifics
    pub path_distribution: Option<HashMap<String, usize>>, // Path String -> Count
    pub node_visits: Option<HashMap<String, usize>>, // Node ID -> Count
}

pub struct DecisionTool;

impl DecisionTool {
    pub async fn run(input: DecisionInput) -> Result<DecisionOutput> {
        let mut client = CurbyClient::new();

        // Estimate needed entropy.
        // For tree simulations, paths can be long. We fetch a safe amount.
        // Usually 64 bytes is enough to seed the CSPRNG which generates infinite stream.
        // The engine uses ChaCha20, so we just need a seed.
        let needed_bytes = 64;

        println!("DecisionTool: Fetching quantum entropy seed...");
        let entropy = client.fetch_bulk_randomness(needed_bytes).await?;
        let session = SimulationSession::new(entropy);

        if let Some(tree) = input.tree {
            // Tree Mode
            return Self::run_tree_simulation(&session, tree, input.simulation_count);
        } else if let Some(options) = input.options {
            // Simple Mode
            let weights = input.weights.as_deref();
            let report = session.simulate_decision(&options, weights, input.simulation_count);

            let report_text = format!(
                "Ran {} simulations. The quantum noise patterns favored '{}' with {} hits.",
                report.total_simulations,
                report.winner,
                report.distribution.get(&report.winner).unwrap_or(&0)
            );

            return Ok(DecisionOutput {
                mode: "Simple".to_string(),
                winner: report.winner,
                report: report_text,
                distribution: report.distribution,
                anomalies: report.anomalies,
                time_series: report.time_series,
                path_distribution: None,
                node_visits: None,
            });
        } else {
            return Err(anyhow::anyhow!("Invalid Input: Must provide either 'options' or 'tree'."));
        }
    }

    fn run_tree_simulation(
        session: &SimulationSession,
        tree: DecisionTree,
        count: usize
    ) -> Result<DecisionOutput> {
        // We can't use the simple `simulate_decision` here because that picks 1 of N.
        // We need to walk the tree `count` times.
        // Since `SimulationSession` owns the seed, we can instantiate a local RNG from it.

        use rand::SeedableRng;
        use rand::Rng;
        use rand_chacha::ChaCha20Rng;

        let mut rng = ChaCha20Rng::from_seed(session.seed);

        let mut path_counts: HashMap<String, usize> = HashMap::new();
        let mut node_visits: HashMap<String, usize> = HashMap::new();

        for _ in 0..count {
            let mut current_node_id = tree.root_node_id.clone();
            let mut path_history = Vec::new();
            let mut depth = 0;

            loop {
                depth += 1;
                if depth > 100 { break; } // Prevent infinite loops in cyclic graphs

                *node_visits.entry(current_node_id.clone()).or_insert(0) += 1;

                let node = match tree.nodes.get(&current_node_id) {
                    Some(n) => n,
                    None => break, // Invalid node ID in tree, stop
                };

                if node.options.is_empty() {
                    break; // Dead end
                }

                // Weighted choice for next step
                let mut cdf = Vec::new();
                let mut acc = 0.0;
                let total_weight: f64 = node.options.iter().map(|o| o.weight.unwrap_or(1.0)).sum();

                for opt in &node.options {
                    let w = opt.weight.unwrap_or(1.0);
                    acc += w / total_weight;
                    cdf.push(acc);
                }

                let r: f64 = rng.gen();
                let mut choice_idx = 0;
                for (idx, &threshold) in cdf.iter().enumerate() {
                    if r <= threshold {
                        choice_idx = idx;
                        break;
                    }
                }
                if choice_idx >= node.options.len() { choice_idx = node.options.len() - 1; }

                let chosen_opt = &node.options[choice_idx];
                path_history.push(format!("{}->{}", node.question, chosen_opt.text));

                if let Some(next) = &chosen_opt.next_node_id {
                    current_node_id = next.clone();
                } else {
                    // Leaf reached
                    break;
                }
            }

            let path_str = path_history.join(" | ");
            *path_counts.entry(path_str).or_insert(0) += 1;
        }

        // Determine winner path
        let mut max_count = 0;
        let mut winner = "No Path".to_string();
        for (p, c) in &path_counts {
            if *c > max_count {
                max_count = *c;
                winner = p.clone();
            }
        }

        let report_text = format!(
            "Tree Simulation Complete ({} runs). Most probable path: '{}' ({} hits)",
            count, winner, max_count
        );

        Ok(DecisionOutput {
            mode: "Tree".to_string(),
            winner,
            report: report_text,
            distribution: HashMap::new(), // Not applicable for tree paths in the same way
            anomalies: vec![], // TODO: Implement path anomaly detection
            time_series: vec![],
            path_distribution: Some(path_counts),
            node_visits: Some(node_visits),
        })
    }
}

// === CLI HELPER FUNCTIONS ===

pub async fn run_decision_cli_interactive(initial_options: Option<Vec<String>>, initial_weights: Option<Vec<f64>>, file_path: Option<String>, simulations: usize) -> Result<()> {
    println!("=== QUANTUM DECISION ENGINE ===");
    println!("Powered by CURBy Quantum Entropy");
    println!("----------------------------------------------------------");

    let input = if let Some(path) = file_path {
        // Load tree from file
        println!("Loading Decision Tree from '{}'...", path);
        let content = fs::read_to_string(path)?;
        let tree: DecisionTree = serde_json::from_str(&content)?;
        DecisionInput {
            options: None,
            weights: None,
            tree: Some(tree),
            simulation_count: simulations,
        }
    } else if let Some(opts) = initial_options {
        // Options provided via args
        DecisionInput {
            options: Some(opts),
            weights: initial_weights,
            tree: None,
            simulation_count: simulations,
        }
    } else {
        // Fully Interactive Simple Mode
        println!("Interactive Mode: Define your options.");
        println!("(Enter empty line to finish)");

        let mut options = Vec::new();
        let mut weights = Vec::new();
        let mut use_weights = false;

        // Ask if using weights
        print!("Enable weighted options? (y/n): ");
        io::stdout().flush()?;
        let mut w_buf = String::new();
        io::stdin().read_line(&mut w_buf)?;
        if w_buf.trim().eq_ignore_ascii_case("y") {
            use_weights = true;
        }

        loop {
            print!("Option {}: ", options.len() + 1);
            io::stdout().flush()?;
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;
            let opt = buf.trim().to_string();
            if opt.is_empty() {
                break;
            }
            options.push(opt);

            if use_weights {
                print!("  Weight (default 1.0): ");
                io::stdout().flush()?;
                let mut w_in = String::new();
                io::stdin().read_line(&mut w_in)?;
                let w = w_in.trim().parse::<f64>().unwrap_or(1.0);
                weights.push(w);
            }
        }

        if options.len() < 2 {
            println!("Error: Need at least 2 options.");
            return Ok(());
        }

        DecisionInput {
            options: Some(options),
            weights: if use_weights { Some(weights) } else { None },
            tree: None,
            simulation_count: simulations,
        }
    };

    println!("\nInitializing Quantum Simulation ({} runs)...", input.simulation_count);
    let output = DecisionTool::run(input).await?;

    println!("\n================ REPORT ================");
    println!("Winner: {}", output.winner);
    println!("{}", output.report);

    if output.mode == "Simple" {
        println!("\n[ Distribution ]");
        // Sort by count descending
        let mut dist: Vec<_> = output.distribution.iter().collect();
        dist.sort_by(|a, b| b.1.cmp(a.1));

        for (opt, count) in dist {
            println!("* {:<20}: {}", opt, count);
        }

        if !output.anomalies.is_empty() {
            println!("\n[ Anomalies ]");
            for a in output.anomalies {
                println!("! {}", a);
            }
        }
    } else {
        println!("\n[ Path Analysis ]");
        if let Some(paths) = output.path_distribution {
             let mut p_dist: Vec<_> = paths.iter().collect();
             p_dist.sort_by(|a, b| b.1.cmp(a.1));
             // Show top 5
             for (path, count) in p_dist.iter().take(5) {
                 println!("* [{}] : {}", count, path);
             }
        }
    }

    Ok(())
}
