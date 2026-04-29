use crate::types::{ParsedEvent, RawEvent};

pub fn parse_sanitized(event: &RawEvent) -> ParsedEvent {
    ParsedEvent {
        event_id: format!("demo-{}-{}", event.slot, event.mint_alias),
        slot: event.slot,
        mint_alias: event.mint_alias.to_string(),
        side: event.side,
    }
}
