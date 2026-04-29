use crate::sanitized_event_parser::ParsedEvent;
use crate::wallet_router_stub::DemoRoute;

#[derive(Debug, Clone)]
pub struct DryRunReceipt {
    pub dry_run_id: String,
    pub message: String,
}

pub fn dry_run_send(event: &ParsedEvent, route: &DemoRoute) -> DryRunReceipt {
    let dry_run_id = format!("dry-run-{}-{}", event.slot, event.mint_alias);

    DryRunReceipt {
        dry_run_id: dry_run_id.clone(),
        message: format!(
            "{} would send via {}, but no transaction was signed or broadcast",
            dry_run_id, route.route_label
        ),
    }
}

