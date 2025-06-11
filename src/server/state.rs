use std::sync::Arc;
use sysinfo::System;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub sys: Arc<Mutex<System>>,
}
