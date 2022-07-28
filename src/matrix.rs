use crate::approx_eq::ApproxEq;
use crate::vector::Vector;
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
    use crate::approx_eq::{assert_approx_eq, ApproxEq};

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
}
