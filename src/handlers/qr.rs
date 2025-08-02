use axum::{
    extract::Query,
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
    Json,
};

use crate::{
    errors::ApiError,
    models::{QrRequest, QrResponse},
    services::QrService,
    config::Config,
};

pub async fn generate_qr_json(Query(params): Query<QrRequest>) -> Result<Json<QrResponse>, ApiError> {
    let config = Config::from_env();
    let qr_service = QrService::new(config);
    let base64_qr = qr_service.generate_qr_code(&params.url)?;

    Ok(Json(QrResponse {
        qr_code: base64_qr,
        format: "png".to_string(),
    }))
}

pub async fn generate_qr_image(Query(params): Query<QrRequest>) -> Result<impl IntoResponse, ApiError> {
    let config = Config::from_env();
    let qr_service = QrService::new(config);
    let image_data = qr_service.generate_qr_image(&params.url)?;

    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/png"));
    headers.insert(
        "content-disposition",
        HeaderValue::from_static("attachment; filename=\"qrcode.png\""),
    );

    Ok((headers, image_data))
}