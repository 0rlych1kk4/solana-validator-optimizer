use crate::{config::AppConfig, rpc_health, system_health};
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
    let system = system_health::check_system_health();

    Ok(evaluate_reports(rpc, system))
}

fn evaluate_reports(
    rpc: rpc_health::RpcHealthReport,
    system: system_health::SystemHealthReport,
) -> PreflightReport {
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

    checks.push(PreflightCheck {
        name: "cpu".to_string(),
        status: if system.logical_cpu_count < 2 {
            CheckStatus::Fail
        } else if system.logical_cpu_count < 8 {
            CheckStatus::Warn
        } else {
            CheckStatus::Pass
        },
        message: format!("Logical CPU count: {}", system.logical_cpu_count),
    });

    checks.push(PreflightCheck {
        name: "memory_total".to_string(),
        status: if system.total_memory_gib() < 4.0 {
            CheckStatus::Fail
        } else if system.total_memory_gib() < 16.0 {
            CheckStatus::Warn
        } else {
            CheckStatus::Pass
        },
        message: format!("Total memory: {:.1} GiB", system.total_memory_gib()),
    });

    checks.push(PreflightCheck {
        name: "memory_available".to_string(),
        status: if system.available_memory_gib() < 4.0 {
            CheckStatus::Warn
        } else {
            CheckStatus::Pass
        },
        message: format!("Available memory: {:.2} GiB", system.available_memory_gib()),
    });

    checks.push(match system.disk_available_gib() {
        Some(available) => PreflightCheck {
            name: "disk_available".to_string(),
            status: if available < 10.0 {
                CheckStatus::Fail
            } else if available < 50.0 {
                CheckStatus::Warn
            } else {
                CheckStatus::Pass
            },
            message: match system.disk_total_gib() {
                Some(total) => {
                    format!("Disk space: {available:.1} GiB available / {total:.1} GiB total")
                }
                None => format!("Disk space available: {available:.1} GiB"),
            },
        },
        None => PreflightCheck {
            name: "disk_available".to_string(),
            status: CheckStatus::Warn,
            message: "Unable to determine disk capacity".to_string(),
        },
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

    fn healthy_system_report() -> system_health::SystemHealthReport {
        system_health::SystemHealthReport {
            logical_cpu_count: 16,
            total_memory_bytes: 64 * 1024 * 1024 * 1024,
            available_memory_bytes: 32 * 1024 * 1024 * 1024,
            disk_total_bytes: Some(2 * 1024 * 1024 * 1024 * 1024),
            disk_available_bytes: Some(500 * 1024 * 1024 * 1024),
        }
    }

    #[test]
    fn healthy_rpc_is_ready() {
        let report = evaluate_reports(healthy_rpc_report(), healthy_system_report());

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

        let report = evaluate_reports(rpc, healthy_system_report());

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

        let report = evaluate_reports(rpc, healthy_system_report());

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

        let report = evaluate_reports(rpc, healthy_system_report());

        assert!(!report.ready);
        assert!(report
            .checks
            .iter()
            .any(|check| { check.name == "current_slot" && check.status == CheckStatus::Fail }));
    }
    #[test]
    fn low_memory_is_not_ready() {
        let mut system = healthy_system_report();
        system.total_memory_bytes = 2 * 1024 * 1024 * 1024;

        let report = evaluate_reports(healthy_rpc_report(), system);

        assert!(!report.ready);
        assert!(report
            .checks
            .iter()
            .any(|check| { check.name == "memory_total" && check.status == CheckStatus::Fail }));
    }

    #[test]
    fn low_available_memory_warns_but_remains_ready() {
        let mut system = healthy_system_report();
        system.available_memory_bytes = 512 * 1024 * 1024;

        let report = evaluate_reports(healthy_rpc_report(), system);

        assert!(report.ready);
        assert!(report.checks.iter().any(|check| {
            check.name == "memory_available" && check.status == CheckStatus::Warn
        }));
    }

    #[test]
    fn low_disk_warns_but_remains_ready() {
        let mut system = healthy_system_report();
        system.disk_available_bytes = Some(25 * 1024 * 1024 * 1024);

        let report = evaluate_reports(healthy_rpc_report(), system);

        assert!(report.ready);
        assert!(report
            .checks
            .iter()
            .any(|check| { check.name == "disk_available" && check.status == CheckStatus::Warn }));
    }
}
