use crate::insider_grading_stub::DemoScore;
use crate::sanitized_event_parser::ParsedEvent;

#[derive(Debug, Clone)]
pub struct DemoRoute {
    pub route_label: String,
    pub max_position_sol: f64,
}

pub fn choose_demo_route(event: &ParsedEvent, score: &DemoScore) -> DemoRoute {
    let route_label = format!("dry-run-route-{}-{}", event.mint_alias, score.label);

    DemoRoute {
        route_label,
        max_position_sol: 0.25,
    }
}

