//! solana-validator-optimizer library interface

/// Version of the crate
pub const VERSION: &str = "1.0.1";

/// A sample exported function
pub fn help_summary() -> &'static str {
    "Optimizes Solana validator performance using snapshot prefetching, RPC caching, and Prometheus metrics."
}

