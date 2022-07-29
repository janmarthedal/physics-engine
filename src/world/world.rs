use crate::math::matrix::Matrix;
use crate::math::quaternion::Quaternion;
use crate::math::vector::Vector;

use super::rigid_body::RigidBody;

struct RigidBodyState {
    // position
    x: Vector,
    // orientation
    q: Quaternion,
    // linear momentum
    p: Vector,
    // angular momentum
    l: Vector,
    // orientation matrix
    r: Matrix,
    // linear velocity
    v: Vector,
    // angular velocity
    w: Vector,
}

impl RigidBodyState {
    fn new(
        x: &Vector,
        q: &Quaternion,
        p: &Vector,
        l: &Vector,
        inv_mass: f64,
        inv_inertia: &Matrix,
    ) -> Self {
        let q = q.normalize(); // always necessary?
        let r = q.to_rotation_matrix();
        let v = p * inv_mass;
        let w = &r * (inv_inertia * (&r.transpose() * l));
        Self {
            x: x.clone(),
            q,
            p: p.clone(),
            l: l.clone(),
            r,
            v,
            w,
        }
    }
}

struct WorldObject {
    state: RigidBodyState,
    inv_mass: f64,
    inv_inertia: Matrix,
    body: Box<dyn RigidBody>,
}

const FORCE: Vector = Vector::new(0.0, 0.0, -1.0);
const TORQUE: Vector = Vector::new(0.0, 0.0, 0.0);

impl WorldObject {
    fn step(&mut self, _t: f64, dt: f64) {
        let halfdt = 0.5 * dt;
        let thirddt = dt / 3.0;
        let sixthdt = dt / 6.0;
        let s0 = &self.state;

        // a1 = G(t, s0), b1 = s0 + (dt / 2) * a1
        let a1dxdt = &s0.v;
        let a1dqdt = &Quaternion::new(&s0.w * 0.5, 0.0) * &s0.q;
        let a1dpdt = &FORCE;
        let a1dldt = &TORQUE;

        let x = &s0.x + &(a1dxdt * halfdt);
        let q = &s0.q + &(&a1dqdt * halfdt);
        let p = &s0.p + &(a1dpdt * halfdt);
        let l = &s0.l + &(a1dldt * halfdt);
        let b1 = RigidBodyState::new(&x, &q, &p, &l, self.inv_mass, &self.inv_inertia);

        // a2 = G(t + dt / 2, b1), b2 = s0 + (dt / 2) * a2
        let a2dxdt = &b1.v;
        let a2dqdt = &Quaternion::new(&b1.w * 0.5, 0.0) * &b1.q;
        let a2dpdt = &FORCE;
        let a2dldt = &TORQUE;

        let x = &s0.x + &(a2dxdt * halfdt);
        let q = &s0.q + &(&a2dqdt * halfdt);
        let p = &s0.p + &(a2dpdt * halfdt);
        let l = &s0.l + &(a2dldt * halfdt);
        let b2 = RigidBodyState::new(&x, &q, &p, &l, self.inv_mass, &self.inv_inertia);

        // a3 = G(t + dt / 2, b2), b3 = s0 + dt * a3
        let a3dxdt = &b2.v;
        let a3dqdt = &Quaternion::new(&b2.w * 0.5, 0.0) * &b2.q;
        let a3dpdt = &FORCE;
        let a3dldt = &TORQUE;

        let x = &s0.x + &(a3dxdt * dt);
        let q = &s0.q + &(&a3dqdt * dt);
        let p = &s0.p + &(a3dpdt * dt);
        let l = &s0.l + &(a3dldt * dt);
        let b3 = RigidBodyState::new(&x, &q, &p, &l, self.inv_mass, &self.inv_inertia);

        // a4 = G(t + dt, b4), s1 = s0 + (dt / 6) * (a1 + 2 * a2 + 2 * a3 + a4)
        let a4dxdt = &b3.v;
        let a4dqdt = &Quaternion::new(&b3.w * 0.5, 0.0) * &b3.q;
        let a4dpdt = &FORCE;
        let a4dldt = &TORQUE;

        let x = &s0.x
            + &(a1dxdt * sixthdt)
            + &(a2dxdt * thirddt)
            + &(a3dxdt * thirddt)
            + &(a4dxdt * sixthdt);
        let q = &s0.q
            + &(&a1dqdt * sixthdt)
            + &(&a2dqdt * thirddt)
            + &(&a3dqdt * thirddt)
            + &(&a4dqdt * sixthdt);
        let p = &s0.p
            + &(a1dpdt * sixthdt)
            + &(a2dpdt * thirddt)
            + &(a3dpdt * thirddt)
            + &(a4dpdt * sixthdt);
        let l = &s0.l
            + &(a1dldt * sixthdt)
            + &(a2dldt * thirddt)
            + &(a3dldt * thirddt)
            + &(a4dldt * sixthdt);
        let s1 = RigidBodyState::new(&x, &q, &p, &l, self.inv_mass, &self.inv_inertia);

        self.state = s1;
    }
}

pub struct World {
    objects: Vec<WorldObject>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(
        &mut self,
        body: impl RigidBody + 'static,
        x: &Vector,
        q: &Quaternion,
        v: &Vector,
        l: &Vector,
    ) {
        let inv_mass = 1.0 / body.mass();
        let inv_inertia = body.inertia_tensor().inverse().unwrap();
        let state = RigidBodyState::new(x, q, v, l, inv_mass, &inv_inertia);
        let object = WorldObject {
            state,
            inv_mass,
            inv_inertia,
            body: Box::new(body),
        };
        self.objects.push(object);
    }
    pub fn step(&mut self, t: f64, dt: f64) {
        for o in &mut self.objects {
            o.step(t, dt);
        }
    }
    pub fn draw(&self) {
        for o in &self.objects {
            o.body.draw(&o.state.x, &o.state.r);
        }
    }
}
