use axum::{
    routing::post,
    Json, Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use serde::Deserialize;

use crate::tools::decision::{DecisionTool, DecisionInput};
use crate::tools::geolocation::{GeolocationTool, GeoPoint};
use crate::client::CurbyClient;
use crate::engine::SimulationSession;
use crate::tools::feng_shui::{FengShuiConfig, generate_report};

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
struct FengShuiApiInput {
    birth_year: Option<i32>,
    gender: Option<String>,
    construction_year: Option<i32>,
    facing_degrees: Option<f64>,
    intention: Option<String>,
}

async fn handle_fengshui(
    Json(payload): Json<FengShuiApiInput>,
) -> Json<serde_json::Value> {
    let now = chrono::Local::now();
    use chrono::Datelike;
    let config = FengShuiConfig {
        birth_year: payload.birth_year,
        gender: payload.gender,
        construction_year: payload.construction_year.unwrap_or(2024),
        facing_degrees: payload.facing_degrees.unwrap_or(180.0), // South default
        current_year: Some(now.year()),
        current_month: Some(now.month()),
        current_day: Some(now.day()),
        intention: payload.intention,
    };

    match generate_report(config).await {
        Ok(report) => Json(serde_json::to_value(report).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}
