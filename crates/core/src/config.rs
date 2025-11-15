use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub snapshot_url: String,
    pub snapshot_sha256: Option<String>, //  New field for optional hash check
    pub rpc_url: String,
    pub metrics_port: u16,
    pub cache_size: usize,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        Config::builder()
            .add_source(config::File::with_name("Config").required(false))
            .add_source(config::Environment::with_prefix("OPTIMIZER"))
            .build()?
            .try_deserialize()
    }
}
