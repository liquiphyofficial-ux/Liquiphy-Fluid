# Superteam / Jito Submission

## Project

Liquiphy + Fluid: Jito-Powered Low-Latency Solana Trading Intelligence

## Short Description

Liquiphy + Fluid is a Jito-oriented Solana trading intelligence prototype that explores low-latency market data intake, sanitized event parsing, dry-run execution research, outcome tracking, and public-facing analytics.

## Long Description

Liquiphy is the low-latency Solana intelligence and execution research layer. Fluid is the public analytics and dashboard layer. Together they show how Jito-style infrastructure can support latency-sensitive Solana systems without publishing unsafe production trading logic.

This public submission includes selected safe interfaces and demos: mock low-latency intake, sanitized parsers, fake signal generation, stubbed policy/routing/scoring, dry-run execution adapters, and fake outcome watchers. The private production strategy, wallet routing, insider grading, signing, and execution hot path remain closed-source.

## Why Jito

Jito matters because latency-sensitive Solana applications benefit from faster data and stronger transaction landing paths. The architecture maps to:

- ShredStream-style low-latency data intake.
- Jito low-latency transaction send and Block Engine concepts.
- Bundle-aware and MEV-aware execution research.
- Outcome tracking for safer infrastructure iteration.

## What Was Built

- Public-safe Liquiphy core interface demo.
- Public-safe Jito demo package.
- Fluid public site and dashboard linkage.
- Architecture docs.
- Security review docs.
- Sample fake/redacted events and metrics.

## Demo

```bash
cd liquiphy-core-public
cargo run
```

```bash
cd liquiphy-jito-demo/rust-public-examples
cargo run
```

```bash
python liquiphy-jito-demo/python-public-examples/fake_signal_generator.py
```

## Links

- Submission repo: https://github.com/liquiphyofficial-ux/Liquiphy-Fluid
- Fluid site: https://trifluid.com
- Fluid repo: https://github.com/liquiphyofficial-ux/Fluid

## Team Location

Germany.

## Security

The full Liquiphy trading core remains private. This public repo intentionally excludes real alpha, strategy, wallet routing, insider grading, thresholds, signing, private endpoints, keys, logs, databases, wallets, and production configs.
