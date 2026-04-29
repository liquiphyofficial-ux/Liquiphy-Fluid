mod dry_run;
mod mock_intake;
mod outcome;
mod parser;
mod policy_stub;
mod signal_stub;
mod types;

use dry_run::dry_run_send;
use mock_intake::next_events;
use outcome::watch_fake_outcome;
use parser::parse_sanitized;
use policy_stub::evaluate_public_stub;
use signal_stub::score_demo_signal;

fn main() {
    println!("Liquiphy Core public-safe demo");
    println!("Mode: dry-run only. No signing. No network calls. No secrets.\n");

    for raw in next_events() {
        println!(
            "received mock event: slot={} source={} observed_at_ms={}",
            raw.slot, raw.source, raw.observed_at_ms
        );

        let parsed = parse_sanitized(&raw);
        println!(
            "parsed sanitized event: id={} mint={}",
            parsed.event_id, parsed.mint_alias
        );

        let signal = score_demo_signal(&parsed);
        println!("produced demo signal: {} score={}", signal.label, signal.score);

        let decision = evaluate_public_stub(&parsed, &signal);
        println!("applied public stub policy: {}", decision.reason);

        let receipt = dry_run_send(&parsed, &decision);
        println!(
            "dry-run send only: id={} broadcast={}",
            receipt.id, receipt.broadcast
        );

        let outcome = watch_fake_outcome(&receipt);
        println!("watched fake outcome: {}\n", outcome);
    }
}
