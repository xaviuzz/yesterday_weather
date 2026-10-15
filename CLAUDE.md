# yesterdayWeather

Rust CLI that compares the weather now (and the next 24 hours) with yesterday's and says it in natural language. It exists to **learn Rust**: readability of the code outranks brevity, cleverness and performance.

Reference: `.claude/reference/weather-rust.md` (APIs, crates, design of the natural-language summary). Its limits and versions were checked on 2026-10-15; verify them before relying on them.

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
- Walking skeleton: scope and ordered steps in `.claude/specs/walking-skeleton.md`. Read it when starting or resuming a step.
- Output language is Spanish only. A second language earns a seam by the rule of three.
- Domain code depends on the `WeatherSource` trait, implemented by the Open-Meteo adapter. Why: tests of the domain run on a fake without network or clock. One `#[ignore]` integration test hits the real API with Valencia (39.47, -0.38).
- `WeatherSeries` answers "now and yesterday at this hour". Why: the 24-hour lookup is a domain rule, not a JSON detail.
- Crate is `lib.rs` plus a thin `main.rs`. Why: tests in `tests/` see only the public library.
- Errors are an own `WeatherError` enum, no `anyhow`. Why: learning Rust's error model. Revisit when three or more variants only propagate.
- Arguments parsed by hand from `std::env::args`, no `clap`. Why: two flags.
- "Yesterday at this hour" is found by position (`i - 24` in the hourly series), no date crate. Known limitation: the autumn clock change (25 October 2026 in Spain) shifts it by one hour. `jiff` enters when a failing test needs date arithmetic.
- Git repository on branch `main`, remote `https://github.com/xaviuzz/yesterday_weather.git` (the repo name is snake_case, the project name is not).
