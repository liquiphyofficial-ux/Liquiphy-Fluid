# Security Review

## Scope

This repository is a clean public submission hub assembled from already-sanitized public exports:

- `liquiphy-core-public/`
- `liquiphy-jito-demo/`
- `fluid/`

No private Liquiphy source tree, private git history, logs, databases, wallets, or environment files were copied.

## Excluded

- Production Liquiphy trading brain.
- Real strategy and execution code.
- Real wallet routing.
- Real insider grading.
- Real thresholds and sizing.
- Signing and key management.
- Private RPC/Jito/Telegram/API credentials.
- `.env` files.
- Logs, databases, backups, archives, and runtime artifacts.

## Included

- Dry-run Rust demos.
- Synthetic sample events and metrics.
- Public documentation.
- Fluid links.
- Placeholder-only `.env.example` files.

## Required Final Check

Before making public:

- Confirm `git log --oneline --all` shows only public-safe history.
- Confirm `git ls-files` contains no private filenames.
- Confirm secret scan reports zero high-confidence hits.
- Confirm GitHub UI does not show old private files.

## Status

Safe to push as a private repository after local scan passes. Make public only after manual GitHub review.
