use crate::types::{DemoSignal, ParsedEvent, PolicyDecision};

pub fn evaluate_public_stub(_event: &ParsedEvent, signal: &DemoSignal) -> PolicyDecision {
    if signal.score >= 55 {
        PolicyDecision {
            accepted: true,
            reason: "accepted by public demo policy",
        }
    } else {
        PolicyDecision {
            accepted: false,
            reason: "rejected by public demo policy",
        }
    }
}
