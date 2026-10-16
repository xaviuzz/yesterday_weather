# Rust concepts covered

Depth: `seen` = one line was enough, `explained` = rule plus example, `solid` = Xavier used or asked about it again and it is clear. Raise the depth after each explanation. Explain only what is new or below `explained`.

| Concept | Depth | Where it came up |
|---|---|---|
| `f64`: 64-bit decimal, `Copy`; alias and newtype as alternatives | explained | `Celsius(pub f64)` |
| `self`, `&self`, `&mut self`: move, shared borrow, mutable borrow | explained | `degrees(&self)` |
| Associated function without `self` (`between`) vs method | explained | `ThermalSensationChange::between` |
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
| `format!` builds a `String`; `{}` placeholders filled by the arguments | explained | `Summary::text` |
| Format spec `{:+.1}`: `+` forces the sign, `.1` one decimal | explained | `Summary::text` |
| `String` (owned, returned) vs `&str` literal (borrowed, fixed text) | seen | `text() -> String` |
| `\` at the end of a string line continues it, skipping the next indentation | seen | `Summary::text` |
| `pub struct` with private fields and a `pub fn of(...)` constructor | seen | `Summary` |
| `match` on an enum: one arm per variant, must be exhaustive, is an expression | explained | `Summary::comparison` |
| Returning `&str` from `&self`: the literal lives for the whole program, so the borrow is valid | seen | `Summary::comparison` |
| Private tuple field: `Celsius(f64)` is built only inside its file; outside, a public constructor | explained | `Celsius::new` |
| `new` is a naming convention for a constructor, not a keyword | seen | `Celsius::new` |
| Privacy is per module: private items are visible in the same file and its child modules, not from `tests/` | explained | `degrees()` read by `Summary` |
| `Vec<T>`: growable list that owns its items; `Vec::new()` and `push` (needs `mut`) | explained | `WeatherSeries` |
| Range `0..48`: from 0 up to, not including, 48 | explained | `Sut` loop |
| `vec[i]` reads by position and panics if `i` is out of range | explained | `change()` |
| Reading `vec[i]` copies the item out, so the item type needs `Copy` | explained | `#[derive(Clone, Copy)]` on `Celsius` |
| `usize::from(u8)`: lossless widening; `usize` is the type of positions | explained | `change()` |
| `Result<T, E>`: `Ok(value)` or `Err(error)`; a function that can fail returns one | explained | `WeatherSeries::new` |
| Own error `enum` with `#[derive(Debug, PartialEq)]` | explained | `WeatherError` |
| `Option<T>` (`Some`/`None`) and `.expect("msg")` on `Option` and `Result`: take the value or panic with the message | explained | `Sut::band`, `Sut::error` |
| `.err()` turns a `Result` into the `Option` of its error | seen | `Sut::error` |
| `self` by value (no `&`): the method consumes the value, which cannot be used afterward | seen | `Sut::band(self)` |
| `for _ in 0..n`: `_` names a value you do not use | seen | `Sut::with_count_at_hour` |

Not covered yet: `?`, traits, lifetimes, modules, ownership of non-`Copy` values.
