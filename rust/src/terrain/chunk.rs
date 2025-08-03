use godot::classes::{
    CollisionShape2D, IStaticBody2D, PhysicsMaterial, RectangleShape2D, Sprite2D, StaticBody2D,
    Texture2D, VisibleOnScreenNotifier2D,
};
use godot::global::randf;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=StaticBody2D)]
pub struct Chunk {
    base: Base<StaticBody2D>,
    #[export]
    length: i32,
    pub desired_position: Option<Vector2>,
}

#[godot_api]
impl IStaticBody2D for Chunk {
    fn ready(&mut self) {
        if self.get_length() < 1 {
            godot_error!("chunk must have length >= 1")
        }
    }
}

#[godot_api]
impl Chunk {
    #[func]
    pub fn setup(&mut self) {
        let mut visibility_notifier = VisibleOnScreenNotifier2D::new_alloc();
        visibility_notifier.set_rect(Rect2 {
            position: Vector2 { x: 0.0, y: 0.0 },
            size: Vector2 {
                x: self.get_length() as f32 * Self::TILE_SIZE,
                y: 100.0, // arbitrary height; chunks don't move up and down (for now)
            },
        });
        self.base_mut().add_child(&visibility_notifier);
        visibility_notifier
            .signals()
            .screen_exited()
            .connect_other(&self.to_gd(), |this| this.signals().screen_exited().emit());

        self.base_mut().set_modulate(Color {
            r: randf() as f32,
            g: randf() as f32,
            b: randf() as f32,
            a: 1.0,
        });

        let mut physics_material = PhysicsMaterial::new_gd();
        physics_material.set_friction(0.0);
        self.base_mut()
            .set_physics_material_override(&physics_material);
    }

    #[signal]
    pub fn screen_exited();

    #[func]
    pub fn generate_terrain(&mut self) {
        for child in self.base().get_children().iter_shared() {
            if child.get_class() != "VisibleOnScreenNotifier2D".into() {
                self.base_mut().remove_child(&child);
            }
        }

        let length = self.get_length();

        let mut ground = Sprite2D::new_alloc();
        ground.set_texture(&load::<Texture2D>("res://assets/tiles/green_square.svg"));
        ground.set_scale(Vector2 {
            x: length as f32,
            y: Self::GROUND_TILE_HEIGHT,
        });
        ground.set_position(Vector2 {
            x: 0.5 * Self::TILE_SIZE * length as f32,
            y: Self::GROUND_Y + Self::TILE_SIZE * Self::GROUND_TILE_HEIGHT * 0.5 - Self::TILE_SIZE * 0.5,
        });
        self.base_mut().add_child(&ground);

        let mut collision_shape = CollisionShape2D::new_alloc();
        let mut shape = RectangleShape2D::new_gd();
        shape.set_size(Vector2::new(
            Self::TILE_SIZE * length as f32,
            Self::TILE_SIZE * Self::GROUND_TILE_HEIGHT,
        ));
        collision_shape.set_shape(&shape);
        collision_shape.set_position(Vector2 {
            x: 0.5 * Self::TILE_SIZE * length as f32,
            y: Self::GROUND_Y + Self::TILE_SIZE * Self::GROUND_TILE_HEIGHT * 0.5 - Self::TILE_SIZE * 0.5,
        });
        self.base_mut().add_child(&collision_shape);

        for x in 0..length {
            if randf() > 0.6 {
                let mut sprite2d = Sprite2D::new_alloc();
                sprite2d.set_texture(&load::<Texture2D>("res://assets/tiles/green_square.svg"));
                sprite2d.set_position(Vector2 {
                    x: (Self::TILE_SIZE * x as f32) + Self::TILE_SIZE * 0.5,
                    y: Self::GROUND_Y - Self::TILE_SIZE,
                });
                // sprite2d.set_scale(Vector2::splat(1.0 + (1.0 / 16.0)));
                self.base_mut().add_child(&sprite2d);

                let mut collision_shape = CollisionShape2D::new_alloc();
                let mut shape = RectangleShape2D::new_gd();
                shape.set_size(Vector2::splat(Self::TILE_SIZE));
                collision_shape.set_shape(&shape);
                collision_shape.set_position(Vector2 {
                    x: (Self::TILE_SIZE * x as f32) + Self::TILE_SIZE * 0.5,
                    y: Self::GROUND_Y - Self::TILE_SIZE,
                });
                self.base_mut().add_child(&collision_shape);
            }
        }
    }
}

impl Chunk {
    pub const TILE_SIZE: f32 = 96.0;
    pub const GROUND_Y: f32 = Self::TILE_SIZE * 6.5;
    pub const GROUND_TILE_HEIGHT: f32 = 10.0;
}
