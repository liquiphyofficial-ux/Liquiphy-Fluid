use crate::types::{DryRunReceipt, ParsedEvent, PolicyDecision};

pub fn dry_run_send(event: &ParsedEvent, decision: &PolicyDecision) -> DryRunReceipt {
    DryRunReceipt {
        id: format!("dry-run-{}", event.event_id),
        broadcast: decision.accepted && false,
    }
}
