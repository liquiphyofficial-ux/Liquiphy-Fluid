mod dry_run_sender;
mod execution_policy_stub;
mod insider_grading_stub;
mod mock_shredstream_intake;
mod outcome_watcher_demo;
mod sanitized_event_parser;
mod wallet_router_stub;

use dry_run_sender::dry_run_send;
use execution_policy_stub::evaluate_demo_policy;
use insider_grading_stub::score_wallet_activity;
use mock_shredstream_intake::sample_events;
use outcome_watcher_demo::watch_fake_outcome;
use sanitized_event_parser::parse_event;
use wallet_router_stub::choose_demo_route;

fn main() {
    println!("Liquiphy + Fluid public-safe Jito demo");
    println!("Mode: dry-run only. No signing. No network calls. No secrets.\n");

    for raw in sample_events() {
        println!(
            "received fake event: slot={} source={} observed_at_ms={}",
            raw.slot, raw.source, raw.observed_at_ms
        );

        let parsed = match parse_event(&raw) {
            Some(event) => event,
            None => {
                println!("skipped event: parser rejected demo input\n");
                continue;
            }
        };
        println!(
            "parsed fake event: id={} side={} mint={}",
            parsed.event_id, parsed.side, parsed.mint_alias
        );

        let score = score_wallet_activity(&parsed);
        println!("produced fake signal: label={} score={}", score.label, score.score);

        let route = choose_demo_route(&parsed, &score);
        println!(
            "selected stub route: {} max_position_sol={:.2}",
            route.route_label, route.max_position_sol
        );

        let policy = evaluate_demo_policy(&parsed, &score, &route);
        println!("applied stub policy: {}", policy.reason);

        if policy.should_send {
            let receipt = dry_run_send(&parsed, &route);
            println!("dry-run send only: {}", receipt.message);
            let outcome = watch_fake_outcome(&receipt);
            println!("fake outcome watched: {} latency={}ms", outcome.status, outcome.latency_ms);
        } else {
            println!("dry-run send skipped by public stub policy");
        }

        println!();
    }
}
