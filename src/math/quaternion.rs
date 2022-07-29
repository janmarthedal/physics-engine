use crate::math::approx_eq::ApproxEq;
use crate::math::matrix::Matrix;
use crate::math::sq;
use crate::math::vector::Vector;
use std::ops::{Div, Mul};

#[derive(Debug)]
pub struct Quaternion {
    pub v: Vector,
    pub w: f64,
}

impl Quaternion {
    pub const fn new(v: Vector, w: f64) -> Self {
        Self { v, w }
    }
    #[cfg(test)]
    pub const fn coords(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self::new(Vector::new(x, y, z), w)
    }
    // Assumes `axis` is normalized
    pub fn from_rotation(axis: &Vector, angle: f64) -> Self {
        let phi = 0.5 * angle;
        Quaternion::new(axis * phi.sin(), phi.cos())
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
    pub fn to_rotation_matrix(&self) -> Matrix {
        Matrix::new([
            1.0 - 2.0 * sq(self.v.y) - 2.0 * sq(self.v.z),
            2.0 * self.v.x * self.v.y - 2.0 * self.w * self.v.z,
            2.0 * self.v.x * self.v.z + 2.0 * self.w * self.v.y,
            2.0 * self.v.x * self.v.y + 2.0 * self.w * self.v.z,
            1.0 - 2.0 * sq(self.v.x) - 2.0 * sq(self.v.z),
            2.0 * self.v.y * self.v.z - 2.0 * self.w * self.v.x,
            2.0 * self.v.x * self.v.z - 2.0 * self.w * self.v.y,
            2.0 * self.v.y * self.v.z + 2.0 * self.w * self.v.x,
            1.0 - 2.0 * sq(self.v.x) - 2.0 * sq(self.v.y),
        ])
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

impl Mul<f64> for &Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: f64) -> Self::Output {
        Quaternion {
            v: &self.v * rhs,
            w: self.w * rhs,
        }
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
    use crate::math::approx_eq::{assert_approx_eq, ApproxEq};
    use std::f64::consts::PI;

    #[test]
    fn test_multiplication_of_units() {
        let i = Quaternion::coords(1.0, 0.0, 0.0, 0.0);
        let j = Quaternion::coords(0.0, 1.0, 0.0, 0.0);
        let k = Quaternion::coords(0.0, 0.0, 1.0, 0.0);
        let one = Quaternion::coords(0.0, 0.0, 0.0, 1.0);

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
        let q1 = Quaternion::coords(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::coords(5.0, 6.0, 7.0, 8.0);
        assert_approx_eq!(&q1 * &q2, Quaternion::coords(24.0, 48.0, 48.0, -6.0));
        assert_approx_eq!(&q2 * &q1, Quaternion::coords(32.0, 32.0, 56.0, -6.0));
    }

    #[test]
    fn test_unit_quaternions_to_rotations() {
        assert_approx_eq!(
            Quaternion::coords(1.0, 0.0, 0.0, 0.0).to_rotation_matrix(),
            Matrix::new([1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0])
        );
        assert_approx_eq!(
            Quaternion::coords(0.0, 1.0, 0.0, 0.0).to_rotation_matrix(),
            Matrix::new([-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0])
        );
        assert_approx_eq!(
            Quaternion::coords(0.0, 0.0, 1.0, 0.0).to_rotation_matrix(),
            Matrix::new([-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0])
        );
    }

    #[test]
    fn test_rotation_x_quaternion_to_matrix() {
        let a = PI / 3.0;
        let c = a.cos();
        let s = a.sin();
        let r = Quaternion::from_rotation(&Vector::new(1.0, 0.0, 0.0), a).to_rotation_matrix();
        assert_approx_eq!(r, Matrix::new([1.0, 0.0, 0.0, 0.0, c, -s, 0.0, s, c]));
    }

    #[test]
    fn test_rotation_y_quaternion_to_matrix() {
        let a = PI / 3.0;
        let c = a.cos();
        let s = a.sin();
        let r = Quaternion::from_rotation(&Vector::new(0.0, 1.0, 0.0), a).to_rotation_matrix();
        assert_approx_eq!(r, Matrix::new([c, 0.0, s, 0.0, 1.0, 0.0, -s, 0.0, c]));
    }

    #[test]
    fn test_rotation_z_quaternion_to_matrix() {
        let a = PI / 3.0;
        let c = a.cos();
        let s = a.sin();
        let r = Quaternion::from_rotation(&Vector::new(0.0, 0.0, 1.0), a).to_rotation_matrix();
        assert_approx_eq!(r, Matrix::new([c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0]));
    }

    #[test]
    fn test_rotation_about_z_axis_using_quaternion_vs_matrix() {
        let q = Quaternion::from_rotation(&Vector::new(0.0, 0.0, 1.0), PI / 3.0);
        let r = q.to_rotation_matrix();
        let v = Vector::new(2.0, 1.0, 3.0);
        let u = &r * &v;
        let uhat = &q * &Quaternion::from(&v) * &q.conj();
        assert_approx_eq!(Quaternion::from(&u), uhat);
    }

    #[test]
    fn test_rotation_about_some_axis_using_quaternion_vs_matrix() {
        let q = Quaternion::from_rotation(
            &Vector::new(1.0 / 3f64.sqrt(), -1.0 / 3f64.sqrt(), 1.0 / 3f64.sqrt()),
            PI / 3.0,
        );
        let r = q.to_rotation_matrix();
        let v = Vector::new(2.0, 1.0, 3.0);
        let u = &r * &v;
        let uhat = &q * &Quaternion::from(&v) * &q.conj();
        assert_approx_eq!(Quaternion::from(&u), uhat);
    }
}
