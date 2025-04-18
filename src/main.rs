mod blockstream_optimizer;
mod config;
mod config_autotuner;
mod metrics;
mod rpc_cache_layer;
mod snapshot_prefetcher;

use clap::Parser;
use config::AppConfig;

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
