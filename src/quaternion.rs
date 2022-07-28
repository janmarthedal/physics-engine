use crate::{approx_eq::ApproxEq, vector::Vector};
use std::ops::{Div, Mul};

#[derive(Debug)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quaternion {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
    pub fn from_vector_constant(v: &Vector, w: f64) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w,
        }
    }
    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn normalize(&self) -> Self {
        self / self.magnitude()
    }
    pub fn conj(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}

impl ApproxEq for Quaternion {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x)
            && self.y.approx_eq(&other.y)
            && self.z.approx_eq(&other.z)
            && self.w.approx_eq(&other.w)
    }
}

impl Mul for &Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion {
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z,
            z: self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
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
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl From<&Vector> for Quaternion {
    fn from(q: &Vector) -> Self {
        Quaternion::from_vector_constant(q, 0.0)
    }
}

#[cfg(test)]
impl std::ops::Neg for &Quaternion {
    type Output = Quaternion;
    fn neg(self) -> Self::Output {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Quaternion;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};

    #[test]
    fn test_multiplication_of_units() {
        let i = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        let j = Quaternion::new(0.0, 1.0, 0.0, 0.0);
        let k = Quaternion::new(0.0, 0.0, 1.0, 0.0);
        let one = Quaternion::new(0.0, 0.0, 0.0, 1.0);

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
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(5.0, 6.0, 7.0, 8.0);
        assert_approx_eq!(&q1 * &q2, Quaternion::new(24.0, 48.0, 48.0, -6.0));
        assert_approx_eq!(&q2 * &q1, Quaternion::new(32.0, 32.0, 56.0, -6.0));
    }
}
