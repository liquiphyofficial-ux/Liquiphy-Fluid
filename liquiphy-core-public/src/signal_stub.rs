use crate::types::{DemoSignal, ParsedEvent, Side};

pub fn score_demo_signal(event: &ParsedEvent) -> DemoSignal {
    let side_offset = match event.side {
        Side::Buy => 9,
        Side::Sell => 4,
    };
    let score = 50 + ((event.slot % 17) as u8) + side_offset;

    DemoSignal {
        label: "demo-score-not-production",
        score,
    }
}
