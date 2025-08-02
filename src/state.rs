use std::sync::Arc;
use crate::{config::Config, services::QrService};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub qr_service: Arc<QrService>,
}

impl AppState {
    pub fn new() -> Self {
        let config = Arc::new(Config::from_env());
        let qr_service = Arc::new(QrService::new((*config).clone()));
        
        Self {
            config,
            qr_service,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}