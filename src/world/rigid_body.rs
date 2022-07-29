use crate::math::matrix::Matrix;
use crate::math::vector::Vector;

pub trait RigidBody {
    fn mass(&self) -> f64;
    fn inertia_tensor(&self) -> Matrix;
    fn draw(&self, x: &Vector, r: &Matrix);
}
