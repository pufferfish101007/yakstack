use godot::classes::{Camera2D, ICamera2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Camera2D)]
pub struct Camera {
    base: Base<Camera2D>,
    #[init(val = 1.0)]
    #[var]
    pub target_zoom: f32,
}

#[godot_api]
impl ICamera2D for Camera {
    fn physics_process(&mut self, delta: f64) {
        let zoom = self.base().get_zoom();
        let new_zoom = zoom.lerp(Vector2::splat(self.target_zoom), delta as f32 * 3.0);
        self.base_mut().set_zoom(new_zoom);
    }
}
