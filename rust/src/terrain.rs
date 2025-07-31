use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

pub mod chunk;

use chunk::Chunk;

use crate::level::Level;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Terrain {
    base: Base<Node2D>,
    #[export]
    chunk_names: Array<StringName>,
    chunks: Vec<Gd<Chunk>>,
    #[export]
    auto_generate: bool,
}

#[godot_api]
impl INode2D for Terrain {
    fn physics_process(&mut self, delta: f64) {
        for chunk in &mut self.chunks {
            let desired_position = chunk.bind_mut().desired_position;
            let pos = if let Some(p) = desired_position {
                chunk.bind_mut().desired_position = None;
                p
            } else {
                chunk.get_position()
            };
            chunk.set_position(
                pos + Vector2 {
                    x: -(Level::SPEED * delta) as f32,
                    y: 0.0,
                },
            );
        }
    }
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

                new_chunk.bind_mut().setup();

                new_chunk.bind_mut().generate_terrain();

                let mut pos = new_chunk.get_position();
                pos.x += (Self::AUTO_CHUNK_LEN as f32 * Chunk::TILE_SIZE)
                    * (i as f32);
                new_chunk.set_position(pos);

                self.base_mut().add_child(&new_chunk);

                self.chunks.push(new_chunk);
            }
        } else {
            assert!(
                !self.chunks.is_empty(),
                "non-auto-generating terrain has no chunks specified"
            );

            for chunk_name in self.chunk_names.iter_shared() {
                let chunk = self.base().get_node_as::<Chunk>(chunk_name.arg());
                self.chunks.push(chunk);
            }
        }

        for chunk in &mut self.chunks {
            chunk.signals().screen_exited().connect_self(|this| {
                let mut pos = this.base().get_position();
                pos.x +=
                    Self::AUTO_CHUNK_LEN as f32 * Chunk::TILE_SIZE * Self::AUTO_GEN_CHUNKS as f32;
                this.desired_position = Some(pos);
                this.generate_terrain();
            });
            chunk.set_constant_linear_velocity(Vector2 {
                x: -Level::SPEED as f32,
                y: 0.0,
            });
        }
    }
}

impl Terrain {
    const AUTO_GEN_CHUNKS: i32 = 3;
    const AUTO_CHUNK_LEN: i32 = 16;
}
