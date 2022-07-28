use crate::matrix::Matrix;
use crate::quaternion::Quaternion;
use crate::vector::Vector;

impl From<&Quaternion> for Matrix {
    fn from(q: &Quaternion) -> Self {
        Matrix::new([
            1.0 - 2.0 * q.v.y * q.v.y - 2.0 * q.v.z * q.v.z,
            2.0 * q.v.x * q.v.y - 2.0 * q.w * q.v.z,
            2.0 * q.v.x * q.v.z + 2.0 * q.w * q.v.y,
            2.0 * q.v.x * q.v.y + 2.0 * q.w * q.v.z,
            1.0 - 2.0 * q.v.x * q.v.x - 2.0 * q.v.z * q.v.z,
            2.0 * q.v.y * q.v.z - 2.0 * q.w * q.v.x,
            2.0 * q.v.x * q.v.z - 2.0 * q.w * q.v.y,
            2.0 * q.v.y * q.v.z + 2.0 * q.w * q.v.x,
            1.0 - 2.0 * q.v.x * q.v.x - 2.0 * q.v.y * q.v.y,
        ])
    }
}

// Assumes `axis` is normalized
pub fn rotation_about(axis: &Vector, angle: f64) -> Quaternion {
    let phi = 0.5 * angle;
    Quaternion::new(axis * phi.sin(), phi.cos())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use std::f64::consts::PI;

    fn new_quaternion(x: f64, y: f64, z: f64, w: f64) -> Quaternion {
        Quaternion::new(Vector::new(x, y, z), w)
    }

    #[test]
    fn test_unit_quaternions_to_rotations() {
        assert_approx_eq!(
            Matrix::from(&new_quaternion(1.0, 0.0, 0.0, 0.0)),
            Matrix::new([1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0])
        );
        assert_approx_eq!(
            Matrix::from(&new_quaternion(0.0, 1.0, 0.0, 0.0)),
            Matrix::new([-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0])
        );
        assert_approx_eq!(
            Matrix::from(&new_quaternion(0.0, 0.0, 1.0, 0.0)),
            Matrix::new([-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0])
        );
    }

    #[test]
    fn test_rotation_x_quaternion_to_matrix() {
        let a = PI / 3.0;
        let c = a.cos();
        let s = a.sin();
        let r = Matrix::from(&rotation_about(&Vector::new(1.0, 0.0, 0.0), a));
        assert_approx_eq!(r, Matrix::new([1.0, 0.0, 0.0, 0.0, c, -s, 0.0, s, c]));
    }

    #[test]
    fn test_rotation_y_quaternion_to_matrix() {
        let a = PI / 3.0;
        let c = a.cos();
        let s = a.sin();
        let r = Matrix::from(&rotation_about(&Vector::new(0.0, 1.0, 0.0), a));
        assert_approx_eq!(r, Matrix::new([c, 0.0, s, 0.0, 1.0, 0.0, -s, 0.0, c]));
    }

    #[test]
    fn test_rotation_z_quaternion_to_matrix() {
        let a = PI / 3.0;
        let c = a.cos();
        let s = a.sin();
        let r = Matrix::from(&rotation_about(&Vector::new(0.0, 0.0, 1.0), a));
        assert_approx_eq!(r, Matrix::new([c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0]));
    }

    #[test]
    fn test_rotation_about_z_axis_using_quaternion_vs_matrix() {
        let q = rotation_about(&Vector::new(0.0, 0.0, 1.0), PI / 3.0);
        let r = Matrix::from(&q);
        let v = Vector::new(2.0, 1.0, 3.0);
        let u = &r * &v;
        let uhat = &q * &Quaternion::from(&v) * &q.conj();
        assert_approx_eq!(Quaternion::from(&u), uhat);
    }

    #[test]
    fn test_rotation_about_some_axis_using_quaternion_vs_matrix() {
        let q = rotation_about(&Vector::new(1.0 / 3f64.sqrt(), -1.0 / 3f64.sqrt(), 1.0 / 3f64.sqrt()), PI / 3.0);
        let r = Matrix::from(&q);
        let v = Vector::new(2.0, 1.0, 3.0);
        let u = &r * &v;
        let uhat = &q * &Quaternion::from(&v) * &q.conj();
        assert_approx_eq!(Quaternion::from(&u), uhat);
    }
}
