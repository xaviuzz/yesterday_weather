use crate::celsius::Celsius;
use crate::difference_band::DifferenceBand;

pub struct ThermalSensationChange {
    degrees: f64,
}

impl ThermalSensationChange {
    pub fn between(yesterday: Celsius, now: Celsius) -> ThermalSensationChange {
        let degrees: f64 = now.degrees() - yesterday.degrees();
        ThermalSensationChange { degrees }
    }

    pub(crate) fn degrees(&self) -> f64 {
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
