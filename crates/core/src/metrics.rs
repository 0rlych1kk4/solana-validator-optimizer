use axum::{routing::get, Router, serve};
use prometheus::{Encoder, TextEncoder, IntCounter};
use std::net::SocketAddr;
use crate::config::AppConfig;
use tokio::net::TcpListener;
use once_cell::sync::Lazy;

// Core metrics — registered globally
pub static REQUEST_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    prometheus::register_int_counter!(
        "rpc_requests_total",
        "Total number of RPC requests handled"
    ).unwrap()
});

pub static CACHE_HIT_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    prometheus::register_int_counter!(
        "rpc_cache_hits_total",
        "Total RPC cache hits"
    ).unwrap()
});

pub static CACHE_MISS_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    prometheus::register_int_counter!(
        "rpc_cache_misses_total",
        "Total RPC cache misses"
    ).unwrap()
});

pub async fn start_metrics_server(config: &AppConfig) -> anyhow::Result<()> {
    // Force Pro metrics to initialize (if compiled)
    #[cfg(feature = "pro")]
    {
        use solana_validator_optimizer_pro::metrics::{
            RPC_EVICTIONS_TOTAL,
            RPC_CACHE_HIT_RATIO,
            SNAPSHOT_REPUTATION_SCORE,
        };

        // Touch to trigger lazy init and global registration
        let _ = &*RPC_EVICTIONS_TOTAL;
        let _ = &*RPC_CACHE_HIT_RATIO;
        let _ = &*SNAPSHOT_REPUTATION_SCORE;
    }

    let app = Router::new().route("/metrics", get(serve_metrics));
    let addr = SocketAddr::from(([0, 0, 0, 0], config.metrics_port));

    println!(" Metrics server running at http://{}/metrics", addr);

    let listener = TcpListener::bind(addr).await?;
    serve(listener, app).await?;

    Ok(())
}

async fn serve_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather(); // Uses global registry
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

