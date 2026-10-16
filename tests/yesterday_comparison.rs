use yesterday_weather::{
    Celsius, Summary, WeatherError, WeatherSeries, WeatherSource,
    YesterdayComparison,
};

#[test]
fn says_the_summary_of_the_series_the_source_gives() {
    let sut: Sut<FakeWeatherSource> =
        Sut::at_hour_yesterday_and_now(14, 16.0, 20.0);

    assert_eq!(
        sut.text(),
        "Hoy la sensación térmica es más calurosa que ayer a \
         esta hora (+4.0 °C)."
    );
}

#[test]
fn passes_on_the_error_when_the_source_is_unavailable() {
    let sut: Sut<UnavailableWeatherSource> = Sut::unavailable();

    assert_eq!(sut.error(), WeatherError::SourceUnavailable);
}

struct FakeWeatherSource {
    hourly: Vec<Celsius>,
    current_hour: u8,
}

impl WeatherSource for FakeWeatherSource {
    fn weather_series(&self) -> Result<WeatherSeries, WeatherError> {
        WeatherSeries::new(self.hourly.clone(), self.current_hour)
    }
}

struct UnavailableWeatherSource;

impl WeatherSource for UnavailableWeatherSource {
    fn weather_series(&self) -> Result<WeatherSeries, WeatherError> {
        Err(WeatherError::SourceUnavailable)
    }
}

struct Sut<S: WeatherSource> {
    comparison: YesterdayComparison<S>,
}

impl Sut<UnavailableWeatherSource> {
    fn unavailable() -> Sut<UnavailableWeatherSource> {
        let comparison: YesterdayComparison<UnavailableWeatherSource> =
            YesterdayComparison::of(UnavailableWeatherSource);
        Sut { comparison }
    }
}

impl Sut<FakeWeatherSource> {
    fn at_hour_yesterday_and_now(
        hour: u8,
        yesterday: f64,
        now: f64,
    ) -> Sut<FakeWeatherSource> {
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
        let source: FakeWeatherSource = FakeWeatherSource {
            hourly,
            current_hour: hour,
        };
        let comparison: YesterdayComparison<FakeWeatherSource> =
            YesterdayComparison::of(source);
        Sut { comparison }
    }
}

impl<S: WeatherSource> Sut<S> {
    fn text(&self) -> String {
        let summary: Summary = self
            .comparison
            .summary()
            .expect("the source should give a series");
        summary.text()
    }

    fn error(&self) -> WeatherError {
        let failed: Option<WeatherError> = self.comparison.summary().err();
        failed.expect("the source should be unavailable")
    }
}
