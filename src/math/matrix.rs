use crate::math::approx_eq::{ApproxEq, EPSILON};
use crate::math::vector::Vector;
use std::ops::Mul;

#[derive(Debug)]
pub struct Matrix {
    // row-major elements
    elems: [f64; 9],
}

impl Matrix {
    pub fn new(elems: [f64; 9]) -> Self {
        Self { elems }
    }
    pub fn transpose(&self) -> Self {
        Self {
            elems: [
                self.elems[0],
                self.elems[3],
                self.elems[6],
                self.elems[1],
                self.elems[4],
                self.elems[7],
                self.elems[2],
                self.elems[5],
                self.elems[8],
            ],
        }
    }
    pub fn inverse(&self) -> Option<Self> {
        let [m00, m01, m02, m10, m11, m12, m20, m21, m22] = self.elems;
        // cofactors
        let c00 = m11 * m22 - m12 * m21;
        let c01 = -(m10 * m22 - m12 * m20);
        let c02 = m10 * m21 - m11 * m20;
        let c10 = -(m01 * m22 - m21 * m02);
        let c11 = m00 * m22 - m02 * m20;
        let c12 = -(m00 * m21 - m01 * m20);
        let c20 = m01 * m12 - m11 * m02;
        let c21 = -(m00 * m12 - m10 * m02);
        let c22 = m00 * m11 - m01 * m10;
        // determinant of whole matrix
        let m = m00 * c00 + m01 * c01 + m02 * c02;
        if m.abs() > EPSILON {
            let im = 1.0 / m;
            Some(Matrix {
                elems: [
                    c00 * im,
                    c10 * im,
                    c20 * im,
                    c01 * im,
                    c11 * im,
                    c21 * im,
                    c02 * im,
                    c12 * im,
                    c22 * im,
                ],
            })
        } else {
            None
        }
    }
}

impl ApproxEq for Matrix {
    fn approx_eq(&self, other: &Self) -> bool {
        self.elems.approx_eq(&other.elems)
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut elems = [0f64; 9];
        for i in 0..3 {
            for j in 0..3 {
                elems[3 * i + j] = self.elems[i * 3 + 0] * rhs.elems[0 * 3 + j]
                    + self.elems[i * 3 + 1] * rhs.elems[1 * 3 + j]
                    + self.elems[i * 3 + 2] * rhs.elems[2 * 3 + j];
            }
        }
        Matrix { elems }
        // Matrix {
        //     elems: [
        //         self.elems[0] * rhs.elems[0]
        //             + self.elems[1] * rhs.elems[3]
        //             + self.elems[2] * rhs.elems[6],
        //         self.elems[0] * rhs.elems[1]
        //             + self.elems[1] * rhs.elems[4]
        //             + self.elems[2] * rhs.elems[7],
        //         self.elems[0] * rhs.elems[2]
        //             + self.elems[1] * rhs.elems[5]
        //             + self.elems[2] * rhs.elems[8],
        //         self.elems[3] * rhs.elems[0]
        //             + self.elems[4] * rhs.elems[3]
        //             + self.elems[5] * rhs.elems[6],
        //         self.elems[3] * rhs.elems[1]
        //             + self.elems[4] * rhs.elems[4]
        //             + self.elems[5] * rhs.elems[7],
        //         self.elems[3] * rhs.elems[2]
        //             + self.elems[4] * rhs.elems[5]
        //             + self.elems[5] * rhs.elems[8],
        //         self.elems[6] * rhs.elems[0]
        //             + self.elems[7] * rhs.elems[3]
        //             + self.elems[8] * rhs.elems[6],
        //         self.elems[6] * rhs.elems[1]
        //             + self.elems[7] * rhs.elems[4]
        //             + self.elems[8] * rhs.elems[7],
        //         self.elems[6] * rhs.elems[2]
        //             + self.elems[7] * rhs.elems[5]
        //             + self.elems[8] * rhs.elems[8],
        //     ],
        // }
    }
}

impl Mul<&Vector> for &Matrix {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        Vector::new(
            self.elems[0] * rhs.x + self.elems[1] * rhs.y + self.elems[2] * rhs.z,
            self.elems[3] * rhs.x + self.elems[4] * rhs.y + self.elems[5] * rhs.z,
            self.elems[6] * rhs.x + self.elems[7] * rhs.y + self.elems[8] * rhs.z,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::approx_eq::{assert_approx_eq, ApproxEq};

    const IDENTITY: Matrix = Matrix {
        elems: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
    };

    #[test]
    fn test_matrix_equality_with_identical_matrices() {
        let m1 = Matrix::new([1.0, 2.0, 3.0, 5.0, 6.0, 7.0, 9.0, 8.0, 7.0]);
        let m2 = Matrix::new([1.0, 2.0, 3.0, 5.0, 6.0, 7.0, 9.0, 8.0, 7.0]);
        assert_approx_eq!(m1, m2);
    }

    #[test]
    fn test_matrix_equality_with_different_matrices() {
        let m1 = Matrix::new([1.0, 2.0, 3.0, 5.0, 6.0, 7.0, 9.0, 8.0, 7.0]);
        let m2 = Matrix::new([2.0, 3.0, 4.0, 6.0, 7.0, 8.0, 8.0, 7.0, 6.0]);
        assert!(!m1.approx_eq(&m2));
    }

    #[test]
    fn test_multiplying_two_matrices() {
        let a = Matrix::new([1.0, 2.0, 3.0, 5.0, 6.0, 7.0, 9.0, 8.0, 7.0]);
        let b = Matrix::new([-2.0, 1.0, 2.0, 3.0, 2.0, 1.0, 4.0, 3.0, 6.0]);
        assert_approx_eq!(
            &a * &b,
            Matrix::new([16.0, 14.0, 22.0, 36.0, 38.0, 58.0, 34.0, 46.0, 68.0])
        )
    }

    #[test]
    fn test_a_matrix_multiplied_by_a_vector() {
        let a = Matrix::new([1.0, 2.0, 3.0, 2.0, 4.0, 4.0, 8.0, 6.0, 4.0]);
        let b = Vector::new(1.0, 2.0, 3.0);
        assert_approx_eq!(&a * &b, Vector::new(14.0, 22.0, 32.0));
    }

    #[test]
    fn test_multiplying_a_matrix_by_the_identity_matrix() {
        let a = Matrix::new([0.0, 1.0, 2.0, 1.0, 2.0, 4.0, 2.0, 4.0, 8.0]);
        assert_approx_eq!(&a * &IDENTITY, a)
    }

    #[test]
    fn test_multiplying_the_identity_matrix_by_a_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_approx_eq!(&IDENTITY * &a, a);
    }

    #[test]
    fn test_transposing_a_matrix() {
        let a = Matrix::new([0.0, 9.0, 3.0, 9.0, 8.0, 0.0, 1.0, 8.0, 5.0]);
        let tra = Matrix::new([0.0, 9.0, 1.0, 9.0, 8.0, 8.0, 3.0, 0.0, 5.0]);
        assert_approx_eq!(a.transpose(), tra);
    }

    #[test]
    fn test_transposing_the_identity_matrix() {
        assert_approx_eq!(IDENTITY.transpose(), IDENTITY);
    }

    #[test]
    fn test_calculating_the_inverse_of_a_matrix() {
        let a = Matrix::new([-5.0, 2.0, 6.0, 1.0, -5.0, 1.0, 7.0, 7.0, -6.0]);
        let b = a.inverse().unwrap();
        assert_approx_eq!(&a * &b, IDENTITY);
    }

    #[test]
    fn test_calculating_the_inverse_of_another_matrix() {
        let a = Matrix::new([8.0, -5.0, 9.0, 7.0, 5.0, 6.0, -6.0, 0.0, 9.0]);
        let b = a.inverse().unwrap();
        assert_approx_eq!(&a * &b, IDENTITY);
    }

    #[test]
    fn test_calculating_the_inverse_of_third_matrix() {
        let a = Matrix::new([9.0, 3.0, 0.0, -5.0, -2.0, -6.0, -4.0, 9.0, 6.0]);
        let b = a.inverse().unwrap();
        assert_approx_eq!(&a * &b, IDENTITY);
    }

    #[test]
    fn test_multiplying_a_product_by_its_inverse() {
        let a = Matrix::new([3.0, -9.0, 7.0, 3.0, -8.0, 2.0, -4.0, 4.0, 4.0]);
        let b = Matrix::new([8.0, 2.0, 2.0, 3.0, -1.0, 7.0, 7.0, 0.0, 5.0]);
        let c = &a * &b;
        assert_approx_eq!(&c * &b.inverse().unwrap(), a);
    }
}
