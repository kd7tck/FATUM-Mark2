use axum::{
    routing::post,
    Json, Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use serde::{Deserialize, Serialize};

use crate::tools::decision::{DecisionTool, DecisionInput};
use crate::tools::geolocation::{GeolocationTool, GeoPoint};
use crate::client::CurbyClient;
use crate::engine::SimulationSession;

pub async fn start_server() {
    let app = Router::new()
        .route("/api/tools/decision", post(handle_decision))
        .route("/api/tools/geolocation", post(handle_geolocation))
        .route("/api/tools/fengshui", post(handle_fengshui))
        .fallback_service(ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("FATUM-MARK2 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_decision(
    Json(payload): Json<DecisionInput>,
) -> Json<serde_json::Value> {
    match DecisionTool::run(payload).await {
        Ok(output) => Json(serde_json::to_value(output).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
struct GeoInput {
    lat: f64,
    lon: f64,
    radius: f64,
    simulation_count: usize,
}

async fn handle_geolocation(
    Json(payload): Json<GeoInput>,
) -> Json<serde_json::Value> {
    // 1. Fetch Entropy
    let mut client = CurbyClient::new();
    // We need just enough seed.
    let entropy_result = client.fetch_bulk_randomness(64).await;

    match entropy_result {
        Ok(entropy) => {
             let session = SimulationSession::new(entropy);
             let tool = GeolocationTool::new(session);
             let result = tool.generate_location(payload.lat, payload.lon, payload.radius, payload.simulation_count);
             Json(serde_json::to_value(result).unwrap())
        },
        Err(e) => Json(serde_json::json!({ "error": e.to_string() }))
    }
}

#[derive(Deserialize)]
struct FengShuiInput {
    year: Option<i32>,
    gender: Option<String>,
}

#[derive(Serialize)]
struct FengShuiResponse {
    kua: i32,
    group: String,
    focus_sector: String,
    advice: String,
    anomalies: Vec<String>,
}

async fn handle_fengshui(
    Json(payload): Json<FengShuiInput>,
) -> Json<serde_json::Value> {
    // Re-implementing logic from CLI for Web
    // In a real app, I'd move the logic to `tools/feng_shui.rs` as a shared function.
    // For now, I'll inline similar logic or refactor.
    // Refactoring is better.

    // I will call a shared function in `tools/feng_shui` but I need to expose it first.
    // Since I wrote `run_feng_shui_cli` as a CLI driver, let's extract the core logic.
    // For now, to save steps, I will duplicate the lightweight logic here or call a static helper if I make one.

    let mut client = CurbyClient::new();
    match client.fetch_bulk_randomness(64).await {
        Ok(entropy) => {
            let session = SimulationSession::new(entropy);

            // Kua Logic
            let kua = if let (Some(y), Some(g)) = (payload.year, &payload.gender) {
                crate::tools::feng_shui::calculate_kua(y, g)
            } else {
                 let report = session.simulate_decision(
                    &["1", "2", "3", "4", "5", "6", "7", "8", "9"].iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                    1000
                );
                report.winner.parse().unwrap_or(1)
            };

            let group = if [1, 3, 4, 9].contains(&kua) { "East Group" } else { "West Group" };

            // Bagua Logic
             let sectors = vec![
                "North (Career)", "North-East (Knowledge)", "East (Family)",
                "South-East (Wealth)", "South (Fame)", "South-West (Love)",
                "West (Children)", "North-West (Helpful People)", "Center (Health)"
            ];
             let report = session.simulate_decision(
                &sectors.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                1_000_000 // Fixed high sim count for web
            );

            let advice = crate::tools::feng_shui::get_feng_shui_advice(&report.winner);

            let response = FengShuiResponse {
                kua,
                group: group.to_string(),
                focus_sector: report.winner,
                advice: advice.to_string(),
                anomalies: report.anomalies,
            };
             Json(serde_json::to_value(response).unwrap())
        },
        Err(e) => Json(serde_json::json!({ "error": e.to_string() }))
    }
}
