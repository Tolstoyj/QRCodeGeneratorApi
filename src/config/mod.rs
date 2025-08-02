use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub max_url_length: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            log_level: "info".to_string(),
            max_url_length: 2048,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(host) = env::var("HOST") {
            config.host = host;
        }

        if let Ok(port) = env::var("PORT") {
            config.port = port.parse().unwrap_or(3000);
        }

        if let Ok(log_level) = env::var("LOG_LEVEL") {
            config.log_level = log_level;
        }

        if let Ok(max_url_length) = env::var("MAX_URL_LENGTH") {
            config.max_url_length = max_url_length.parse().unwrap_or(2048);
        }

        config
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}