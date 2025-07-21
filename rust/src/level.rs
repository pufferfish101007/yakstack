use crate::camera::Camera;
use crate::terrain::Terrain;
use crate::yak::Yak;
use godot::classes::{INode2D, Input, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Level {
    base: Base<Node2D>,
    yak_count: u32,
    yaks: Array<StringName>,
}

#[godot_api]
impl INode2D for Level {
    fn ready(&mut self) {
        let mut terrain = self
            .base()
            .try_get_node_as::<Terrain>("Terrain")
            .expect("`Level` node needs a `Terrain` child");
        terrain.bind_mut().setup();

        let mut camera = self
            .base()
            .try_get_node_as::<Camera>("Camera")
            .expect("`Level` must have Camera as child");
        camera.set_position(Vector2 { x: 578.0, y: 323.0 });

        self.spawn_yak();
    }

    fn physics_process(&mut self, _delta: f64) {
        if Input::singleton().is_action_just_pressed("ui_accept") {
            self.spawn_yak();
        }
    }
}

#[godot_api]
impl Level {
    fn spawn_yak(&mut self) {
        let mut yak = Yak::new_alloc();
        self.yaks.push(&yak.get_name());
        let camera_pos = self.get_camera().get_position();
        godot_print!("spawning yak with id {}", self.yak_count);
        yak.bind_mut().setup(
            Vector2 {
                x: camera_pos.x - 100.0,
                y: 120.0,
            },
            self.yak_count,
        );
        self.yak_count += 1;
        yak.signals()
            .screen_exited()
            .connect_other(self, |this, name| {
                // let mut camera = this.get_camera();
                // let zoom = camera.bind().get_target_zoom();
                // camera.bind_mut().set_target_zoom(zoom * 0.92);
                this.yaks.erase(&name);
                let mut yak = this.base().get_node_as::<Yak>(name.arg());
                this.base_mut().remove_child(&yak);
                yak.queue_free();
            });
        // yak.set_linear_velocity(Vector2 { x: 300.0, y: 0.0 });
        self.base_mut().add_child(&yak);
    }

    fn get_camera(&self) -> Gd<Camera> {
        self.base()
            .try_get_node_as::<Camera>("Camera")
            .expect("`Level` must have Camera as child")
    }
}

impl Level {
    pub const SPEED: f64 = 300.0;
}
