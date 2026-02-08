use config::Config;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub snapshot_url: String,
    pub snapshot_sha256: Option<String>, // optional hash check
    pub rpc_url: String,
    pub metrics_port: u16,
    pub cache_size: usize,
}

impl AppConfig {
    /// Backward-compatible default loader:
    /// - reads ./Config.toml (base name "Config")
    /// - allows overrides via OPTIMIZER_* env vars
    pub fn load() -> Result<Self, config::ConfigError> {
        let cfg = Config::builder()
            .add_source(config::File::with_name("Config").required(false))
            .add_source(config::Environment::with_prefix("OPTIMIZER"))
            .build()?;

        let mut app: AppConfig = cfg.try_deserialize()?;
        app.normalize();
        Ok(app)
    }

    /// Load config from an explicit path (operator-friendly).
    /// Example: /etc/svo/Config.toml
    pub fn load_from_path(path: &str) -> Result<Self, config::ConfigError> {
        let p = Path::new(path);

        let cfg = Config::builder()
            .add_source(config::File::from(p).required(true))
            .add_source(config::Environment::with_prefix("OPTIMIZER"))
            .build()?;

        let mut app: AppConfig = cfg.try_deserialize()?;
        app.normalize();
        Ok(app)
    }

    /// Normalize config values after deserialization
    fn normalize(&mut self) {
        // Treat empty string as "unset"
        if matches!(self.snapshot_sha256.as_deref(), Some(s) if s.trim().is_empty()) {
            self.snapshot_sha256 = None;
        }

        // Trim snapshot_url for safety
        self.snapshot_url = self.snapshot_url.trim().to_string();

        // Basic safety defaults (prevents operator misconfig)
        if self.cache_size == 0 {
            self.cache_size = 128;
        }

        if self.metrics_port == 0 {
            self.metrics_port = 9090;
        }
    }
}
