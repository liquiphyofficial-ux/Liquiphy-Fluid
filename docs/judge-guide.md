# Judge Guide

## Recommended Review Order

1. `SUBMISSION.md`
   - Competition story, Jito fit, what was built.
2. `liquiphy-core-public/`
   - Small Rust public interface demo: mock intake, sanitized parser, fake
     scoring, stub policy, dry-run adapter, fake outcome watcher.
3. `liquiphy-jito-demo/`
   - Larger public-safe Jito package with Rust/Python examples, fake data, and
     public/private boundary docs.
4. Fluid repo
   - Public analytics/product layer: https://github.com/liquiphyofficial-ux/Fluid
5. Fluid site
   - Public dashboard/product surface: https://trifluid.com

## What This Proves

- The project is built around Jito/ShredStream-style low-latency intake.
- The private live trading system has been reduced into safe public interfaces.
- The demos are dry-run only and do not sign or broadcast transactions.
- Fluid gives the system a user-facing analytics/reporting layer.
- The public/private boundary is intentional, not accidental.

## What Not To Expect

The repo does not include production alpha, real insider grading, live wallet
routing, private thresholds, signing code, key management, private endpoints,
logs, databases, or `.env` files.

## Final Submission Checklist

- Add Loom/YouTube demo link.
- Add presentation deck link.
- Confirm `Fluid`, `Liquiphy-Fluid`, and `liquiphy-jito-demo` are public.
- Run a final secret scan before submitting.
