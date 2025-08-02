use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tracing::info;

pub async fn logging_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    info!("Request: {} {}", method, uri);
    
    let response = next.run(request).await;
    
    info!("Response: {} - {}", method, response.status());
    
    Ok(response)
}