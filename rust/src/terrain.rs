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
            for i in 0..Self::AUTO_GEN_CHUNKS {
                let mut new_chunk = Chunk::new_alloc();

                new_chunk.bind_mut().set_length(Self::AUTO_CHUNK_LEN);

                new_chunk.bind_mut().setup(
                    self.tileset
                        .as_ref()
                        .expect("`Terrain` must have a TileSet specified")
                        .clone(),
                );

                new_chunk.bind_mut().generate_terrain();

                let mut pos = new_chunk.get_position();
                pos.x += Self::AUTO_CHUNK_LEN as f32 * 32.0 * (i as f32);
                new_chunk.set_position(pos);

                new_chunk.signals().screen_exited().connect_self(|this| {
                    let mut pos = this.base().get_position();
                    pos.x += Self::AUTO_CHUNK_LEN as f32 * 32.0 * (Self::AUTO_GEN_CHUNKS as f32);
                    this.base_mut().set_position(pos);
                    this.generate_terrain();
                });

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

impl Terrain {
    const AUTO_GEN_CHUNKS: i32 = 3;
    const AUTO_CHUNK_LEN: i32 = 64;
}
