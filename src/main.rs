use axum::{Extension, Router};
use std::sync::Arc;
use sysinfo::System;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod server;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()) // print logs to stdout
        .init();

    // CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([axum::http::Method::GET])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    // Initialize system information
    let sys = Arc::new(Mutex::new(System::new_all()));
    let disks = Arc::new(Mutex::new(sysinfo::Disks::new_with_refreshed_list()));
    let state = server::state::SystemState { sys, disks };

    let app = Router::new()
        .nest("/server", server::routes::routes())
        .layer(Extension(state))
        .layer(cors);

    // run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!(
        "Server running on http://{}...",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
