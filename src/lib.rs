//! solana-validator-optimizer library interface
//!
//! # Overview
//! This library provides tools to optimize Solana validator performance,
//! including snapshot prefetching, RPC response caching, Prometheus metrics,
//! and configuration auto-tuning.

pub const VERSION: &str = "1.1.0";

/// Returns a summary description of what this library provides.
pub fn help_summary() -> &'static str {
    "Optimizes Solana validator performance using snapshot prefetching, RPC caching, and Prometheus metrics."
}

// Re-export internal modules for public API access
pub mod config;
pub mod config_autotuner;
pub mod metrics;
pub mod rpc_cache_layer;
pub mod snapshot_prefetcher;
pub mod blockstream_optimizer;
pub mod utils;

