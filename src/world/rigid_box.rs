use super::rigid_body::RigidBody;
use crate::math::sq;
use crate::math::matrix::Matrix;

pub struct RigidBox {
    x: f64,
    y: f64,
    z: f64,
    mass: f64,
}

impl RigidBox {
    pub fn new(x: f64, y: f64, z: f64, density: f64) -> Self {
        Self {
            x,
            y,
            z,
            mass: x * y * z * density,
        }
    }
}

impl RigidBody for RigidBox {
    fn mass(&self) -> f64 {
        self.mass
    }
    fn inertia_tensor(&self) -> Matrix {
        let s = self.mass / 12.0;
        Matrix::new([
            s * (sq(self.y) + sq(self.z)),
            0.0,
            0.0,
            0.0,
            s * (sq(self.x) + sq(self.z)),
            0.0,
            0.0,
            0.0,
            s * (sq(self.x) + sq(self.y)),
        ])
    }
}
