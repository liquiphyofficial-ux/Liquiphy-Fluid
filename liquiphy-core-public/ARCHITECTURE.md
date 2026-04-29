# Architecture

```text
Mock Jito-style intake
  -> sanitized parser
  -> demo signal stub
  -> public policy stub
  -> dry-run execution adapter
  -> fake outcome watcher
```

## Components

## Intake

`src/mock_intake.rs` emits fake events shaped like low-latency Solana observations. It does not connect to a network, RPC node, or Jito endpoint.

## Parser

`src/parser.rs` converts fake raw events into sanitized public events. It does not include production token filters, private wallet lists, or private classification logic.

## Signal Stub

`src/signal_stub.rs` creates a deterministic demo score. This is not insider grading and is not production alpha.

## Policy Stub

`src/policy_stub.rs` accepts or rejects events using fake public constants. Real thresholds, sizing, risk controls, and routing remain private.

## Dry-Run Adapter

`src/dry_run.rs` prints what would happen. It has no signing, keypair loading, bundle sending, or transaction broadcast path.

## Outcome Watcher

`src/outcome.rs` returns simulated outcomes only.

## Jito Integration Boundary

Real Jito integration belongs behind a private adapter. This public crate documents the interface shape without exposing production implementation.
