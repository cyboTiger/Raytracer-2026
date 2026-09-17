pub mod ray;

// Constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::FRAC_1_PI;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
