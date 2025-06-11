use axum::{Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod server;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()) // print logs to stdout
        .init();

    let app = Router::new()
        .nest("/server", server::routes::routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://{}...", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
