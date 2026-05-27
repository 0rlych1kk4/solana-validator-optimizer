use serde::Serialize;

const MICRO_LAMPORTS_PER_LAMPORT: u64 = 1_000_000;

#[derive(Debug, Clone, Serialize)]
pub struct PriorityFeeAdvice {
    pub compute_units: u64,
    pub micro_lamports_per_compute_unit: u64,
    pub estimated_priority_fee_lamports: u64,
    pub estimated_priority_fee_sol: f64,
    pub risk_level: String,
    pub recommendation: String,
}

pub fn advise_priority_fee(
    compute_units: u64,
    micro_lamports_per_compute_unit: u64,
) -> PriorityFeeAdvice {
    let estimated_priority_fee_lamports =
        compute_units.saturating_mul(micro_lamports_per_compute_unit) / MICRO_LAMPORTS_PER_LAMPORT;

    let estimated_priority_fee_sol = estimated_priority_fee_lamports as f64 / 1_000_000_000_f64;

    let (risk_level, recommendation) = classify_fee(
        compute_units,
        micro_lamports_per_compute_unit,
        estimated_priority_fee_lamports,
    );

    PriorityFeeAdvice {
        compute_units,
        micro_lamports_per_compute_unit,
        estimated_priority_fee_lamports,
        estimated_priority_fee_sol,
        risk_level,
        recommendation,
    }
}

fn classify_fee(
    compute_units: u64,
    micro_lamports_per_compute_unit: u64,
    estimated_priority_fee_lamports: u64,
) -> (String, String) {
    if compute_units == 0 {
        return (
            "Invalid".to_string(),
            "Compute units must be greater than zero.".to_string(),
        );
    }

    if micro_lamports_per_compute_unit == 0 {
        return (
            "Low".to_string(),
            "No priority fee is configured. This may be acceptable during low network activity but can reduce transaction landing reliability during congestion.".to_string(),
        );
    }

    if estimated_priority_fee_lamports < 100 {
        return (
            "Low".to_string(),
            "Priority fee is very low. Consider increasing compute unit price for production marketplace, reward, or high-frequency transaction flows.".to_string(),
        );
    }

    if estimated_priority_fee_lamports <= 5_000 {
        return (
            "Normal".to_string(),
            "Priority fee appears reasonable for moderate load. Monitor transaction landing rate and RPC latency in production.".to_string(),
        );
    }

    if estimated_priority_fee_lamports <= 50_000 {
        return (
            "Elevated".to_string(),
            "Priority fee is elevated. This may improve landing probability during congestion but should be monitored for cost impact.".to_string(),
        );
    }

    (
        "High".to_string(),
        "Priority fee is high. Verify that this is intentional and justified by congestion, latency-sensitive flows, or high-value transactions.".to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_priority_fee_in_lamports() {
        let advice = advise_priority_fee(250_000, 1_000);
        assert_eq!(advice.estimated_priority_fee_lamports, 250);
    }

    #[test]
    fn detects_zero_compute_units() {
        let advice = advise_priority_fee(0, 1_000);
        assert_eq!(advice.risk_level, "Invalid");
    }

    #[test]
    fn detects_zero_priority_fee() {
        let advice = advise_priority_fee(250_000, 0);
        assert_eq!(advice.risk_level, "Low");
    }
}
