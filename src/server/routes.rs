use axum::{Extension, Json, Router, routing::get};
use serde::Serialize;

use crate::server::state::AppState;

#[derive(Serialize)]
pub struct ServerStatus {
    uptime_seconds: u64,
    cpu_usage: Vec<f32>,
}

/// GET /server/status
///
/// Returns server metrics including uptime and CPU usage per core.
pub async fn status_handler(Extension(state): Extension<AppState>) -> Json<ServerStatus> {
    let mut sys = state.sys.lock().await;
    sys.refresh_cpu_usage();

    let cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    let uptime_seconds = sysinfo::System::uptime();

    Json(ServerStatus {
        uptime_seconds,
        cpu_usage,
    })
}

pub fn routes() -> Router {
    Router::new().route("/status", get(status_handler))
}
