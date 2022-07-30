mod math;
mod world;

use crate::math::vector::Vector;
use crate::world::rigid_box::RigidBox;
use crate::world::world::World;

fn main() {
    let mut world = World::new();
    let rbox = RigidBox::new(1.0, 0.1, 0.5, 1.0);

    world.add(
        0,
        &rbox,
        &Vector::new(-10.0, 0.0, 10.0),
        &math::quaternion::Quaternion::from_rotation(&Vector::new(0.0, 1.0, 0.0), 0.0),
        &Vector::new(1.0, 0.0, 0.0),
        &Vector::new(0.0, 0.0, 0.0),
    );

    let mut t = 0.0;
    let dt = 1.0 / 20.0;

    while t < 1.0 {
        t = world.step(t, dt);
        world.for_each_object(|_body_id, position, _rotation| {
            println!("Box @ {}, {}, {}", position.x, position.y, position.z);
        });
    }
}
