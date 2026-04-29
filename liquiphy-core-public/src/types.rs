#[derive(Debug, Clone)]
pub struct RawEvent {
    pub slot: u64,
    pub source: &'static str,
    pub mint_alias: &'static str,
    pub side: Side,
    pub observed_at_ms: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct ParsedEvent {
    pub event_id: String,
    pub slot: u64,
    pub mint_alias: String,
    pub side: Side,
}

#[derive(Debug, Clone)]
pub struct DemoSignal {
    pub label: &'static str,
    pub score: u8,
}

#[derive(Debug, Clone)]
pub struct PolicyDecision {
    pub accepted: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone)]
pub struct DryRunReceipt {
    pub id: String,
    pub broadcast: bool,
}
