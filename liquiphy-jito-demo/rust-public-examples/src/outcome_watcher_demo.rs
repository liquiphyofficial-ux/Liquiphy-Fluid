use crate::dry_run_sender::DryRunReceipt;

#[derive(Debug, Clone)]
pub struct DemoOutcome {
    pub status: String,
    pub latency_ms: u64,
}

pub fn watch_fake_outcome(receipt: &DryRunReceipt) -> DemoOutcome {
    let latency_ms = 35 + (receipt.dry_run_id.len() as u64 % 20);

    DemoOutcome {
        status: "simulated_accepted".to_string(),
        latency_ms,
    }
}

