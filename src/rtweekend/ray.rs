use crate::rtweekend;
use crate::rtweekend::ray;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)] // 添加 Copy
pub struct Point(pub f64, pub f64, pub f64);

impl Point {
    pub fn new() -> Self {
        Point(0.0, 0.0, 0.0)
    }

    pub fn norm_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Point {
        *self / self.norm()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0 < s && self.1 < s && self.2 < s
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point(-self.0, -self.1, -self.2)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

pub fn dot(a: Point, b: Point) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

pub struct Ray {
    #[allow(dead_code)]
    pub orig: Point,
    pub dir: Point,
}

impl Ray {
    pub fn new(orig: Point, dir: Point) -> Self {
        Ray { orig, dir }
    }

    #[allow(dead_code)]
    pub fn at(&self, t: f64) -> Point {
        Point(
            self.orig.0 + self.dir.0 * t,
            self.orig.1 + self.dir.1 * t,
            self.orig.2 + self.dir.2 * t,
        )
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray::new(Point::new(), Point::new())
    }
}

pub fn random_point() -> Point {
    Point(
        rtweekend::random_double(),
        rtweekend::random_double(),
        rtweekend::random_double(),
    )
}

pub fn random_point_minmax(min: f64, max: f64) -> Point {
    Point(
        rtweekend::random_double_minmax(min, max),
        rtweekend::random_double_minmax(min, max),
        rtweekend::random_double_minmax(min, max),
    )
}

pub fn random_unit_point() -> Point {
    loop {
        let p = random_point_minmax(-1.0, 1.0);
        let lensq = p.norm_squared();
        if lensq <= 1.0 && lensq > 1e-160 {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: &Point) -> Point {
    let on_unit_sphere = random_unit_point();
    if ray::dot(on_unit_sphere, *normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: &Point, n: &Point) -> Point {
    *v - *n * dot(*v, *n) * 2.0
}
