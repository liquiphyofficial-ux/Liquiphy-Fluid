# Architecture

```text
Jito-style low-latency intake
  -> sanitized parser
  -> demo signal layer
  -> private policy boundary
  -> dry-run execution adapter
  -> fake outcome watcher
  -> Fluid dashboard
```

## Public Components

- `liquiphy-core-public/`: small Rust interface demo.
- `liquiphy-jito-demo/`: hackathon demo package with Rust, Python, docs, and fake data.
- `fluid/`: links and context for the public dashboard layer.

## Private Components

The production Liquiphy system remains private. It is not included in this repository.
