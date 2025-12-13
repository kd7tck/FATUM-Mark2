use axum::{
    routing::{get, post},
    Json, Router, Extension,
    response::{IntoResponse, Response},
    http::{header, StatusCode},
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::services::ServeDir;
use serde::{Deserialize, Serialize};

use crate::client::CurbyClient;
use crate::engine::SimulationSession;
use crate::tools::feng_shui::{FengShuiConfig, generate_report, VirtualCure};
use crate::tools::divination::DivinationTool;
use crate::tools::pdf_generator::generate_pdf;
use crate::tools::ze_ri::{DateSelectionConfig, calculate_auspiciousness};
use crate::tools::zi_wei::{ZiWeiConfig, generate_ziwei_chart};
use crate::tools::da_liu_ren::{DaLiuRenConfig, generate_da_liu_ren};
use crate::tools::entanglement::{EntanglementRequest, calculate_entanglement};
use crate::db::Db;
use crate::services::entropy;

#[derive(Clone)]
pub struct AppState {
    db: Arc<Db>,
}

pub async fn start_server() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:fatum.db".to_string());
    let db = Db::new(&db_url).await.expect("Failed to initialize database");
    let shared_state = AppState { db: Arc::new(db) };

    let app = Router::new()
        .route("/api/tools/fengshui", post(handle_fengshui))
        .route("/api/tools/fengshui/pdf", post(handle_fengshui_pdf))
        .route("/api/tools/divination", post(handle_divination))
        .route("/api/tools/zeri", post(handle_zeri))
        .route("/api/tools/ziwei", post(handle_ziwei))
        .route("/api/tools/daliuren", post(handle_daliuren))
        .route("/api/tools/entanglement", post(handle_entanglement))
        .route("/api/profiles", get(list_profiles).post(create_profile))
        .route("/api/history", get(list_history).post(save_history))
        .route("/api/entropy/batches", get(list_entropy_batches).post(create_entropy_batch))
        .route("/api/entropy/harvest/start", post(start_harvest))
        .route("/api/entropy/harvest/stop", post(stop_harvest))
        .route("/api/entropy/harvest/status", get(harvest_status))
        .fallback_service(ServeDir::new("static"))
        .layer(Extension(shared_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("FATUM-MARK2 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct FengShuiApiInput {
    birth_year: Option<i32>,
    birth_month: Option<u32>,
    birth_day: Option<u32>,
    birth_hour: Option<u32>,
    gender: Option<String>,
    construction_year: Option<i32>,
    facing_degrees: Option<f64>,
    intention: Option<String>,
    quantum_mode: Option<bool>,
    virtual_cures: Option<Vec<VirtualCure>>,
    entropy_batch_id: Option<i64>,
}

async fn handle_fengshui(
    Extension(state): Extension<AppState>,
    Json(payload): Json<FengShuiApiInput>,
) -> Json<serde_json::Value> {
    let now = chrono::Local::now();
    use chrono::Datelike;
    let config = FengShuiConfig {
        birth_year: payload.birth_year,
        birth_month: payload.birth_month,
        birth_day: payload.birth_day,
        birth_hour: payload.birth_hour,
        gender: payload.gender,
        construction_year: payload.construction_year.unwrap_or(2024),
        facing_degrees: payload.facing_degrees.unwrap_or(180.0),
        current_year: Some(now.year()),
        current_month: Some(now.month()),
        current_day: Some(now.day()),
        intention: payload.intention,
        quantum_mode: payload.quantum_mode.unwrap_or(false),
        virtual_cures: payload.virtual_cures,
        entropy_batch_id: payload.entropy_batch_id,
    };

    // Need to pass DB reference to generate_report if using batch
    match generate_report(config, Some(state.db.clone())).await {
        Ok(report) => Json(serde_json::to_value(report).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn handle_fengshui_pdf(
    Extension(state): Extension<AppState>,
    Json(payload): Json<FengShuiApiInput>,
) -> Response {
    let now = chrono::Local::now();
    use chrono::Datelike;
    let config = FengShuiConfig {
        birth_year: payload.birth_year,
        birth_month: payload.birth_month,
        birth_day: payload.birth_day,
        birth_hour: payload.birth_hour,
        gender: payload.gender,
        construction_year: payload.construction_year.unwrap_or(2024),
        facing_degrees: payload.facing_degrees.unwrap_or(180.0),
        current_year: Some(now.year()),
        current_month: Some(now.month()),
        current_day: Some(now.day()),
        intention: payload.intention,
        quantum_mode: payload.quantum_mode.unwrap_or(false),
        virtual_cures: payload.virtual_cures,
        entropy_batch_id: payload.entropy_batch_id,
    };

    match generate_report(config, Some(state.db.clone())).await {
        Ok(report) => {
            match generate_pdf(&report) {
                Ok(pdf_bytes) => {
                    (
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, "application/pdf")],
                        pdf_bytes
                    ).into_response()
                },
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn handle_zeri(
    Json(payload): Json<DateSelectionConfig>,
) -> Json<serde_json::Value> {
    match calculate_auspiciousness(payload) {
        Ok(results) => Json(serde_json::to_value(results).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e })),
    }
}

async fn handle_ziwei(
    Json(payload): Json<ZiWeiConfig>,
) -> Json<serde_json::Value> {
    match generate_ziwei_chart(payload) {
        Ok(chart) => Json(serde_json::to_value(chart).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e })),
    }
}

async fn handle_daliuren(
    Json(payload): Json<DaLiuRenConfig>,
) -> Json<serde_json::Value> {
    match generate_da_liu_ren(payload) {
        Ok(chart) => Json(serde_json::to_value(chart).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e })),
    }
}

async fn handle_divination() -> Json<serde_json::Value> {
    let mut client = CurbyClient::new();
    // Fetch entropy
    if let Ok(entropy) = client.fetch_bulk_randomness(1024).await {
        let session = SimulationSession::new(entropy);
        match DivinationTool::cast_hexagram(&session) {
            Ok(hex) => Json(serde_json::to_value(hex).unwrap()),
            Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
        }
    } else {
        Json(serde_json::json!({ "error": "Failed to fetch entropy" }))
    }
}

async fn handle_entanglement(
    Json(payload): Json<EntanglementRequest>,
) -> Json<serde_json::Value> {
    match calculate_entanglement(&payload) {
        Ok(report) => Json(serde_json::to_value(report).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

// === ENTROPY HANDLERS ===

#[derive(Deserialize)]
struct CreateBatchInput {
    name: String,
}

#[derive(Deserialize)]
struct StartHarvestInput {
    batch_id: i64,
}

async fn list_entropy_batches(
    Extension(state): Extension<AppState>,
) -> Json<serde_json::Value> {
    // We should also get the size for each batch
    match state.db.list_batches().await {
        Ok(batches) => {
            // Enrich with size
            let mut result = Vec::new();
            for b in batches {
                let size = state.db.get_batch_size(b.id).await.unwrap_or(0);
                result.push(serde_json::json!({
                    "id": b.id,
                    "name": b.name,
                    "status": b.status,
                    "created_at": b.created_at,
                    "count": size,
                    // Each pulse is 512 bits = 64 bytes
                    "size_bytes": size * 64
                }));
            }
            Json(serde_json::json!(result))
        },
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn create_entropy_batch(
    Extension(state): Extension<AppState>,
    Json(input): Json<CreateBatchInput>,
) -> Json<serde_json::Value> {
    match state.db.create_batch(&input.name).await {
        Ok(id) => Json(serde_json::json!({ "id": id })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn start_harvest(
    Extension(state): Extension<AppState>,
    Json(input): Json<StartHarvestInput>,
) -> Json<serde_json::Value> {
    entropy::start_harvesting(state.db.clone(), input.batch_id).await;
    Json(serde_json::json!({ "status": "started" }))
}

async fn stop_harvest(
    Extension(state): Extension<AppState>,
) -> Json<serde_json::Value> {
    entropy::stop_harvesting(state.db.clone()).await;
    Json(serde_json::json!({ "status": "stopped" }))
}

async fn harvest_status() -> Json<serde_json::Value> {
    let batch_id = entropy::get_harvest_status().await;
    Json(serde_json::json!({ "active_batch_id": batch_id }))
}

// === DB HANDLERS ===

#[derive(Serialize, Deserialize)]
struct ProfileInput {
    name: String,
    birth_year: i32,
    birth_month: i32,
    birth_day: i32,
    birth_hour: i32,
    gender: String,
}

#[derive(sqlx::FromRow, Serialize)]
struct ProfileRow {
    id: i64,
    name: String,
    birth_year: Option<i64>,
    birth_month: Option<i64>,
    birth_day: Option<i64>,
    birth_hour: Option<i64>,
    gender: Option<String>,
}

async fn create_profile(
    Extension(state): Extension<AppState>,
    Json(input): Json<ProfileInput>,
) -> Json<serde_json::Value> {
    let res = sqlx::query(
        "INSERT INTO profiles (name, birth_year, birth_month, birth_day, birth_hour, gender) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(input.name)
    .bind(input.birth_year)
    .bind(input.birth_month)
    .bind(input.birth_day)
    .bind(input.birth_hour)
    .bind(input.gender)
    .execute(&state.db.pool)
    .await;

    match res {
        Ok(r) => Json(serde_json::json!({ "id": r.last_insert_rowid() })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn list_profiles(
    Extension(state): Extension<AppState>,
) -> Json<serde_json::Value> {
    let res = sqlx::query_as::<_, ProfileRow>("SELECT id, name, birth_year, birth_month, birth_day, birth_hour, gender FROM profiles ORDER BY created_at DESC")
        .fetch_all(&state.db.pool)
        .await;

    match res {
        Ok(rows) => {
             Json(serde_json::json!(rows))
        },
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

#[derive(Serialize, Deserialize)]
struct HistoryInput {
    profile_id: Option<i64>,
    tool_type: String,
    summary: String,
    full_report: serde_json::Value,
}

#[derive(sqlx::FromRow, Serialize)]
struct HistoryRow {
    id: i64,
    tool_type: String,
    summary: Option<String>,
    created_at: Option<chrono::NaiveDateTime>, // or String depending on driver
    profile_name: Option<String>,
}

async fn save_history(
    Extension(state): Extension<AppState>,
    Json(input): Json<HistoryInput>,
) -> Json<serde_json::Value> {
    let res = sqlx::query(
        "INSERT INTO history (profile_id, tool_type, summary, full_report) VALUES (?, ?, ?, ?)"
    )
    .bind(input.profile_id)
    .bind(input.tool_type)
    .bind(input.summary)
    .bind(input.full_report)
    .execute(&state.db.pool)
    .await;

    match res {
        Ok(r) => Json(serde_json::json!({ "id": r.last_insert_rowid() })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn list_history(
    Extension(state): Extension<AppState>,
) -> Json<serde_json::Value> {
    let res = sqlx::query_as::<_, HistoryRow>(
        "SELECT h.id, h.tool_type, h.summary, h.created_at, p.name as profile_name
         FROM history h
         LEFT JOIN profiles p ON h.profile_id = p.id
         ORDER BY h.created_at DESC LIMIT 50"
    )
    .fetch_all(&state.db.pool)
    .await;

    match res {
        Ok(rows) => {
             Json(serde_json::json!(rows))
        },
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}
