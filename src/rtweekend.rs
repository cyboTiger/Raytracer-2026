pub mod interval;
pub mod ray;
use std::f64::consts::PI;

use rand;

// Constants
pub const INFINITY: f64 = f64::INFINITY;

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

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
