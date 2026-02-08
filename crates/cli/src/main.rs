use anyhow::Result;
use clap::{Parser, Subcommand};

use solana_validator_optimizer::{
    config::AppConfig,
    config_autotuner,
    metrics,
    rpc_cache_layer,
    snapshot_prefetcher,
};

#[derive(Parser, Debug)]
#[command(
    name = "svo",
    version = env!("CARGO_PKG_VERSION"),
    about = "Solana Validator Optimizer CLI (snapshot prefetch, RPC cache, metrics)."
)]
struct Cli {
    /// Path to config file (default: ./Config.toml)
    #[arg(long, default_value = "Config.toml")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run snapshot prefetch -> autotune -> rpc cache -> metrics server (until Ctrl+C)
    Run,

    /// Only prefetch snapshot (no-op if snapshot_url is empty)
    Snapshot,

    /// Only run autotune (currently informational)
    Autotune,

    /// Start the RPC cache worker loop (or run once and exit)
    RpcCache {
        /// Run one warm-up cycle and exit (useful for cron/init containers)
        #[arg(long)]
        once: bool,
    },

    /// Start the Prometheus metrics server (until Ctrl+C)
    Metrics,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Operator-friendly: explicit config path; still allows OPTIMIZER_* env overrides.
    let cfg = AppConfig::load_from_path(&cli.config)?;

    match cli.command {
        Commands::Run => {
            snapshot_prefetcher::run(&cfg).await?;
            config_autotuner::autotune_config(&cfg).await?;

            // Start RPC cache loop (expected to spawn and return quickly).
            rpc_cache_layer::start_rpc_cache(&cfg).await?;

            // Start metrics server (blocks in core when feature "metrics" is enabled).
            // We run it as a task so we can also listen for Ctrl+C and exit cleanly.
            let cfg_for_metrics = cfg.clone();
            let metrics_task = tokio::spawn(async move {
                metrics::start_metrics_server(&cfg_for_metrics).await
            });

            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    // Clean shutdown requested
                    println!("Received Ctrl+C, shutting down...");
                }
                res = metrics_task => {
                    // Metrics server ended (unexpected for normal operation)
                    // Propagate both join errors and inner errors.
                    res??;
                }
            }
        }

        Commands::Snapshot => {
            snapshot_prefetcher::run(&cfg).await?;
        }

        Commands::Autotune => {
            config_autotuner::autotune_config(&cfg).await?;
        }

        Commands::RpcCache { once } => {
            if once {
                rpc_cache_layer::run_rpc_cache_once(&cfg).await?;
            } else {
                rpc_cache_layer::start_rpc_cache(&cfg).await?;
                println!("RPC cache loop running. Press Ctrl+C to stop.");
                tokio::signal::ctrl_c().await?;
                println!("Received Ctrl+C, shutting down...");
            }
        }

        Commands::Metrics => {
            // Same pattern: allow Ctrl+C even if you later refactor core behavior.
            let cfg_for_metrics = cfg.clone();
            let metrics_task = tokio::spawn(async move {
                metrics::start_metrics_server(&cfg_for_metrics).await
            });

            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    println!("Received Ctrl+C, shutting down...");
                }
                res = metrics_task => {
                    res??;
                }
            }
        }
    }

    Ok(())
}
