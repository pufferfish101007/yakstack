use crate::terrain::Terrain;
use crate::yak::Yak;
use godot::classes::{Camera2D, INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Level {
    base: Base<Node2D>,
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
            .try_get_node_as::<Camera2D>("Camera2D")
            .expect("`Level` must have Camera2D as child");
        camera.set_position(Vector2 { x: 578.0, y: 323.0 });

        let mut yak = Yak::new_alloc();
        self.yaks.push(&yak.get_name());
        yak.bind_mut().setup();
        self.base_mut().add_child(&yak);
    }

    fn physics_process(&mut self, delta: f64) {
        let mut camera = self
            .base()
            .try_get_node_as::<Camera2D>("Camera2D")
            .expect("`Level` must have Camera2D as child");
        let mut pos = camera.get_position();
        pos.x += (Self::SPEED * delta) as f32;
        camera.set_position(pos);
    }
}

impl Level {
    pub const SPEED: f64 = 300.0;
}
