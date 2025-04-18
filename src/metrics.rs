use axum::{routing::get, Router, serve};
use prometheus::{Encoder, TextEncoder, Registry, IntCounter};
use std::net::SocketAddr;
use crate::config::AppConfig;
use tokio::net::TcpListener;
use once_cell::sync::Lazy;

pub static REQUEST_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new("rpc_requests_total", "Total number of RPC requests handled").unwrap()
});

pub static CACHE_HIT_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new("rpc_cache_hits_total", "Total RPC cache hits").unwrap()
});

pub static CACHE_MISS_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new("rpc_cache_misses_total", "Total RPC cache misses").unwrap()
});

pub async fn start_metrics_server(config: &AppConfig) -> anyhow::Result<()> {
    let registry = Registry::new();

    //  Register all metrics
    registry.register(Box::new(REQUEST_COUNTER.clone()))?;
    registry.register(Box::new(CACHE_HIT_COUNTER.clone()))?;
    registry.register(Box::new(CACHE_MISS_COUNTER.clone()))?;

    let app = Router::new().route("/metrics", get(move || serve_metrics(registry.clone())));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.metrics_port));
    println!(" Metrics server running at http://{}/metrics", addr);

    let listener = TcpListener::bind(addr).await?;
    serve(listener, app).await?;

    Ok(())
}

async fn serve_metrics(registry: Registry) -> String {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

