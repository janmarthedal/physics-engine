use crate::math::approx_eq::ApproxEq;
use std::ops::{Add, Div, Mul, Neg};

#[derive(Debug, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    // pub fn cross(&self, b: &Self) -> Self {
    //     Self {
    //         x: self.y * b.z - self.z * b.y,
    //         y: self.z * b.x - self.x * b.z,
    //         z: self.x * b.y - self.y * b.x,
    //     }
    // }
}

impl ApproxEq for Vector {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x) && self.y.approx_eq(&other.y) && self.z.approx_eq(&other.z)
    }
}

impl Add for &Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: &Vector) -> Self::Output {
        &self + rhs
    }
}

impl Neg for &Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        -&self
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;
    fn div(self, other: f64) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::approx_eq::assert_approx_eq;

    #[test]
    fn test_new_vector() {
        let p = Vector::new(4.0, -4.0, 3.0);
        assert_approx_eq!(p.x, 4.0);
        assert_approx_eq!(p.y, -4.0);
        assert_approx_eq!(p.z, 3.0);
    }

    // #[test]
    // fn test_subtracting_two_vectors() {
    //     let v1 = Vector::new(3.0, 2.0, 1.0);
    //     let v2 = Vector::new(5.0, 6.0, 7.0);
    //     assert_approx_eq!(v1 - &v2, Vector::new(-2.0, -4.0, -6.0));
    // }

    // #[test]
    // fn test_subtracting_a_vector_from_the_zero_vector() {
    //     let zero = Vector::new(0.0, 0.0, 0.0);
    //     let v = Vector::new(1.0, -2.0, 3.0);
    //     assert_approx_eq!(zero - &v, Vector::new(-1.0, 2.0, -3.0));
    // }

    #[test]
    fn test_negating_a_vector() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert_approx_eq!(-a, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_multiplying_a_vector_by_a_scalar() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert_approx_eq!(&a * 3.5, Vector::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn test_multiplying_a_vector_by_a_fraction() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert_approx_eq!(&a * 0.5, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn test_dividing_a_vector_by_a_scalar() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert_approx_eq!(&a / 2.0, Vector::new(0.5, -1.0, 1.5));
    }

    // #[test]
    // fn test_the_magnitude_of_vector1() {
    //     let v = Vector::new(1.0, 0.0, 0.0);
    //     assert_approx_eq!(v.magnitude(), 1.0);
    // }

    // #[test]
    // fn test_the_magnitude_of_vector2() {
    //     let v = Vector::new(0.0, 1.0, 0.0);
    //     assert_approx_eq!(v.magnitude(), 1.0);
    // }

    // #[test]
    // fn test_the_magnitude_of_vector3() {
    //     let v = Vector::new(0.0, 0.0, 1.0);
    //     assert_approx_eq!(v.magnitude(), 1.0);
    // }

    // #[test]
    // fn test_the_magnitude_of_vector4() {
    //     let v = Vector::new(1.0, 2.0, 3.0);
    //     assert_approx_eq!(v.magnitude(), (14f64).sqrt());
    // }

    // #[test]
    // fn test_the_magnitude_of_vector5() {
    //     let v = Vector::new(-1.0, -2.0, -3.0);
    //     assert_approx_eq!(v.magnitude(), (14f64).sqrt());
    // }

    // #[test]
    // fn test_normalizing_vector1() {
    //     let v = Vector::new(4.0, 0.0, 0.0);
    //     assert_approx_eq!(v.normalize(), Vector::new(1.0, 0.0, 0.0));
    // }

    // #[test]
    // fn test_normalizing_vector2() {
    //     let v = Vector::new(1.0, 2.0, 3.0);
    //     assert_approx_eq!(
    //         v.normalize(),
    //         Vector::new(1.0 / 14f64.sqrt(), 2.0 / 14f64.sqrt(), 3.0 / 14f64.sqrt())
    //     );
    // }

    // #[test]
    // fn test_the_magnitude_of_a_normalized_vector() {
    //     let v = Vector::new(1.0, 2.0, 3.0);
    //     let norm = v.normalize();
    //     assert_approx_eq!(norm.magnitude(), 1.0);
    // }

    #[test]
    fn test_the_dot_product_of_two_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert_approx_eq!(a.dot(&b), 20.0);
    }

    // #[test]
    // fn test_the_cross_product_of_two_vectors() {
    //     let a = Vector::new(1.0, 2.0, 3.0);
    //     let b = Vector::new(2.0, 3.0, 4.0);
    //     assert_approx_eq!(a.cross(&b), Vector::new(-1.0, 2.0, -1.0));
    //     assert_approx_eq!(b.cross(&a), Vector::new(1.0, -2.0, 1.0));
    // }
}
