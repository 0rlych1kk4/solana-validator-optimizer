//! solana-validator-optimizer library interface
//!
//! # Overview
//! This library provides tools to optimize Solana validator performance,
//! including:
//! - Snapshot prefetching from trusted mirrors
//! - RPC response caching with LRU
//! - Prometheus-ready metrics
//! - Configuration auto-tuning for validator nodes

/// Current version of the optimizer
pub const VERSION: &str = "1.1.1";

/// A brief description of this optimizer's capabilities.
pub fn help_summary() -> &'static str {
    "Optimizes Solana validator performance using snapshot prefetching, RPC caching, and Prometheus metrics."
}

// === Public API modules ===

pub mod blockstream_optimizer;
pub mod config;
pub mod config_autotuner;
pub mod metrics;
pub mod rpc_cache_layer;
pub mod snapshot_prefetcher;
pub mod utils;

