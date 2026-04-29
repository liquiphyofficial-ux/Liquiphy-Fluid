# Public / Private Boundary

This public repository is not the full trading bot.

## Public

- Demo interfaces.
- Sanitized parsing examples.
- Fake/redacted metrics.
- Dry-run-only sender.
- Documentation and architecture diagrams.
- Fluid dashboard links if manually reviewed.

## Private

- Production strategy.
- Real insider/wallet grading.
- Real wallet routing.
- Real buy/sell thresholds.
- Transaction signing hot path.
- Private RPC/Jito/Telegram/API keys.
- Wallet files, seed phrases, databases, logs, and backups.

## Why This Boundary Is Correct

Publishing a full live trading system would risk leaking secrets, exposing users, and creating operational harm. This repo gives judges enough to assess engineering quality and Jito relevance without exposing the private trading edge.

