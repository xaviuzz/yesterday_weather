# yesterdayWeather

Rust CLI that compares the weather now (and the next 24 hours) with yesterday's and says it in natural language. It exists to **learn Rust**: readability of the code outranks brevity, cleverness and performance.

Reference: `.claude/reference/weather-rust.md` (APIs, crates, design of the natural-language summary). Its limits and versions were checked on 2026-10-15; verify them before relying on them.

## Learning

Technology: Rust. Doctrine: PrometeuZZ `.claude/learning-projects.md`. Ledger: PrometeuZZ `.claude/concepts/rust.md`. Explain each new Rust construct (`&self`, `mut`, `derive`, `Copy`) the first time it appears in code.

## Commands

- `cargo test`
- `cargo clippy --all-targets -- -D warnings`
- `cargo fmt --check`
- `cargo build`

Scope and ordered steps of the walking skeleton are in `.claude/specs/walking-skeleton.md`. Read it when starting or resuming a step.

Decisions (each rule and its why) live in `.claude/decisions.md`. Read it before changing design or when you need the why of a rule; record each new decision there, replacing the bullet it changes.
