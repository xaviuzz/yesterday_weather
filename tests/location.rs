use yesterday_weather::{Location, WeatherError};

#[test]
fn reads_latitude_and_longitude_from_their_flags() {
    let sut: Sut = Sut::from_args(&["--lat", "39.47", "--lon", "-0.38"]);

    assert_eq!(sut.latitude(), 39.47);
    assert_eq!(sut.longitude(), -0.38);
}

#[test]
fn accepts_the_flags_in_either_order() {
    let sut: Sut = Sut::from_args(&["--lon", "-0.38", "--lat", "39.47"]);

    assert_eq!(sut.latitude(), 39.47);
    assert_eq!(sut.longitude(), -0.38);
}

#[test]
fn a_missing_flag_is_invalid() {
    let sut: Sut = Sut::from_args(&["--lat", "39.47"]);

    assert_eq!(sut.error(), WeatherError::InvalidArguments);
}

#[test]
fn a_flag_without_its_value_is_invalid() {
    let sut: Sut = Sut::from_args(&["--lat", "39.47", "--lon"]);

    assert_eq!(sut.error(), WeatherError::InvalidArguments);
}

#[test]
fn a_value_that_is_not_a_number_is_invalid() {
    let sut: Sut = Sut::from_args(&["--lat", "north", "--lon", "-0.38"]);

    assert_eq!(sut.error(), WeatherError::InvalidArguments);
}

#[test]
fn latitude_beyond_ninety_is_invalid() {
    let sut: Sut = Sut::from_args(&["--lat", "90.1", "--lon", "0"]);

    assert_eq!(sut.error(), WeatherError::InvalidArguments);
}

#[test]
fn longitude_beyond_one_hundred_eighty_is_invalid() {
    let sut: Sut = Sut::from_args(&["--lat", "0", "--lon", "180.1"]);

    assert_eq!(sut.error(), WeatherError::InvalidArguments);
}

#[test]
fn the_exact_limits_are_valid() {
    let sut: Sut = Sut::from_args(&["--lat", "-90", "--lon", "180"]);

    assert_eq!(sut.latitude(), -90.0);
    assert_eq!(sut.longitude(), 180.0);
}

#[test]
fn an_unknown_flag_is_invalid() {
    let sut: Sut =
        Sut::from_args(&["--lat", "39.47", "--lon", "-0.38", "--zoom", "3"]);

    assert_eq!(sut.error(), WeatherError::InvalidArguments);
}

struct Sut {
    built: Result<Location, WeatherError>,
}

impl Sut {
    fn from_args(args: &[&str]) -> Sut {
        let mut owned: Vec<String> = Vec::new();
        for arg in args {
            owned.push(arg.to_string());
        }
        let built: Result<Location, WeatherError> = Location::from_args(&owned);
        Sut { built }
    }

    fn latitude(&self) -> f64 {
        let location: &Location =
            self.built.as_ref().expect("the arguments should be valid");
        location.latitude()
    }

    fn longitude(&self) -> f64 {
        let location: &Location =
            self.built.as_ref().expect("the arguments should be valid");
        location.longitude()
    }

    fn error(self) -> WeatherError {
        let failed: Option<WeatherError> = self.built.err();
        failed.expect("the arguments should be invalid")
    }
}
