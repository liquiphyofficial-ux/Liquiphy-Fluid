# Security Review

## Release Model

This public release replaces the previous private-history checkout with a fresh public-safe history. The old production-shaped files must not be present in public history.

## Excluded

- `src/execution.rs`
- `src/trade_policy.rs`
- `src/sizing.rs`
- `src/hot_path/*`
- `src/jito_feed.rs`
- `src/jupiter_hot.rs`
- `src/historical_grading.rs`
- `src/runtime_supervisor.rs`
- `src/trade_state.rs`
- Any `.env`, logs, databases, backups, wallets, keypairs, or private configuration.

## Included

- Small demo-only Rust crate.
- Fake events.
- Dry-run sender.
- Stub policy and signal modules.
- Public architecture docs.

## Publish Status

Safe to publish only after:

- `git log --oneline` shows a single public-safe root commit.
- `git ls-files` shows no private production files.
- Secret scan finds no high-confidence secrets.
- GitHub repository remains private until final manual review.

The real Liquiphy core remains private.
