# Public / Private Boundary

This repository exposes interfaces, not the production trading system.

## Public

- Data intake shape.
- Parser shape.
- Signal interface.
- Policy interface.
- Dry-run execution interface.
- Outcome watcher interface.

## Private

- Production strategy.
- Real wallet routing.
- Insider grading.
- Buy/sell thresholds.
- Signing and key management.
- Jito send path.
- RPC providers and endpoints.
- Logs, databases, and runtime state.

This boundary protects user safety, infrastructure security, and the Liquiphy research edge.
