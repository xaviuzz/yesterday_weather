use yesterday_weather::{Celsius, FeelsLikeDifference};

#[test]
fn a_warmer_now_gives_a_positive_difference() {
    let yesterday: Celsius = Celsius(18.0);
    let now: Celsius = Celsius(20.0);

    let difference: FeelsLikeDifference =
        FeelsLikeDifference::between(yesterday, now);

    assert_eq!(difference.degrees(), 2.0);
}

#[test]
fn a_colder_now_gives_a_negative_difference() {
    let yesterday: Celsius = Celsius(20.0);
    let now: Celsius = Celsius(18.0);

    let difference: FeelsLikeDifference =
        FeelsLikeDifference::between(yesterday, now);

    assert_eq!(difference.degrees(), -2.0);
}

#[test]
fn the_same_temperature_gives_no_difference() {
    let yesterday: Celsius = Celsius(20.0);
    let now: Celsius = Celsius(20.0);

    let difference: FeelsLikeDifference =
        FeelsLikeDifference::between(yesterday, now);

    assert_eq!(difference.degrees(), 0.0);
}
