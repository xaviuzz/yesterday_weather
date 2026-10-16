use crate::weather_error::WeatherError;
use crate::weather_series::WeatherSeries;

pub trait WeatherSource {
    fn weather_series(&self) -> Result<WeatherSeries, WeatherError>;
}
