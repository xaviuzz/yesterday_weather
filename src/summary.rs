use crate::difference_band::DifferenceBand;
use crate::thermal_sensation_change::ThermalSensationChange;

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
