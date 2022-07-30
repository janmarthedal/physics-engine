use crate::math::matrix::Matrix;

pub trait RigidBody {
    fn mass(&self) -> f64;
    fn inertia_tensor(&self) -> Matrix;
}
