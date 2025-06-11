use std::path::PathBuf;

use axum::{Extension, Json, Router, routing::get};
use serde::Serialize;
use sysinfo::Disk;

use crate::server::state::SystemState;

#[derive(Serialize)]
pub struct CPUUsage {
    uptime_seconds: u64,
    cpu_usage: Vec<f32>,
}

#[derive(Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: PathBuf,
    pub total_space: u64,
    pub available_space: u64,
}

impl From<&Disk> for DiskInfo {
    fn from(disk: &Disk) -> Self {
        Self {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_path_buf(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        }
    }
}

/// GET /server/cpu_usage
///
/// Returns server metrics including uptime and CPU usage per core.
pub async fn cpu_usage_handler(Extension(state): Extension<SystemState>) -> Json<CPUUsage> {
    let mut sys = state.sys.lock().await;
    sys.refresh_cpu_usage();

    let cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    let uptime_seconds = sysinfo::System::uptime();

    Json(CPUUsage {
        uptime_seconds,
        cpu_usage,
    })
}

/// GET /server/disk_usage
///
/// Returns disk usage statistics.
pub async fn disk_usage_handler(Extension(state): Extension<SystemState>) -> Json<Vec<DiskInfo>> {
    let disks = state.disks.lock().await;
    let disk_info: Vec<DiskInfo> = disks.iter().map(DiskInfo::from).collect();
    Json(disk_info)
}

pub fn routes() -> Router {
    Router::new()
        .route("/cpu_usage", get(cpu_usage_handler))
        .route("/disk_usage", get(disk_usage_handler))
}
