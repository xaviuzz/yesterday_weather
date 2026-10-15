# yesterdayWeather

Rust CLI that compares the weather now (and the next 24 hours) with yesterday's and says it in natural language. It exists to **learn Rust**: readability of the code outranks brevity, cleverness and performance.

Reference: `reference/weather-rust.md` (APIs, crates, design of the natural-language summary). Its limits and versions were checked on 2026-10-15; verify them before relying on them.

## Commands

- `cargo test`
- `cargo clippy --all-targets -- -D warnings`
- `cargo fmt --check`
- `cargo build`

## Decisions

- Package `yesterday_weather` (cargo wants snake_case); the folder is `yesterdayWeather`.
- Readability for a Rust learner: explicit types where inference hides what a value is, one concept per small function or type, names that say the domain term (`Daypart`, `FeelsLikeDifference`), no macro or trait tricks while a plain construct works.
- The global conventions (explaining variables, explicit loops, one return, no ternary, no comments) apply. Where Rust forces an idiom (`match`, `?`, `Option`, iterators over borrowed data), prefer the most plain form and say so here when it is a decision.
- No dependencies yet. Each crate is added when a failing test needs it, with its why recorded here. The reference proposes `ureq`, `serde`, `serde_json`, `anyhow`, `clap`, `jiff` as candidates.
- Git repository on branch `main`, remote `https://github.com/xaviuzz/yesterday_weather.git` (the repo name is snake_case, the project name is not).
