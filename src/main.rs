mod config;
mod errors;
mod handlers;
mod middleware;
mod models;
mod services;
mod state;

use axum::{
    middleware as axum_middleware,
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber;

use handlers::{health, generate_qr_json, generate_qr_image};
use handlers::v2::{generate_qr_json_v2, generate_qr_image_v2, generate_qr_query_v2};
use middleware::logging_middleware;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create application state (loads config once)
    let app_state = AppState::new();
    
    info!("Starting QR Code API v{}", env!("CARGO_PKG_VERSION"));
    info!("Configuration: {:?}", app_state.config);

    // Build router with routes and middleware
    let app = Router::new()
        // V1 Routes (Legacy)
        .route("/", get(health))
        .route("/generate", get(generate_qr_json))
        .route("/image", get(generate_qr_image))
        // V2 Routes (Enhanced)
        .route("/v2/generate", post(generate_qr_json_v2))
        .route("/v2/generate", get(generate_qr_query_v2))
        .route("/v2/image", post(generate_qr_image_v2))
        // Middleware
        .layer(axum_middleware::from_fn(logging_middleware))
        .layer(CorsLayer::permissive())
        // State
        .with_state(app_state.clone());

    // Bind to address
    let bind_address = app_state.config.bind_address();
    let listener = TcpListener::bind(&bind_address)
        .await
        .expect("Failed to bind to address");

    info!("🚀 QR Code API server running on http://{}", bind_address);
    info!("📖 Available endpoints:");
    info!("   Health:     GET  /");
    info!("   V1 JSON:    GET  /generate?url=<url>");
    info!("   V1 PNG:     GET  /image?url=<url>");
    info!("   V2 JSON:    POST /v2/generate");
    info!("   V2 Query:   GET  /v2/generate?url=<url>&size=<size>&...");
    info!("   V2 Image:   POST /v2/image");
    
    // Start server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
