use std::num::ParseFloatError;

#[derive(Clone, Copy)]
pub struct Celsius(f64);

impl Celsius {
    pub fn new(degrees: f64) -> Celsius {
        Celsius(degrees)
    }

    fn degrees(&self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DifferenceBand {
    AboutTheSame,
    SlightlyWarmer,
    SlightlyColder,
    Warmer,
    Colder,
    MuchWarmer,
    MuchColder,
}

pub struct ThermalSensationChange {
    degrees: f64,
}

impl ThermalSensationChange {
    pub fn between(yesterday: Celsius, now: Celsius) -> ThermalSensationChange {
        let degrees: f64 = now.degrees() - yesterday.degrees();
        ThermalSensationChange { degrees }
    }

    fn degrees(&self) -> f64 {
        self.degrees
    }

    pub fn band(&self) -> DifferenceBand {
        let mut band: DifferenceBand = DifferenceBand::AboutTheSame;
        for threshold in THRESHOLDS.iter() {
            if threshold.reaches(self.degrees) {
                band = threshold.band_for(self.degrees);
            }
        }
        band
    }
}

#[derive(Debug, PartialEq)]
pub enum WeatherError {
    IncompleteSeries,
    SourceUnavailable,
    InvalidArguments,
}

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

pub struct Location {
    latitude: f64,
    longitude: f64,
}

const ARGUMENT_COUNT: usize = 4;
const MAX_LATITUDE: f64 = 90.0;
const MAX_LONGITUDE: f64 = 180.0;

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

pub trait WeatherSource {
    fn weather_series(&self) -> Result<WeatherSeries, WeatherError>;
}

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

pub struct Summary {
    difference: ThermalSensationChange,
}

impl Summary {
    pub fn of(difference: ThermalSensationChange) -> Summary {
        Summary { difference }
    }

    pub fn text(&self) -> String {
        let comparison: &str = self.comparison();
        let degrees: f64 = self.difference.degrees();
        format!(
            "Hoy la sensación térmica es {} a esta hora ({:+.1} °C).",
            comparison, degrees
        )
    }

    fn comparison(&self) -> &str {
        let band: DifferenceBand = self.difference.band();
        let comparison: &str = match band {
            DifferenceBand::AboutTheSame => "parecida a la de ayer",
            DifferenceBand::SlightlyWarmer => "algo más templada que ayer",
            DifferenceBand::SlightlyColder => "algo más fresca que ayer",
            DifferenceBand::Warmer => "más calurosa que ayer",
            DifferenceBand::Colder => "más fresca que ayer",
            DifferenceBand::MuchWarmer => "mucho más calurosa que ayer",
            DifferenceBand::MuchColder => "mucho más fría que ayer",
        };
        comparison
    }
}

struct Threshold {
    from: f64,
    warmer: DifferenceBand,
    colder: DifferenceBand,
}

impl Threshold {
    fn reaches(&self, degrees: f64) -> bool {
        let magnitude: f64 = degrees.abs();
        magnitude >= self.from
    }

    fn band_for(&self, degrees: f64) -> DifferenceBand {
        let mut band: DifferenceBand = self.warmer;
        if degrees < 0.0 {
            band = self.colder;
        }
        band
    }
}

const THRESHOLDS: [Threshold; 3] = [
    Threshold {
        from: 1.0,
        warmer: DifferenceBand::SlightlyWarmer,
        colder: DifferenceBand::SlightlyColder,
    },
    Threshold {
        from: 3.0,
        warmer: DifferenceBand::Warmer,
        colder: DifferenceBand::Colder,
    },
    Threshold {
        from: 6.0,
        warmer: DifferenceBand::MuchWarmer,
        colder: DifferenceBand::MuchColder,
    },
];
