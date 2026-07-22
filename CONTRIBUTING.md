# Contributing

Contributions are welcome. Please follow these guidelines:

## Before submitting a PR

- `cargo clippy --workspace -- -D warnings` must pass with no warnings
- `cargo test --workspace` must pass
- No credentials, tenant IDs, or user/tenant data in any committed file
- New Graph API endpoints must be read-only

## Commit style

Use the prefix format: `[feat]`, `[fix]`, `[docs]`, `[refactor]`, `[test]`

Example: `[feat] Add ownership edges to the access graph`

## Adding a new analysis capability

1. Add the relevant model types to `eagraph-core/src/model.rs`
2. Extend `node_builder.rs` or `edge_analyzer.rs` with the new node/edge logic, with unit tests
3. If it changes escalation-path detection, update `chain_detector.rs` and `risk_scorer.rs`
4. Add the Graph API call in `graph_client.rs`
5. Wire the new capability into `eagraph-cli/src/main.rs`
6. Update `ROADMAP.md` and `CHANGELOG.md`
