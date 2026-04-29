use std::fmt;

use crate::mock_shredstream_intake::RawShredEvent;

#[derive(Debug, Clone)]
pub enum TradeSide {
    Buy,
    Sell,
}

impl fmt::Display for TradeSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeSide::Buy => write!(f, "buy"),
            TradeSide::Sell => write!(f, "sell"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedEvent {
    pub event_id: String,
    pub slot: u64,
    pub mint_alias: String,
    pub side: TradeSide,
    pub amount_sol: f64,
    pub confidence_hint: u8,
}

pub fn parse_event(raw: &RawShredEvent) -> Option<ParsedEvent> {
    if raw.signature.trim().is_empty() || raw.mint_alias.trim().is_empty() {
        return None;
    }

    let side = match raw.side {
        "buy" => TradeSide::Buy,
        "sell" => TradeSide::Sell,
        _ => return None,
    };

    let confidence_hint = match raw.program_hint {
        "pump-like-demo" => 64,
        "amm-demo" => 58,
        _ => 50,
    };

    Some(ParsedEvent {
        event_id: format!("demo-{}-{}", raw.slot, raw.mint_alias),
        slot: raw.slot,
        mint_alias: raw.mint_alias.to_string(),
        side,
        amount_sol: raw.amount_sol,
        confidence_hint,
    })
}

