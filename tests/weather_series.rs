use yesterday_weather::{Celsius, DifferenceBand, WeatherError, WeatherSeries};

#[test]
fn compares_now_with_the_same_hour_yesterday() {
    let sut: Sut = Sut::at_hour_yesterday_and_now(14, 18.0, 21.5);

    assert_eq!(sut.band(), DifferenceBand::Warmer);
}

#[test]
fn at_midnight_yesterday_is_the_first_value() {
    let sut: Sut = Sut::at_hour_yesterday_and_now(0, 20.0, 12.0);

    assert_eq!(sut.band(), DifferenceBand::MuchColder);
}

#[test]
fn at_the_last_hour_now_is_the_last_value() {
    let sut: Sut = Sut::at_hour_yesterday_and_now(23, 12.0, 20.0);

    assert_eq!(sut.band(), DifferenceBand::MuchWarmer);
}

#[test]
fn a_series_that_ends_before_now_is_incomplete() {
    let sut: Sut = Sut::with_count_at_hour(38, 14);

    assert_eq!(sut.error(), WeatherError::IncompleteSeries);
}

#[test]
fn a_series_that_ends_at_now_is_complete() {
    let sut: Sut = Sut::with_count_at_hour(39, 14);

    assert_eq!(sut.band(), DifferenceBand::AboutTheSame);
}

struct Sut {
    built: Result<WeatherSeries, WeatherError>,
}

impl Sut {
    fn at_hour_yesterday_and_now(hour: u8, yesterday: f64, now: f64) -> Sut {
        let yesterday_position: usize = usize::from(hour);
        let now_position: usize = usize::from(hour) + 24;
        let mut hourly: Vec<Celsius> = Vec::new();
        for position in 0..48 {
            let mut degrees: f64 = 0.0;
            if position == yesterday_position {
                degrees = yesterday;
            }
            if position == now_position {
                degrees = now;
            }
            hourly.push(Celsius::new(degrees));
        }
        let built: Result<WeatherSeries, WeatherError> =
            WeatherSeries::new(hourly, hour);
        Sut { built }
    }

    fn with_count_at_hour(count: usize, hour: u8) -> Sut {
        let mut hourly: Vec<Celsius> = Vec::new();
        for _ in 0..count {
            hourly.push(Celsius::new(0.0));
        }
        let built: Result<WeatherSeries, WeatherError> =
            WeatherSeries::new(hourly, hour);
        Sut { built }
    }

    fn band(self) -> DifferenceBand {
        let series: WeatherSeries =
            self.built.expect("the series should be complete");
        series.change().band()
    }

    fn error(self) -> WeatherError {
        let error: Option<WeatherError> = self.built.err();
        error.expect("the series should be incomplete")
    }
}
