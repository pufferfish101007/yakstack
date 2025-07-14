use godot::classes::{ITileMapLayer, TileMapLayer, TileSet, VisibleOnScreenNotifier2D};
use godot::global::randf;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=TileMapLayer)]
pub struct Chunk {
    base: Base<TileMapLayer>,
    #[export]
    length: i32,
}

#[godot_api]
impl ITileMapLayer for Chunk {
    fn ready(&mut self) {
        if self.get_length() < 1 {
            godot_error!("chunk must have length >= 1")
        }
    }

    fn physics_process(&mut self, delta: f64) {}
}

#[godot_api]
impl Chunk {
    #[func]
    pub fn setup(&mut self, tileset: Gd<TileSet>) {
        self.base_mut().set_tile_set(&tileset);
        self.base_mut().set_collision_enabled(true);
        self.base_mut().set_scale(Vector2 { x: 2.0, y: 2.0 });

        let mut visibility_notifier = VisibleOnScreenNotifier2D::new_alloc();
        visibility_notifier.set_rect(Rect2 {
            position: Vector2 { x: 0.0, y: 0.0 },
            size: Vector2 {
                x: self.get_length() as f32 * 32.0,
                y: 20.0 * 32.0,
            },
        });
        self.base_mut().add_child(&visibility_notifier);
        visibility_notifier
            .signals()
            .screen_exited()
            .connect_other(&self.to_gd(), |this| this.signals().screen_exited().emit());
    }

    #[signal]
    pub fn screen_exited();

    #[func]
    pub fn generate_terrain(&mut self) {
        for x in 0..self.get_length() {
            self.base_mut()
                .set_cell_ex(Vector2i { x, y: 15 })
                .atlas_coords(Vector2i { x: 0, y: 0 })
                .source_id(0)
                .done();
            if randf() > 0.5 {
                self.base_mut()
                    .set_cell_ex(Vector2i { x, y: 16 })
                    .atlas_coords(Vector2i { x: 0, y: 0 })
                    .source_id(0)
                    .done();
            }
        }
    }
}
