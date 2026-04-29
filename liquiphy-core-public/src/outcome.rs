use crate::types::DryRunReceipt;

pub fn watch_fake_outcome(receipt: &DryRunReceipt) -> &'static str {
    if receipt.broadcast {
        "unexpected-live-broadcast"
    } else {
        "simulated-dry-run-observed"
    }
}
