use axum::{
    routing::post,
    Json, Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use crate::tools::decision::{DecisionTool, DecisionInput};

pub async fn start_server() {
    let app = Router::new()
        .route("/api/tools/decision", post(handle_decision))
        .fallback_service(ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("FATUM-MARK2 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_decision(
    Json(payload): Json<DecisionInput>,
) -> Json<serde_json::Value> { // Using Value to allow flexible error returning if needed, or specific struct
    match DecisionTool::run(payload).await {
        Ok(output) => Json(serde_json::to_value(output).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}
