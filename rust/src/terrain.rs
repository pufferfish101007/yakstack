use godot::classes::{INode2D, Node2D, TileSet};
use godot::prelude::*;

pub mod chunk;

use chunk::Chunk;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Terrain {
    base: Base<Node2D>,
    #[export]
    tileset: Option<Gd<TileSet>>,
    #[export]
    chunks: Array<StringName>,
    #[export]
    auto_generate: bool,
}

#[godot_api]
impl Terrain {
    #[func]
    pub fn setup(&mut self) {
        if self.get_auto_generate() {
            assert!(
                self.chunks.is_empty(),
                "auto-generating terrain already has chunks specified"
            );
            for i in 0..3 {
                let mut new_chunk = Chunk::new_alloc();

                new_chunk.bind_mut().set_length(64);

                new_chunk.bind_mut().setup(
                    self.tileset
                        .as_ref()
                        .expect("`Terrain` must have a TileSet specified")
                        .clone(),
                );

                new_chunk.bind_mut().generate_terrain();

                let mut pos = new_chunk.get_position();
                pos.x += 64.0 * 32.0 * (i as f32);
                new_chunk.set_position(pos);

                self.base_mut().add_child(&new_chunk);

                self.chunks.push(&new_chunk.get_name());
            }
        } else {
            assert!(
                !self.chunks.is_empty(),
                "non-auto-generating terrain has no chunks specified"
            );
        }
    }
}
