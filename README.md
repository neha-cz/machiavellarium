# machiavellarium

Experimental platform for studying how **social interaction between LLM agents** with unfrozen weights in a simulated environment can induce or enhance learning.

This repository is scaffolded for a Rust-first simulation core, with room to add Python/ML components later for training and analysis.

## Project layout

```
machiavellarium/
├── Cargo.toml                 # workspace root
├── configs/default.toml       # default experiment config
├── crates/
│   ├── machiavellarium-core/  # shared types, config, errors
│   ├── machiavellarium-env/   # simulation environment & loop
│   └── machiavellarium-cli/   # binary entry point
└── .github/workflows/ci.yml   # CI (fmt, clippy, test)
```

Planned crates (not yet created):

- `machiavellarium-agent` — agent state, policies, and unfrozen-weight updates
- `machiavellarium-interaction` — social interaction protocols and message passing

## Prerequisites

- [Rust](https://rustup.rs/) (stable; see `rust-toolchain.toml`)
- Optional: copy `.env.example` to `.env` for local overrides

## Quick start

```bash
# Build the workspace
cargo build

# Run tests
cargo test

# Lint and format
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings

# Initialize and inspect (does not run steps)
cargo run -p machiavellarium-cli

# Run a full simulation from default config
cargo run -p machiavellarium-cli -- --run

# Use a custom config
cargo run -p machiavellarium-cli -- --config configs/default.toml --run
```
