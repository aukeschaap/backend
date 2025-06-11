use axum::{Router, routing::get, Json};
use serde::Serialize;

#[derive(Serialize)]
struct ServerStatus {
    status: &'static str,
    uptime_seconds: u64,
}

async fn status_handler() -> Json<ServerStatus> {
    Json(ServerStatus {
        status: "ok",
        uptime_seconds: 1234, // Replace with actual uptime later
    })
}

pub fn routes() -> Router {
    Router::new()
        .route("/status", get(status_handler))
}
