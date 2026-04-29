use crate::sanitized_event_parser::{ParsedEvent, TradeSide};

#[derive(Debug, Clone)]
pub struct DemoScore {
    pub label: String,
    pub score: u8,
}

pub fn score_wallet_activity(event: &ParsedEvent) -> DemoScore {
    let base = match event.side {
        TradeSide::Buy => 62,
        TradeSide::Sell => 54,
    };
    let score = ((base + event.confidence_hint) / 2).min(99);

    DemoScore {
        label: "demo-score-not-production".to_string(),
        score,
    }
}

