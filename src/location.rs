use std::num::ParseFloatError;

use crate::weather_error::WeatherError;

const ARGUMENT_COUNT: usize = 4;
const MAX_LATITUDE: f64 = 90.0;
const MAX_LONGITUDE: f64 = 180.0;

pub struct Location {
    latitude: f64,
    longitude: f64,
}

impl Location {
    pub fn from_args(args: &[String]) -> Result<Location, WeatherError> {
        let has_two_pairs: bool = args.len() == ARGUMENT_COUNT;
        let latitude: f64 =
            Location::degrees_after(args, "--lat", MAX_LATITUDE)?;
        let longitude: f64 =
            Location::degrees_after(args, "--lon", MAX_LONGITUDE)?;
        let mut built: Result<Location, WeatherError> =
            Err(WeatherError::InvalidArguments);
        if has_two_pairs {
            built = Ok(Location {
                latitude,
                longitude,
            });
        }
        built
    }

    fn degrees_after(
        args: &[String],
        flag: &str,
        limit: f64,
    ) -> Result<f64, WeatherError> {
        let mut degrees: Result<f64, WeatherError> =
            Err(WeatherError::InvalidArguments);
        for position in 0..args.len() {
            if args[position] == flag {
                let value: Option<&String> = args.get(position + 1);
                degrees = Location::degrees_within(value, limit);
            }
        }
        degrees
    }

    fn degrees_within(
        value: Option<&String>,
        limit: f64,
    ) -> Result<f64, WeatherError> {
        let mut degrees: Result<f64, WeatherError> =
            Err(WeatherError::InvalidArguments);
        if let Some(text) = value {
            let parsed: Result<f64, ParseFloatError> = text.parse::<f64>();
            degrees = match parsed {
                Ok(number) if number.abs() <= limit => Ok(number),
                _ => Err(WeatherError::InvalidArguments),
            };
        }
        degrees
    }

    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
    }
}
