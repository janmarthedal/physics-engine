use std::time::Duration;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
// use speedy2d::shape::Rectangle;
use crate::math::vector::Vector;
use crate::world::rigid_box::RigidBox;
use crate::world::world::World;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

mod math;
mod world;

const RECT_X: f64 = 5.0;
const RECT_Y: f64 = 0.1;
const RECT_Z: f64 = 4.0;

struct MyWindowHandler {
    world: World,
    t: f64,
    dt: f64,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.t = self.world.step(self.t, self.dt);
        println!("Time {}", self.t);

        graphics.clear_screen(Color::WHITE);

        self.world.for_each_object(|_body_id, position, rotation| {
            let c1 = rotation * Vector::new(-RECT_X / 2.0, 0.0, -RECT_Z / 2.0) + position;
            let c2 = rotation * Vector::new(-RECT_X / 2.0, 0.0, RECT_Z / 2.0) + position;
            let c3 = rotation * Vector::new(RECT_X / 2.0, 0.0, RECT_Z / 2.0) + position;
            let c4 = rotation * Vector::new(RECT_X / 2.0, 0.0, -RECT_Z / 2.0) + position;
            let vs = [
                Vector2::new(400.0 + 40.0 * c1.x as f32, 600.0 - 40.0 * c1.z as f32),
                Vector2::new(400.0 + 40.0 * c2.x as f32, 600.0 - 40.0 * c2.z as f32),
                Vector2::new(400.0 + 40.0 * c3.x as f32, 600.0 - 40.0 * c3.z as f32),
                Vector2::new(400.0 + 40.0 * c4.x as f32, 600.0 - 40.0 * c4.z as f32)
            ];
            graphics.draw_quad(vs, Color::from_rgb(0.8, 0.9, 1.0));
        });

        if self.t < 10.0 {
            std::thread::sleep(Duration::from_millis(100));
            helper.request_redraw();
        }
    }
}

fn main() {
    let mut world = World::new(Vector::new(0.0, 0.0, -1.0));
    let rbox = RigidBox::new(RECT_X, RECT_Y, RECT_Z, 1.0);

    world.add(
        0,
        &rbox,
        &Vector::new(-10.0, 0.0, 10.0),
        &math::quaternion::Quaternion::from_rotation(&Vector::new(0.0, 1.0, 0.0), 0.0),
        &Vector::new(10.0, 0.0, 0.0),
        &Vector::new(0.0, 10.0, 0.0),
    );

    let window = Window::new_centered("Physics engine", (800, 600)).unwrap();

    window.run_loop(MyWindowHandler {
        world,
        t: 0.0,
        dt: 1.0 / 100.0,
    });
}
