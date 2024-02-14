//Minimal, naive implementation of complex numbers
//with print format and basic arithmetic operator overloads

use std::ops;

#[derive(Copy, Clone)]
pub struct Complex64{
    pub x: f64,
    pub y: f64,
}

impl Complex64{
    pub fn re(&self) -> f64 {
        self.x
    }

    pub fn im(&self) -> f64 {
        self.y
    }

    pub fn r(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn arg(&self) -> f64 {
        f64::atan2(self.y, self.x)
    }

    pub fn i() -> Complex64 {
        Complex64{x: 0.0, y: 1.0}
    }

    pub fn expi(phi: f64) -> Complex64 {
        Complex64{x: f64::cos(phi), y: f64::sin(phi)}
    }
}

impl std::fmt::Display for Complex64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} + {}i", self.x, self.y)
    }
}

impl ops::Neg for Complex64 {
    type Output = Complex64;

    fn neg(self) -> Complex64 {
        Complex64{x: -self.x, y: -self.y}
    }
}

impl ops::Add<Complex64> for Complex64 {
    type Output = Complex64;

    fn add(self, rhs: Complex64) -> Complex64 {
        Complex64{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl ops::Add<f64> for Complex64 {
    type Output = Complex64;

    fn add(self, rhs: f64) -> Complex64 {
        Complex64{x: self.x + rhs, y: self.y}
    }
}

impl ops::Sub<Complex64> for Complex64 {
    type Output = Complex64;

    fn sub(self, rhs: Complex64) -> Complex64 {
        Complex64{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl ops::Mul<Complex64> for Complex64 {
    type Output = Complex64;

    fn mul(self, rhs: Complex64) -> Complex64 {
        Complex64{
            x: self.x * rhs.x - self.y * rhs.y, 
            y: self.x * rhs.y + self.y * rhs.x,
        }
    }
}

impl ops::Mul<f64> for Complex64 {
    type Output = Complex64;

    fn mul(self, rhs: f64) -> Complex64 {
        Complex64{
            x: self.x * rhs, 
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<Complex64> for f64 {
    type Output = Complex64;

    fn mul(self, rhs: Complex64) -> Complex64 {
        Complex64{
            x: self * rhs.x, 
            y: self * rhs.y,
        }
    }
}

impl ops::Div<f64> for Complex64 {
    type Output = Complex64;

    fn div(self, rhs: f64) -> Complex64 {
        Complex64{
            x: self.x / rhs, 
            y: self.y / rhs,
        }
    }
}

impl ops::AddAssign<Complex64> for Complex64 {
    fn add_assign(&mut self, other: Complex64) {
        *self = Self{
            x: self.x + other.x,
            y: self.y + other.y,         
        };
    }
}

