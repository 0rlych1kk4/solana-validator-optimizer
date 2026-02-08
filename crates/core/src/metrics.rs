use crate::config::AppConfig;
use once_cell::sync::Lazy;
use prometheus::{Encoder, IntCounter, TextEncoder};

/// Core metrics — registered globally
pub static REQUEST_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    prometheus::register_int_counter!(
        "rpc_requests_total",
        "Total number of RPC requests handled"
    )
    .expect("failed to register rpc_requests_total")
});

pub static CACHE_HIT_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    prometheus::register_int_counter!(
        "rpc_cache_hits_total",
        "Total RPC cache hits"
    )
    .expect("failed to register rpc_cache_hits_total")
});

pub static CACHE_MISS_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    prometheus::register_int_counter!(
        "rpc_cache_misses_total",
        "Total RPC cache misses"
    )
    .expect("failed to register rpc_cache_misses_total")
});

/// Public entry point (always available)
pub async fn start_metrics_server(config: &AppConfig) -> anyhow::Result<()> {
    start_metrics_server_impl(config).await
}

//
// ===== Feature: metrics (HTTP server enabled) =====
//

#[cfg(feature = "metrics")]
async fn start_metrics_server_impl(config: &AppConfig) -> anyhow::Result<()> {
    use axum::{routing::get, Router};
    use std::net::SocketAddr;
    use tokio::net::TcpListener;

    // Initialize Pro metrics if feature enabled
    #[cfg(feature = "pro")]
    {
        use solana_validator_optimizer_pro::metrics::{
            RPC_CACHE_HIT_RATIO,
            RPC_EVICTIONS_TOTAL,
            SNAPSHOT_REPUTATION_SCORE,
        };

        let _ = &*RPC_EVICTIONS_TOTAL;
        let _ = &*RPC_CACHE_HIT_RATIO;
        let _ = &*SNAPSHOT_REPUTATION_SCORE;
    }

    let app = Router::new().route("/metrics", get(serve_metrics));
    let addr = SocketAddr::from(([0, 0, 0, 0], config.metrics_port));

    println!("Metrics server running at http://{}/metrics", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(feature = "metrics")]
async fn serve_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();

    encoder
        .encode(&metric_families, &mut buffer)
        .expect("failed to encode metrics");

    String::from_utf8(buffer).expect("failed to convert metrics to UTF-8")
}

//
// ===== No metrics feature (library-only mode) =====
//

#[cfg(not(feature = "metrics"))]
async fn start_metrics_server_impl(_config: &AppConfig) -> anyhow::Result<()> {
    // No-op when the metrics feature is not enabled.
    Ok(())
}
