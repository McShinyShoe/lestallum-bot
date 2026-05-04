use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub port: Option<String>,
    pub email: String,
    pub mc_version: String,
    pub anthropic_api_key: String,
    pub context_file: String,
}

impl AppConfig {
    pub fn new() -> crate::app_error::Result<AppConfig> {
        tracing::warn!("Getting App Config");
        let cfg = Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(Environment::default().prefix("APP").separator("_"))
            .build()?;

        Ok(cfg.try_deserialize()?)
    }
}
