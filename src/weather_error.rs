#[derive(Debug, PartialEq)]
pub enum WeatherError {
    IncompleteSeries,
    SourceUnavailable,
    InvalidArguments,
}
