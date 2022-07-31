use crate::math::vector::Vector;
use crate::world::rigid_box::RigidBox;
use crate::world::world::World;
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::window::Window;
use math::quaternion::Quaternion;
use nalgebra::{Isometry3, Point3, Translation3, UnitQuaternion, Vector3};

mod math;
mod world;

const RECT_X: f32 = 4.0;
const RECT_Y: f32 = 2.0;
const RECT_Z: f32 = 3.0;

fn main() {
    let mut world = World::new(Vector::new(0.0, 0.0, -1.0));
    let rbox = RigidBox::new(RECT_X as f64, RECT_Y as f64, RECT_Z as f64, 1.0);

    world.add(
        0,
        &rbox,
        &Vector::new(-10.0, 0.0, 10.0),
        &math::quaternion::Quaternion::from_rotation(&Vector::new(0.0, 1.0, 0.0), 0.0),
        &Vector::new(10.0, 0.0, 0.0),
        &Vector::new(1.0, 10.0, 4.0),
    );

    let mut window = Window::new("Physics Engine");

    let mut c = window.add_cube(RECT_X, RECT_Y, RECT_Z);
    c.set_color(1.0, 1.0, 1.0);

    let mut floor = window.add_cube(100.0, 100.0, 1.0);
    floor.append_translation(&Translation3::new(0.0, 0.0, -0.5));
    floor.set_color(0.2, 0.2, 0.5);

    let mut xp = window.add_cube(10.0, 0.05, 0.05);
    xp.append_translation(&Translation3::new(5.0, 0.0, 0.0));
    xp.set_color(1.0, 0.0, 0.0);

    let mut yp = window.add_cube(0.05, 10.0, 0.05);
    yp.append_translation(&Translation3::new(0.0, 5.0, 0.0));
    yp.set_color(0.0, 1.0, 0.0);

    let mut zp = window.add_cube(0.05, 0.05, 10.0);
    zp.append_translation(&Translation3::new(0.0, 0.0, 5.0));
    zp.set_color(0.0, 0.0, 1.0);

    window.set_light(Light::StickToCamera);

    let mut camera = ArcBall::new(Point3::new(4.0, -30.0, 4.0), Point3::origin());
    camera.set_up_axis(Vector3::z());

    let mut t = 0.0;

    while window.render_with_camera(&mut camera) {
        t = world.step(t, 0.05);
        world.for_each_object(|_object_id: usize, p: &Vector, q: &Quaternion| {
            c.set_local_translation(Translation3::new(p.x as f32, p.y as f32, p.z as f32));
            c.set_local_rotation(UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(
                q.w as f32,
                q.v.x as f32,
                q.v.y as f32,
                q.v.z as f32,
            )));
        })
    }
}
