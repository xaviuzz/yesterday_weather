use crate::summary::Summary;
use crate::thermal_sensation_change::ThermalSensationChange;
use crate::weather_error::WeatherError;
use crate::weather_series::WeatherSeries;
use crate::weather_source::WeatherSource;

pub struct YesterdayComparison<S: WeatherSource> {
    source: S,
}

impl<S: WeatherSource> YesterdayComparison<S> {
    pub fn of(source: S) -> YesterdayComparison<S> {
        YesterdayComparison { source }
    }

    pub fn summary(&self) -> Result<Summary, WeatherError> {
        let series: WeatherSeries = self.source.weather_series()?;
        let change: ThermalSensationChange = series.change();
        Ok(Summary::of(change))
    }
}
