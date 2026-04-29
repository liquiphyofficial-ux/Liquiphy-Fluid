# Liquiphy + Fluid

Jito-oriented Solana trading intelligence prototype with low-latency event intake, dry-run execution research, outcome tracking, and the Fluid public analytics dashboard.

This repository is the public submission hub for both Liquiphy and Fluid. It contains only public-safe demo code, documentation, architecture notes, and links. It does not contain the private production trading system.

## Submission Links

- Fluid site: https://trifluid.com
- Fluid repository: https://github.com/liquiphyofficial-ux/Fluid
- Liquiphy core public-safe interface: `liquiphy-core-public/`
- Jito demo repository export: `liquiphy-jito-demo/`

## What This Shows

- Low-latency Solana architecture designed around Jito-style infrastructure.
- Mock ShredStream-style event intake.
- Sanitized parser and fake signal generation.
- Stub policy, routing, and scoring interfaces.
- Dry-run-only execution adapters.
- Fake outcome watching.
- Fluid as the public dashboard and analytics layer.

## What Is Private

- Real strategy.
- Real wallet routing.
- Real insider grading.
- Real buy/sell thresholds.
- Real execution logic.
- Real signing and key management.
- Private RPC/Jito/Telegram/API credentials.
- Logs, databases, backups, wallets, and production configs.

## Run Demos

Liquiphy core public interface:

```bash
cd liquiphy-core-public
cargo run
```

Jito architecture demo:

```bash
cd liquiphy-jito-demo/rust-public-examples
cargo run
```

Python demo data:

```bash
python liquiphy-jito-demo/python-public-examples/fake_signal_generator.py
python liquiphy-jito-demo/python-public-examples/demo_dashboard_data.py
```

## Repository Layout

```text
liquiphy-core-public/  Public-safe Rust interface demo for Liquiphy core
liquiphy-jito-demo/    Public-safe Jito hackathon demo package
fluid/                 Fluid link and submission notes
docs/                  Root-level submission documentation
```

## Release Safety

This repo uses fresh public-safe contents only. It does not copy private Liquiphy history or production code.
