use crate::{config::AppConfig, rpc_health};
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreflightCheck {
    pub name: String,
    pub status: CheckStatus,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreflightReport {
    pub endpoint: String,
    pub ready: bool,
    pub checks: Vec<PreflightCheck>,
    pub warnings: Vec<String>,
}

pub async fn run(config: &AppConfig) -> Result<PreflightReport> {
    let rpc = rpc_health::check_rpc_health(config).await?;
    Ok(evaluate_rpc_report(rpc))
}

fn evaluate_rpc_report(rpc: rpc_health::RpcHealthReport) -> PreflightReport {
    let mut checks = Vec::new();

    checks.push(PreflightCheck {
        name: "rpc_health".to_string(),
        status: if rpc.healthy {
            CheckStatus::Pass
        } else {
            CheckStatus::Fail
        },
        message: format!("RPC health status: {}", rpc.health_status),
    });

    checks.push(match &rpc.solana_version {
        Some(version) => PreflightCheck {
            name: "solana_version".to_string(),
            status: CheckStatus::Pass,
            message: format!("Detected Solana/Agave version: {version}"),
        },
        None => PreflightCheck {
            name: "solana_version".to_string(),
            status: CheckStatus::Fail,
            message: "Unable to determine Solana/Agave version".to_string(),
        },
    });

    checks.push(match rpc.slot {
        Some(slot) => PreflightCheck {
            name: "current_slot".to_string(),
            status: CheckStatus::Pass,
            message: format!("Current slot: {slot}"),
        },
        None => PreflightCheck {
            name: "current_slot".to_string(),
            status: CheckStatus::Fail,
            message: "Unable to retrieve current slot".to_string(),
        },
    });

    checks.push(match &rpc.latest_blockhash {
        Some(blockhash) => PreflightCheck {
            name: "latest_blockhash".to_string(),
            status: CheckStatus::Pass,
            message: format!("Latest blockhash available: {blockhash}"),
        },
        None => PreflightCheck {
            name: "latest_blockhash".to_string(),
            status: CheckStatus::Fail,
            message: "Unable to retrieve latest blockhash".to_string(),
        },
    });

    let latency_values = [
        ("getHealth", Some(rpc.get_health_latency_ms)),
        ("getVersion", rpc.get_version_latency_ms),
        ("getSlot", rpc.get_slot_latency_ms),
        ("getLatestBlockhash", rpc.get_latest_blockhash_latency_ms),
    ];

    let slow_calls: Vec<String> = latency_values
        .iter()
        .filter_map(|(name, latency)| match latency {
            Some(ms) if *ms > 1_000 => Some(format!("{name}: {ms} ms")),
            _ => None,
        })
        .collect();

    checks.push(if slow_calls.is_empty() {
        PreflightCheck {
            name: "rpc_latency".to_string(),
            status: CheckStatus::Pass,
            message: "RPC latency is within the 1000 ms threshold".to_string(),
        }
    } else {
        PreflightCheck {
            name: "rpc_latency".to_string(),
            status: CheckStatus::Warn,
            message: format!("High RPC latency detected: {}", slow_calls.join(", ")),
        }
    });

    let ready = !checks.iter().any(|check| check.status == CheckStatus::Fail);

    PreflightReport {
        endpoint: rpc.endpoint,
        ready,
        checks,
        warnings: rpc
            .warnings
            .into_iter()
            .filter(|warning| !warning.contains("latency is high"))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn healthy_rpc_report() -> rpc_health::RpcHealthReport {
        rpc_health::RpcHealthReport {
            endpoint: "http://localhost:8899".to_string(),
            healthy: true,
            health_status: "ok".to_string(),
            solana_version: Some("4.2.0-rc.1".to_string()),
            slot: Some(438_200_491),
            latest_blockhash: Some("test-blockhash".to_string()),
            get_health_latency_ms: 100,
            get_version_latency_ms: Some(100),
            get_slot_latency_ms: Some(100),
            get_latest_blockhash_latency_ms: Some(100),
            warnings: Vec::new(),
        }
    }

    #[test]
    fn healthy_rpc_is_ready() {
        let report = evaluate_rpc_report(healthy_rpc_report());

        assert!(report.ready);
        assert!(report
            .checks
            .iter()
            .all(|check| check.status != CheckStatus::Fail));
    }

    #[test]
    fn high_latency_warns_but_remains_ready() {
        let mut rpc = healthy_rpc_report();
        rpc.get_health_latency_ms = 1_500;
        rpc.warnings
            .push("getHealth latency is high: 1500 ms".to_string());

        let report = evaluate_rpc_report(rpc);

        assert!(report.ready);
        assert!(report
            .checks
            .iter()
            .any(|check| { check.name == "rpc_latency" && check.status == CheckStatus::Warn }));
    }

    #[test]
    fn unhealthy_rpc_is_not_ready() {
        let mut rpc = healthy_rpc_report();
        rpc.healthy = false;
        rpc.health_status = "unhealthy".to_string();

        let report = evaluate_rpc_report(rpc);

        assert!(!report.ready);
        assert!(report
            .checks
            .iter()
            .any(|check| { check.name == "rpc_health" && check.status == CheckStatus::Fail }));
    }

    #[test]
    fn missing_slot_is_not_ready() {
        let mut rpc = healthy_rpc_report();
        rpc.slot = None;

        let report = evaluate_rpc_report(rpc);

        assert!(!report.ready);
        assert!(report
            .checks
            .iter()
            .any(|check| { check.name == "current_slot" && check.status == CheckStatus::Fail }));
    }
}
