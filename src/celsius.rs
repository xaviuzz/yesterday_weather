#[derive(Clone, Copy)]
pub struct Celsius(f64);

impl Celsius {
    pub fn new(degrees: f64) -> Celsius {
        Celsius(degrees)
    }

    pub(crate) fn degrees(&self) -> f64 {
        self.0
    }
}
