use crate::insider_grading_stub::DemoScore;
use crate::sanitized_event_parser::ParsedEvent;
use crate::wallet_router_stub::DemoRoute;

#[derive(Debug, Clone)]
pub struct PolicyDecision {
    pub should_send: bool,
    pub reason: String,
}

pub fn evaluate_demo_policy(event: &ParsedEvent, score: &DemoScore, route: &DemoRoute) -> PolicyDecision {
    let should_send = score.score >= 55 && event.amount_sol <= route.max_position_sol * 2.0;
    let reason = if should_send {
        "accepted by fake public policy; this is not a production threshold"
    } else {
        "rejected by fake public policy; this is not a production threshold"
    };

    PolicyDecision {
        should_send,
        reason: reason.to_string(),
    }
}

