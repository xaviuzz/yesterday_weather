# Rust concepts covered

Depth: `seen` = one line was enough, `explained` = rule plus example, `solid` = Xavier used or asked about it again and it is clear. Raise the depth after each explanation. Explain only what is new or below `explained`.

| Concept | Depth | Where it came up |
|---|---|---|
| `f64`: 64-bit decimal, `Copy`; alias and newtype as alternatives | explained | `Celsius(pub f64)` |
| `self`, `&self`, `&mut self`: move, shared borrow, mutable borrow | explained | `degrees(&self)` |
| Associated function without `self` (`between`) vs method | explained | `FeelsLikeDifference::between` |
| Tuple struct and `.0`; accessor method instead of `.0` | explained | `Celsius` |
| Variables immutable by default, `mut` to reassign | explained | `band()` |
| Last expression without `;` is the returned value | seen | `band()`, `reaches()` |
| `enum` variants, path `DifferenceBand::X` | seen | `DifferenceBand` |
| `#[derive(Debug, PartialEq)]` for `assert_eq!` | seen | `DifferenceBand` |
| `#[derive(Clone, Copy)]`: copy instead of move | explained | `THRESHOLDS` returns bands by value |
| `.iter()` in a `for` loop borrows each element | seen | `band()` |
| `const` array of structs | seen | `THRESHOLDS` |
| Private by default, `pub` to expose | seen | `Threshold` is not `pub` |
| `rustfmt.toml` and `max_width` | explained | 80 columns |

Not covered yet: `Option`, `Result` and `?`, `match`, traits, lifetimes, `String` vs `&str`, modules, ownership of non-`Copy` values.
