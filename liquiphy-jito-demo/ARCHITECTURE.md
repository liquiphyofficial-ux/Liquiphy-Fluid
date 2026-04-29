# Architecture

## Text Diagram

```text
                     Public-safe demo boundary
        ------------------------------------------------
        Mock Jito/ShredStream intake
                    |
                    v
        Sanitized event parser
                    |
                    v
        Fake signal / inference layer
                    |
                    v
        Stub policy boundary
                    |
                    v
        Dry-run sender only
                    |
                    v
        Fake outcome watcher
                    |
                    v
        Fluid dashboard / public reporting layer
        ------------------------------------------------

        Private production boundary:
        real strategy, grading, routing, signing, execution,
        private infrastructure, secrets, and production data.
```

## Components

### Data Intake

The demo models a ShredStream-style intake layer with fake events. In production, low-latency intake is the earliest stage where Solana activity becomes actionable.

### Parser

The parser converts raw events into sanitized, redacted events. It does not expose private token filters, private wallet lists, or production classification logic.

### Inference / Demo Signal Layer

The public code emits fake scores only. It proves the interface shape, not the real scoring method.

### Policy Boundary

The policy module is a stub. Real buy/sell thresholds, sizing logic, and risk controls are not included.

### Dry-Run Execution Adapter

The sender never signs or broadcasts a transaction. It only prints a dry-run receipt.

### Outcome Watcher

The outcome watcher simulates post-send feedback with fake outcomes. The real production watcher remains private.

### Fluid Dashboard

Fluid is the public-facing dashboard layer for reports, metrics, and user-facing analytics.

## Public / Private Boundary

Public modules show architecture, safety boundaries, and interfaces. Private modules contain the live trading edge and are intentionally excluded.

## Jito Integration Points

- Low-latency block updates / ShredStream-style intake.
- Block Engine transaction and bundle send concepts.
- Revert-protected, bundle-aware execution research.
- Outcome feedback loops for latency-sensitive systems.

