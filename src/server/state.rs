use std::sync::Arc;
use sysinfo::Disks;
use sysinfo::System;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct SystemState {
    pub sys: Arc<Mutex<System>>,
    pub disks: Arc<Mutex<Disks>>,
}
