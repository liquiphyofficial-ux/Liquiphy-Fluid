use crate::types::{RawEvent, Side};

pub fn next_events() -> Vec<RawEvent> {
    vec![
        RawEvent {
            slot: 501_000_001,
            source: "mock-low-latency-feed",
            mint_alias: "DEMO_MINT_ALPHA",
            side: Side::Buy,
            observed_at_ms: 1_710_000_000_100,
        },
        RawEvent {
            slot: 501_000_014,
            source: "mock-low-latency-feed",
            mint_alias: "DEMO_MINT_BETA",
            side: Side::Sell,
            observed_at_ms: 1_710_000_000_450,
        },
    ]
}
