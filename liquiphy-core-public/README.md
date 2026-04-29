# Liquiphy Core

`liquiphy-core` is a public-safe interface demo for the Rust side of the Liquiphy + Fluid architecture. It shows the shape of a Jito-oriented Solana core without publishing private trading logic.

This repository is intentionally not the production trading core.

## What Is Public Here

- Mock low-latency event intake.
- Sanitized event parsing.
- Demo-only signal scoring.
- Stubbed policy evaluation.
- Dry-run execution adapter.
- Fake outcome watcher.
- Public/private security boundary docs.

## What Is Not Public

- Real strategy.
- Real wallet routing.
- Real insider grading.
- Real buy/sell thresholds.
- Real signing or keypair handling.
- Private RPC, Jito, Telegram, or API credentials.
- Databases, logs, backups, and production configuration.

## Why Jito Matters

The private system is designed around Jito-style low-latency Solana infrastructure. The public crate demonstrates the boundaries where ShredStream-style data intake and low-latency transaction-send concepts would connect, while keeping the live trading edge closed.

## Run

```bash
cargo run
```

Expected output:

```text
received mock event
parsed sanitized event
produced demo signal
applied public stub policy
dry-run send only
watched fake outcome
```

## Public/Private Boundary

This repo is safe to publish because it contains no private execution code, no real strategy, no credentials, and no production hot path. The full private Liquiphy core remains closed-source.
