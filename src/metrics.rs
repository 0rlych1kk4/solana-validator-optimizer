use crate::config::AppConfig;

pub async fn start_metrics_server(_config: &AppConfig) -> anyhow::Result<()> {
    println!(" Metrics server starting...");
    Ok(())
}

