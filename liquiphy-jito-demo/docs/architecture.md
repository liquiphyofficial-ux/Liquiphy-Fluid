# Architecture Notes

The public demo is an intentionally reduced version of a larger private system. It preserves the engineering shape while removing production trading logic.

## Flow

```text
mock stream event
  -> parser
  -> fake score
  -> stub route
  -> stub execution policy
  -> dry-run sender
  -> fake outcome watcher
```

## Design Goals

- Keep the hot path understandable.
- Use clear interfaces where private modules would attach.
- Avoid real signing, wallet routing, and production thresholds.
- Make the public/private boundary obvious to judges and reviewers.

## What Judges Can Evaluate

- Jito-oriented system design.
- Rust module organization for latency-sensitive work.
- Safe disclosure discipline.
- Fluid as a public-facing analytics layer.

