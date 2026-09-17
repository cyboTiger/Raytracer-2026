pub mod interval;
pub mod ray;
use rand;

// Constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::FRAC_1_PI;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::random()
}

pub fn random_double_minmax(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
