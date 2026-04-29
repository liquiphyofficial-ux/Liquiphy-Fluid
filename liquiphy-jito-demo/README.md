# Liquiphy + Fluid: Jito-Powered Low-Latency Solana Trading Intelligence

Liquiphy + Fluid is a public-safe hackathon demo for a low-latency Solana trading intelligence architecture. It shows how Jito-style data intake, dry-run execution adapters, outcome tracking, and a Fluid dashboard layer can fit together without exposing private strategy, wallet routing, signing, or execution logic.

## Problem

Latency-sensitive Solana systems need faster data, disciplined execution boundaries, and clear outcome feedback. Standard RPC polling can be too slow for short-lived market events, while full trading systems are too sensitive to publish safely.

## Why Jito Matters

Jito infrastructure is relevant because ShredStream-style intake can reduce time-to-signal, and Jito Block Engine transaction/bundle concepts are designed for fast landing, MEV-aware routing, and revert-protected execution research.

## What Is Public Here

- Mock ShredStream-like event intake.
- Sanitized parser and fake signal generation.
- Stubbed grading, wallet routing, and execution policy.
- Dry-run-only sender with no signing path.
- Fake outcome watcher.
- Python helpers for fake dashboard data.
- Documentation showing the public/private security boundary.

## What Remains Private

The production Liquiphy trading brain remains closed-source: real strategy, wallet routing, insider grading, execution thresholds, signing, private endpoints, logs, databases, keys, and production configuration are intentionally excluded.

## Architecture

```text
Jito-style intake
  -> sanitized parser
  -> demo signal layer
  -> policy boundary
  -> dry-run sender
  -> fake outcome watcher
  -> Fluid dashboard / public reporting layer
```

## Repository Structure

```text
docs/                  Architecture and security boundary docs
demo/                  Fake sample events, metrics, and outcomes
rust-public-examples/  Compilable dry-run Rust demo
python-public-examples/ Fake dashboard and config examples
screenshots/           Placeholder for manually reviewed screenshots
fluid-link/            Fluid repo/site connection notes
```

## Run the Rust Demo

```bash
cd rust-public-examples
cargo run
```

Expected flow: receive fake event, parse it, produce fake score, apply stub policy, dry-run send, watch fake outcome.

## Run the Python Demo

```bash
python python-public-examples/fake_signal_generator.py
python python-public-examples/demo_dashboard_data.py
```

## Fluid Fit

Fluid is the public-facing analytics and dashboard layer. It helps judges understand how low-latency signals, metrics, and outcomes could be surfaced to users without publishing private trading logic.

Public site placeholder: `https://trifluid.com`  
Fluid repository placeholder: `https://github.com/liquiphyofficial-ux/Fluid`

## Security Boundary

This repo is a public-safe proof-of-work export. It is not the complete production trading bot and does not contain real alpha, private wallets, live signing, real routing, real execution policy, or secrets.

## Roadmap

- Add a richer public dashboard demo with only synthetic data.
- Add optional local replay from sanitized event files.
- Add mock Jito bundle simulation interfaces.
- Add safer backtesting examples with fake or public-only data.
- Keep the private production core separate and closed-source.

