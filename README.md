# Liquiphy + Fluid

Jito-oriented Solana trading intelligence prototype with low-latency event intake, dry-run execution research, outcome tracking, and the Fluid public analytics dashboard.

This repository is the public submission hub for both Liquiphy and Fluid. It contains only public-safe demo code, documentation, architecture notes, and links. It does not contain the private production trading system.

## Judge Path

Start here if you are reviewing the submission:

1. Read [SUBMISSION.md](./SUBMISSION.md) for the competition summary.
2. Read [docs/judge-guide.md](./docs/judge-guide.md) for what each repo proves.
3. Run the two dry-run demos below.
4. Review the Fluid public analytics repo:
   https://github.com/liquiphyofficial-ux/Fluid
5. Review the standalone Jito demo repo:
   https://github.com/liquiphyofficial-ux/liquiphy-jito-demo

## Submission Links

- Fluid site: https://trifluid.com
- Fluid repository: https://github.com/liquiphyofficial-ux/Fluid
- Standalone Jito demo: https://github.com/liquiphyofficial-ux/liquiphy-jito-demo
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

## Why This Repo Is Useful

This is not just a placeholder repo. It is the public-safe submission hub that
shows the system boundary judges need to evaluate:

- how low-latency Jito-style data enters the system
- how events become sanitized signals
- how policy/execution interfaces are structured without live signing
- how outcomes and metrics feed a dashboard product
- where the private production trading edge is intentionally excluded

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
