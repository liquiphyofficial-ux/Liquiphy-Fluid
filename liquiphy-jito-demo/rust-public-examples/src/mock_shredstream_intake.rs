#[derive(Debug, Clone)]
pub struct RawShredEvent {
    pub slot: u64,
    pub signature: &'static str,
    pub program_hint: &'static str,
    pub mint_alias: &'static str,
    pub side: &'static str,
    pub amount_sol: f64,
    pub observed_at_ms: u64,
    pub source: &'static str,
}

pub fn sample_events() -> Vec<RawShredEvent> {
    vec![
        RawShredEvent {
            slot: 401_234_567,
            signature: "redacted_demo_signature_001",
            program_hint: "pump-like-demo",
            mint_alias: "DEMO_MINT_A",
            side: "buy",
            amount_sol: 0.42,
            observed_at_ms: 1_710_000_000_123,
            source: "mock-shredstream",
        },
        RawShredEvent {
            slot: 401_234_578,
            signature: "redacted_demo_signature_002",
            program_hint: "amm-demo",
            mint_alias: "DEMO_MINT_B",
            side: "sell",
            amount_sol: 0.18,
            observed_at_ms: 1_710_000_000_456,
            source: "mock-shredstream",
        },
    ]
}

