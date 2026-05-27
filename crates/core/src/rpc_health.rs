use crate::config::AppConfig;
use anyhow::Result;
use serde::Serialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use std::time::Instant;

#[derive(Debug, Clone, Serialize)]
pub struct RpcHealthReport {
    pub endpoint: String,
    pub healthy: bool,
    pub health_status: String,
    pub solana_version: Option<String>,
    pub slot: Option<u64>,
    pub latest_blockhash: Option<String>,
    pub get_health_latency_ms: u128,
    pub get_version_latency_ms: Option<u128>,
    pub get_slot_latency_ms: Option<u128>,
    pub get_latest_blockhash_latency_ms: Option<u128>,
    pub warnings: Vec<String>,
}

pub async fn check_rpc_health(config: &AppConfig) -> Result<RpcHealthReport> {
    check_rpc_health_url(&config.rpc_url).await
}

pub async fn check_rpc_health_url(endpoint: &str) -> Result<RpcHealthReport> {
    let client =
        RpcClient::new_with_commitment(endpoint.to_string(), CommitmentConfig::confirmed());

    let mut warnings = Vec::new();

    let health_start = Instant::now();
    let health_result = client.get_health().await;
    let get_health_latency_ms = health_start.elapsed().as_millis();

    let (healthy, health_status) = match health_result {
        Ok(()) => {
            // In Solana client v3.x, get_health() returns Ok(()) when the RPC node is healthy.
            // The previous implementation formatted Ok(()) as "()" and then looked for "ok",
            // which incorrectly marked a healthy endpoint as unhealthy.
            (true, "ok".to_string())
        }
        Err(err) => {
            warnings.push(format!("getHealth failed: {err}"));
            (false, "unhealthy".to_string())
        }
    };

    let version_start = Instant::now();
    let version_result = client.get_version().await;
    let get_version_latency_ms = Some(version_start.elapsed().as_millis());

    let solana_version = match version_result {
        Ok(version) => Some(version.solana_core),
        Err(err) => {
            warnings.push(format!("getVersion failed: {err}"));
            None
        }
    };

    let slot_start = Instant::now();
    let slot_result = client.get_slot().await;
    let get_slot_latency_ms = Some(slot_start.elapsed().as_millis());

    let slot = match slot_result {
        Ok(slot) => Some(slot),
        Err(err) => {
            warnings.push(format!("getSlot failed: {err}"));
            None
        }
    };

    let blockhash_start = Instant::now();
    let blockhash_result = client.get_latest_blockhash().await;
    let get_latest_blockhash_latency_ms = Some(blockhash_start.elapsed().as_millis());

    let latest_blockhash = match blockhash_result {
        Ok(blockhash) => Some(blockhash.to_string()),
        Err(err) => {
            warnings.push(format!("getLatestBlockhash failed: {err}"));
            None
        }
    };

    if get_health_latency_ms > 1_000 {
        warnings.push(format!(
            "getHealth latency is high: {get_health_latency_ms} ms"
        ));
    }

    if let Some(latency) = get_version_latency_ms {
        if latency > 1_000 {
            warnings.push(format!("getVersion latency is high: {latency} ms"));
        }
    }

    if let Some(latency) = get_slot_latency_ms {
        if latency > 1_000 {
            warnings.push(format!("getSlot latency is high: {latency} ms"));
        }
    }

    if let Some(latency) = get_latest_blockhash_latency_ms {
        if latency > 1_000 {
            warnings.push(format!("getLatestBlockhash latency is high: {latency} ms"));
        }
    }

    Ok(RpcHealthReport {
        endpoint: endpoint.to_string(),
        healthy,
        health_status,
        solana_version,
        slot,
        latest_blockhash,
        get_health_latency_ms,
        get_version_latency_ms,
        get_slot_latency_ms,
        get_latest_blockhash_latency_ms,
        warnings,
    })
}
