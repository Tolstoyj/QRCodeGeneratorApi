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

use handlers::{health, generate_qr_json, generate_qr_image, generate_qr_query};
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
        // Core Routes
        .route("/", get(health))
        .route("/generate", post(generate_qr_json))
        .route("/generate", get(generate_qr_query))
        .route("/image", post(generate_qr_image))
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
    info!("   Generate:   POST /generate (JSON body with customization)");
    info!("   Generate:   GET  /generate?url=<url>&size=<size>&...");
    info!("   Download:   POST /image (JSON body with customization)");
    
    // Start server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
