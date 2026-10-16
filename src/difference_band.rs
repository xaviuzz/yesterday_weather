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
