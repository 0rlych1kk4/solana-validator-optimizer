use crate::config::AppConfig;

pub async fn run(_config: &AppConfig) -> anyhow::Result<()> {
    println!(" Blockstream optimizer running...");
    Ok(())
}

