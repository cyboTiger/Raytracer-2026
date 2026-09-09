use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)] // 添加 Copy
pub struct Point(pub f64, pub f64, pub f64);

impl Point {
    pub fn norm_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Point {
        *self / self.norm()
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
    type Output = f64;

    fn mul(self, rhs: Point) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
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
    fn at(&self, t: f64) -> Point {
        Point(
            self.orig.0 + self.dir.0 * t,
            self.orig.1 + self.dir.1 * t,
            self.orig.2 + self.dir.2 * t,
        )
    }
}
