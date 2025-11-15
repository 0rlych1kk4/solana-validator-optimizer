//! solana-validator-optimizer CLI entry point

use clap::Parser;
use anyhow::Result;
use solana_validator_optimizer::config::AppConfig;
use solana_validator_optimizer::{
    blockstream_optimizer,
    config_autotuner,
    metrics,
    rpc_cache_layer,
    snapshot_prefetcher,
};

#[cfg(feature = "pro")]
use solana_validator_optimizer_pro::{show_pro_banner, log_rpc_insights};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = AppConfig::load()?;

    println!(" Running solana-validator-optimizer...");

    #[cfg(feature = "pro")]
    {
        show_pro_banner();
        log_rpc_insights();

        use solana_validator_optimizer_pro::metrics::{
            RPC_EVICTIONS_TOTAL,
            RPC_CACHE_HIT_RATIO,
            SNAPSHOT_REPUTATION_SCORE,
        };

        // Initialize Pro metrics
        RPC_EVICTIONS_TOTAL.inc_by(3);
        RPC_CACHE_HIT_RATIO.set(0.93);
        SNAPSHOT_REPUTATION_SCORE.set(0.88);
    }

    tokio::try_join!(
        snapshot_prefetcher::run(&config),
        blockstream_optimizer::run(&config),
        rpc_cache_layer::start_rpc_cache(&config),
        metrics::start_metrics_server(&config),
        config_autotuner::autotune_config(&config),
    )?;

    Ok(())
}

