# Superteam / Jito Submission

## Project Name

Liquiphy + Fluid: Jito-Powered Low-Latency Solana Trading Intelligence

## Short Description

A public-safe demo of a Jito-oriented Solana trading intelligence stack: low-latency event intake, sanitized parsing, dry-run execution adapters, outcome tracking, and a Fluid dashboard layer.

## Long Description

Liquiphy + Fluid explores how latency-sensitive Solana systems can be built around Jito infrastructure without publishing unsafe production trading logic. The public demo focuses on engineering boundaries: mock ShredStream-style intake, a sanitized parser, fake signal generation, stubbed wallet routing and policy evaluation, a dry-run sender, an outcome watcher, and a public Fluid analytics layer.

The production Liquiphy core remains private because it contains sensitive strategy, wallet routing, insider grading, live signing, and execution logic. This repository intentionally exposes only the interfaces and safe patterns needed to show proof-of-work and Jito relevance.

## Fit With "Build on top of Jito infrastructure"

The architecture is designed around two Jito-relevant primitives:

- ShredStream-style low-latency intake for reducing time-to-signal compared with standard polling flows.
- Jito low-latency transaction send / Block Engine concepts for fast landing, bundle-aware execution research, and MEV-aware infrastructure.

This public repo demonstrates the system shape and safe modules while keeping live trading advantage private.

## What Was Built

- Compilable Rust demo modules for intake, parsing, fake scoring, routing stubs, policy stubs, dry-run sending, and outcome watching.
- Python demo utilities for synthetic signals, fake dashboard data, and safe config loading.
- Public/private boundary documentation.
- Architecture and Jito integration documentation.
- Security review and release checklist.
- Fluid dashboard linkage for the user-facing proof-of-work layer.

## Technical Architecture

```text
Jito-style low-latency stream
  -> sanitized event parser
  -> demo signal / inference layer
  -> private policy boundary
  -> dry-run sender
  -> outcome watcher
  -> Fluid dashboard and public reports
```

## Demo Instructions

```bash
cd rust-public-examples
cargo run
```

```bash
python python-public-examples/fake_signal_generator.py
python python-public-examples/demo_dashboard_data.py
```

## Links

GitHub demo repo: `TODO: add GitHub URL`  
Fluid site: `https://trifluid.com`  
Fluid repo: `https://github.com/liquiphyofficial-ux/Fluid`

## Team Location

Germany.

## Roadmap

- Add replayable sanitized event streams.
- Add richer Fluid demo views with synthetic data only.
- Add optional Jito bundle simulation mocks.
- Add more public-safe Rust observability examples.
- Continue hardening the private production boundary.

## Ecosystem Value

This project shows how Jito infrastructure can support latency-sensitive Solana applications beyond simple bots: analytics, execution research, public dashboards, dry-run tooling, and safer infrastructure patterns.

## Security / Private-Core Explanation

The complete production trading core is not public. Real strategy, wallet routing, insider grading, live signing, private endpoints, databases, logs, and keys are intentionally excluded for user safety, operational security, and IP protection.

