use std::path::Path;
use std::sync::OnceLock;

use figment::Figment;
use figment::providers::{Env, Format, Serialized, Toml};
use serde::{Deserialize, Serialize};
use tokio::fs;

mod db;
mod log;
mod tls;

pub use db::DbConfig;
pub use log::LogConfig;
pub use tls::TlsConfig;

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

pub async fn init() {
    let path = Env::var("APP_CONFIG").unwrap_or("config.toml".to_owned());

    // Load from file or create default
    let raw_config = if !Path::new(&path).exists() {
        let config = ServerConfig::default();
        match toml::to_string_pretty(&config) {
            Ok(serialized) => {
                if let Err(e) = fs::write(&path, serialized).await {
                    tracing::error!("Failed to write default config file: {}", e);
                }
            }
            Err(e) => tracing::error!("Failed to generate default server config: {}", e),
        }
        Figment::new().merge(Serialized::defaults(config))
    } else {
        Figment::new().merge(Toml::file(&path))
    };

    // Merge env before loading
    let raw_config = raw_config.merge(Env::prefixed("APP_").global());
    let mut config = match raw_config.extract::<ServerConfig>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("It looks like your config is invalid. The following error occurred: {e}");
            std::process::exit(1);
        }
    };

    // Also accept db url as env in sqlx format
    if config.db.url.is_empty() {
        config.db.url = std::env::var("DATABASE_URL").unwrap_or_default();
    }
    if config.db.url.is_empty() {
        eprintln!("DATABASE_URL is not set");
        std::process::exit(1);
    }

    // Set config
    crate::config::CONFIG
        .set(config)
        .expect("config should be set");
}

pub fn get() -> &'static ServerConfig {
    CONFIG.get().expect("config should be set")
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(default)]
pub struct ServerConfig {
    pub listen_addr: String,

    pub db: DbConfig,
    pub log: LogConfig,
    pub tls: TlsConfig,
}
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:8008".to_owned(),

            db: DbConfig::default(),
            log: LogConfig::default(),
            tls: TlsConfig::default(),
        }
    }
}
