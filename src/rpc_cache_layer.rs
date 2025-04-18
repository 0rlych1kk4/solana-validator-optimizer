use crate::config::AppConfig;

pub async fn start_rpc_cache(_config: &AppConfig) -> anyhow::Result<()> {
    println!(" RPC cache layer running...");
    Ok(())
}

