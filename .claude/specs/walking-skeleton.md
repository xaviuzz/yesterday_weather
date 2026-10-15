# Walking skeleton

`yesterday_weather --lat 39.47 --lon -0.38` prints one Spanish sentence comparing the feels-like temperature now with the same hour yesterday. The sentence carries the signed difference in parentheses, then the line "Datos meteorológicos de Open-Meteo.com" (CC BY 4.0 requires it).

Data: one Open-Meteo request, `current` + `hourly=apparent_temperature`, `past_days=1`, `timezone=auto`. Thresholds on the feels-like difference (today minus yesterday) are those of `.claude/reference/weather-rust.md` section 5: under 1 °C, 1 to 3, 3 to 6, over 6, in both directions.

## Out of scope

Dayparts, rain, `weather_code`, next 24 hours, geocoding, configurable units, a second language, `clap`, `anyhow`, caching.

## Steps

Inside out: pure code first, I/O last. A step is done when its tests, clippy, fmt and build pass. A dependency enters in the step whose failing test needs it.

- [x] 1. `FeelsLikeDifference`: two temperatures to a signed difference.
- [x] 2. Classify the difference into the threshold bands.
- [ ] 3. `Summary` to Spanish text with numeric detail and attribution.
- [ ] 4. `WeatherSeries`: now and yesterday at this hour, tested with a fake `WeatherSource`.
- [ ] 5. `Location` from args, with an error for invalid arguments.
- [ ] 6. Open-Meteo adapter: fixed JSON to `WeatherSeries` (`serde`, `serde_json`).
- [ ] 7. Real `ureq` call behind `WeatherSource`, with the `#[ignore]` test against Valencia.
- [ ] 8. `main.rs` wired: error to stderr, exit code 1.
