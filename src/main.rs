mod config;
mod errors;
mod handlers;
mod middleware;
mod models;
mod services;

use axum::{
    middleware as axum_middleware,
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber;

use config::Config;
use handlers::{health, generate_qr_json, generate_qr_image};
use middleware::logging_middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::from_env();
    
    info!("Starting QR Code API v{}", env!("CARGO_PKG_VERSION"));
    info!("Configuration: {:?}", config);

    // Build router with routes and middleware
    let app = Router::new()
        .route("/", get(health))
        .route("/generate", get(generate_qr_json))
        .route("/image", get(generate_qr_image))
        .layer(axum_middleware::from_fn(logging_middleware))
        .layer(CorsLayer::permissive());

    // Bind to address
    let bind_address = config.bind_address();
    let listener = TcpListener::bind(&bind_address)
        .await
        .expect("Failed to bind to address");

    info!("🚀 QR Code API server running on http://{}", bind_address);
    info!("📖 Available endpoints:");
    info!("   Health: GET /");
    info!("   JSON:   GET /generate?url=<your-url>");
    info!("   PNG:    GET /image?url=<your-url>");
    
    // Start server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
