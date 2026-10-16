use crate::celsius::Celsius;
use crate::thermal_sensation_change::ThermalSensationChange;
use crate::weather_error::WeatherError;

const HOURS_PER_DAY: usize = 24;

pub struct WeatherSeries {
    hourly: Vec<Celsius>,
    current_hour: u8,
}

impl WeatherSeries {
    pub fn new(
        hourly: Vec<Celsius>,
        current_hour: u8,
    ) -> Result<WeatherSeries, WeatherError> {
        let now_position: usize = usize::from(current_hour) + HOURS_PER_DAY;
        let needed: usize = now_position + 1;
        let is_complete: bool = hourly.len() >= needed;
        let mut built: Result<WeatherSeries, WeatherError> =
            Err(WeatherError::IncompleteSeries);
        if is_complete {
            built = Ok(WeatherSeries {
                hourly,
                current_hour,
            });
        }
        built
    }

    pub fn change(&self) -> ThermalSensationChange {
        let yesterday_position: usize = usize::from(self.current_hour);
        let now_position: usize = yesterday_position + HOURS_PER_DAY;
        let yesterday: Celsius = self.hourly[yesterday_position];
        let now: Celsius = self.hourly[now_position];
        ThermalSensationChange::between(yesterday, now)
    }
}
