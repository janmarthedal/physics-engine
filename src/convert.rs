use crate::matrix::Matrix;
use crate::quaternion::Quaternion;

impl From<&Quaternion> for Matrix {
    fn from(q: &Quaternion) -> Self {
        Matrix::new([
            1.0 - 2.0 * q.y * q.y - 2.0 * q.z * q.z,
            2.0 * q.x * q.y - 2.0 * q.w * q.z,
            2.0 * q.x * q.z + 2.0 * q.w * q.y,
            2.0 * q.x * q.y + 2.0 * q.w * q.z,
            1.0 - 2.0 * q.x * q.x - 2.0 * q.z * q.z,
            2.0 * q.y * q.z - 2.0 * q.w * q.x,
            2.0 * q.x * q.z - 2.0 * q.w * q.y,
            2.0 * q.y * q.z + 2.0 * q.w * q.x,
            1.0 - 2.0 * q.x * q.x - 2.0 * q.y * q.y,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};

    #[test]
    fn test_unit_quaternions_to_rotations() {
        assert_approx_eq!(
            Matrix::from(&Quaternion::new(1.0, 0.0, 0.0, 0.0)),
            Matrix::new([1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0])
        );
        assert_approx_eq!(
            Matrix::from(&Quaternion::new(0.0, 1.0, 0.0, 0.0)),
            Matrix::new([-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0])
        );
        assert_approx_eq!(
            Matrix::from(&Quaternion::new(0.0, 0.0, 1.0, 0.0)),
            Matrix::new([-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0])
        );
    }
}
