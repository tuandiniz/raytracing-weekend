use std::ops::{Div, DivAssign, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f64;3]
}

type _Point3 = Vec3;

impl Vec3 {
    // Static
    pub fn zero_length() -> Self {
        Self { e: [0.0; 3] }
    }
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        Self::new(self.e[0] / len, self.e[0] / len, self.e[0] / len)
    }

    fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] +
            self.e[1] * self.e[1] +
            self.e[2] * self.e[2]
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, denominator: f64) -> Self::Output {
        self * (1.0 / denominator)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, denominator: f64) {
        self.e[0] /= denominator;
        self.e[1] /= denominator;
        self.e[2] /= denominator;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(mut self, multiplier: f64) -> Self::Output {
        self.e[0] *= multiplier;
        self.e[1] *= multiplier;
        self.e[2] *= multiplier;
        self
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, mut vector: Vec3) -> Self::Output {
        vector.e[0] *= self;
        vector.e[1] *= self;
        vector.e[2] *= self;
        vector
    }
}
