use crate::approx_eq::ApproxEq;
use crate::vector::Vector;
use std::ops::{Div, Mul};

#[derive(Debug)]
pub struct Quaternion {
    pub v: Vector,
    pub w: f64,
}

impl Quaternion {
    pub fn new(v: Vector, w: f64) -> Self {
        Self { v, w }
    }
    fn dot(&self, other: &Self) -> f64 {
        self.v.dot(&self.v) + self.w * other.w
    }
    fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn normalize(&self) -> Self {
        self / self.magnitude()
    }
    pub fn conj(&self) -> Self {
        Self {
            v: -&self.v,
            w: self.w,
        }
    }
}

impl ApproxEq for Quaternion {
    fn approx_eq(&self, other: &Self) -> bool {
        self.v.approx_eq(&other.v) && self.w.approx_eq(&other.w)
    }
}

impl Mul for &Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion {
            v: Vector::new(
                self.w * rhs.v.x + self.v.x * rhs.w + self.v.y * rhs.v.z - self.v.z * rhs.v.y,
                self.w * rhs.v.y + self.v.y * rhs.w + self.v.z * rhs.v.x - self.v.x * rhs.v.z,
                self.w * rhs.v.z + self.v.z * rhs.w + self.v.x * rhs.v.y - self.v.y * rhs.v.x,
            ),
            w: self.w * rhs.w - self.v.x * rhs.v.x - self.v.y * rhs.v.y - self.v.z * rhs.v.z,
        }
    }
}

impl Mul<&Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: &Self) -> Self::Output {
        &self * rhs
    }
}

impl Div<f64> for &Quaternion {
    type Output = Quaternion;
    fn div(self, rhs: f64) -> Self::Output {
        Quaternion {
            v: &self.v / rhs,
            w: self.w / rhs,
        }
    }
}

impl From<&Vector> for Quaternion {
    fn from(q: &Vector) -> Self {
        Quaternion::new(q.clone(), 0.0)
    }
}

#[cfg(test)]
impl std::ops::Neg for &Quaternion {
    type Output = Quaternion;
    fn neg(self) -> Self::Output {
        Quaternion {
            v: -&self.v,
            w: -self.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};

    fn new_quaternion(x: f64, y: f64, z: f64, w: f64) -> Quaternion {
        Quaternion::new(Vector::new(x, y, z), w)
    }

    #[test]
    fn test_multiplication_of_units() {
        let i = new_quaternion(1.0, 0.0, 0.0, 0.0);
        let j = new_quaternion(0.0, 1.0, 0.0, 0.0);
        let k = new_quaternion(0.0, 0.0, 1.0, 0.0);
        let one = new_quaternion(0.0, 0.0, 0.0, 1.0);

        assert_approx_eq!(&i * &i, -&one);
        assert_approx_eq!(&i * &j, k);
        assert_approx_eq!(&i * &k, -&j);
        assert_approx_eq!(&i * &one, i);
        assert_approx_eq!(&j * &i, -&k);
        assert_approx_eq!(&j * &j, -&one);
        assert_approx_eq!(&j * &k, i);
        assert_approx_eq!(&j * &one, j);
        assert_approx_eq!(&k * &i, j);
        assert_approx_eq!(&k * &j, -&i);
        assert_approx_eq!(&k * &k, -&one);
        assert_approx_eq!(&k * &one, k);
        assert_approx_eq!(&one * &i, i);
        assert_approx_eq!(&one * &j, j);
        assert_approx_eq!(&one * &k, k);
        assert_approx_eq!(&one * &one, one);
    }

    #[test]
    fn test_some_multiplication() {
        let q1 = new_quaternion(1.0, 2.0, 3.0, 4.0);
        let q2 = new_quaternion(5.0, 6.0, 7.0, 8.0);
        assert_approx_eq!(&q1 * &q2, new_quaternion(24.0, 48.0, 48.0, -6.0));
        assert_approx_eq!(&q2 * &q1, new_quaternion(32.0, 32.0, 56.0, -6.0));
    }
}
