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
| `trait`: a named set of methods a type promises to have; `impl Trait for Type` makes the promise | explained | `WeatherSource`, `FakeWeatherSource` |
| Generic struct `Struct<S: Trait>`: `S` is any type that implements the trait, fixed at compile time | explained | `YesterdayComparison<S>` |
| `impl<S: Trait> Struct<S>`: the `impl` block repeats the generic parameter | explained | `YesterdayComparison` |
| `?` after a `Result`: on `Err` return it from the function, on `Ok` unwrap the value | explained | `summary()` |
| `.clone()` makes an owned copy of a non-`Copy` value such as a `Vec` | seen | `FakeWeatherSource` |
| `&[String]`: a slice, a borrowed view of a list; `&owned` turns a `Vec<String>` into one | explained | `Location::from_args` |
| `text.parse::<f64>()`: text to number, returns a `Result`; `::<f64>` names the target type (turbofish) | explained | `degrees_within` |
| `if let Some(x) = option`: run the block only when the `Option` has a value | explained | `degrees_within` |
| `match` arm with a guard: `Ok(n) if n.abs() <= limit => ...` | explained | `degrees_within` |
| `_` as a catch-all `match` arm | seen | `degrees_within` |
| `slice.get(i)` returns `Option<&T>` instead of panicking like `slice[i]` | explained | `degrees_after` |
| `const` with a primitive type | seen | `ARGUMENT_COUNT` |
| `use path::Item;` brings a name into scope | seen | `ParseFloatError` |
| `mod name;` in `lib.rs` declares a module whose code lives in `name.rs` | explained | `lib.rs` |
| `pub use module::Item;` re-exports, so outsiders write `crate_name::Item` and not `crate_name::module::Item` | explained | `lib.rs` |
| `use crate::module::Item;` imports from another module of the same crate | explained | `summary.rs` |
| `pub(crate)`: visible in the whole crate, hidden from `tests/` and other crates | explained | `Celsius::degrees` |

Not covered yet: lifetimes, ownership of non-`Copy` values.
