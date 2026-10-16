use yesterday_weather::{Celsius, Summary, ThermalSensationChange};

#[test]
fn one_and_a_half_degrees_warmer_says_slightly_warmer_with_the_detail() {
    let sut: Sut = Sut::yesterday_and_now(18.0, 19.5);

    assert_eq!(
        sut.text(),
        "Hoy la sensación térmica es algo más templada que ayer a \
         esta hora (+1.5 °C)."
    );
}

#[test]
fn no_difference_says_about_the_same_with_the_detail() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 20.0);

    assert_eq!(
        sut.text(),
        "Hoy la sensación térmica es parecida a la de ayer a \
         esta hora (+0.0 °C)."
    );
}

#[test]
fn two_degrees_colder_says_slightly_colder_with_the_detail() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 18.0);

    assert_eq!(
        sut.text(),
        "Hoy la sensación térmica es algo más fresca que ayer a \
         esta hora (-2.0 °C)."
    );
}

#[test]
fn four_degrees_warmer_says_warmer_with_the_detail() {
    let sut: Sut = Sut::yesterday_and_now(16.0, 20.0);

    assert_eq!(
        sut.text(),
        "Hoy la sensación térmica es más calurosa que ayer a \
         esta hora (+4.0 °C)."
    );
}

#[test]
fn four_degrees_colder_says_colder_with_the_detail() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 16.0);

    assert_eq!(
        sut.text(),
        "Hoy la sensación térmica es más fresca que ayer a \
         esta hora (-4.0 °C)."
    );
}

#[test]
fn eight_degrees_warmer_says_much_warmer_with_the_detail() {
    let sut: Sut = Sut::yesterday_and_now(12.0, 20.0);

    assert_eq!(
        sut.text(),
        "Hoy la sensación térmica es mucho más calurosa que ayer a \
         esta hora (+8.0 °C)."
    );
}

#[test]
fn eight_degrees_colder_says_much_colder_with_the_detail() {
    let sut: Sut = Sut::yesterday_and_now(20.0, 12.0);

    assert_eq!(
        sut.text(),
        "Hoy la sensación térmica es mucho más fría que ayer a \
         esta hora (-8.0 °C)."
    );
}

struct Sut {
    summary: Summary,
}

impl Sut {
    fn yesterday_and_now(yesterday: f64, now: f64) -> Sut {
        let difference: ThermalSensationChange =
            ThermalSensationChange::between(
                Celsius::new(yesterday),
                Celsius::new(now),
            );
        let summary: Summary = Summary::of(difference);
        Sut { summary }
    }

    fn text(&self) -> String {
        self.summary.text()
    }
}
