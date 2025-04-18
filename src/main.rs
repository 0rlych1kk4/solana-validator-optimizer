mod config;
mod snapshot_prefetcher;
mod blockstream_optimizer;
mod rpc_cache_layer;
mod metrics;
mod config_autotuner;

use config::AppConfig;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = AppConfig::load()?;

    println!(" Running solana-validator-optimizer...");

    tokio::try_join!(
        snapshot_prefetcher::run(&config),
        blockstream_optimizer::run(&config),
        rpc_cache_layer::start_rpc_cache(&config),
        metrics::start_metrics_server(&config),
        config_autotuner::autotune_config(&config),
    )?;

    Ok(())
}

