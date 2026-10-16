use yesterday_weather::{Celsius, DifferenceBand, ThermalSensationChange};

#[test]
fn no_difference_is_about_the_same() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 20.0);

    assert_eq!(sut.band(), DifferenceBand::AboutTheSame);
}

#[test]
fn two_degrees_warmer_is_slightly_warmer() {
    let sut: Sut = Sut::yesterday_and_now(18.0, 20.0);

    assert_eq!(sut.band(), DifferenceBand::SlightlyWarmer);
}

#[test]
fn two_degrees_colder_is_slightly_colder() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 18.0);

    assert_eq!(sut.band(), DifferenceBand::SlightlyColder);
}

#[test]
fn four_degrees_warmer_is_warmer() {
    let sut: Sut = Sut::yesterday_and_now(16.0, 20.0);

    assert_eq!(sut.band(), DifferenceBand::Warmer);
}

#[test]
fn four_degrees_colder_is_colder() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 16.0);

    assert_eq!(sut.band(), DifferenceBand::Colder);
}

#[test]
fn eight_degrees_warmer_is_much_warmer() {
    let sut: Sut = Sut::yesterday_and_now(12.0, 20.0);

    assert_eq!(sut.band(), DifferenceBand::MuchWarmer);
}

#[test]
fn eight_degrees_colder_is_much_colder() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 12.0);

    assert_eq!(sut.band(), DifferenceBand::MuchColder);
}

#[test]
fn just_under_one_degree_is_about_the_same() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 20.5);

    assert_eq!(sut.band(), DifferenceBand::AboutTheSame);
}

#[test]
fn exactly_one_degree_warmer_is_slightly_warmer() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 21.0);

    assert_eq!(sut.band(), DifferenceBand::SlightlyWarmer);
}

#[test]
fn exactly_one_degree_colder_is_slightly_colder() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 19.0);

    assert_eq!(sut.band(), DifferenceBand::SlightlyColder);
}

#[test]
fn exactly_three_degrees_warmer_is_warmer() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 23.0);

    assert_eq!(sut.band(), DifferenceBand::Warmer);
}

#[test]
fn exactly_six_degrees_warmer_is_much_warmer() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 26.0);

    assert_eq!(sut.band(), DifferenceBand::MuchWarmer);
}

struct Sut {
    difference: ThermalSensationChange,
}

impl Sut {
    fn yesterday_and_now(yesterday: f64, now: f64) -> Sut {
        let difference: ThermalSensationChange =
            ThermalSensationChange::between(
                Celsius::new(yesterday),
                Celsius::new(now),
            );
        Sut { difference }
    }

    fn band(&self) -> DifferenceBand {
        self.difference.band()
    }
}
