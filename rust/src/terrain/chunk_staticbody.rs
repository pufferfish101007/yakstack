use godot::classes::{
    CollisionShape2D, IStaticBody2D, RectangleShape2D, Sprite2D, StaticBody2D, Texture2D, TileSet,
    VisibleOnScreenNotifier2D,
};
use godot::global::randf;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=StaticBody2D)]
pub struct Chunk {
    base: Base<StaticBody2D>,
    #[export]
    length: i32,
}

#[godot_api]
impl IStaticBody2D for Chunk {
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
    pub fn setup(&mut self) {
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
        for child in self.base().get_children().iter_shared() {
            if child.get_class() != "VisibleOnScreenNotifier2D".into() {
                self.base_mut().remove_child(&child);
            }
        }

        let length = self.get_length();

        let mut ground = Sprite2D::new_alloc();
        ground.set_texture(&load::<Texture2D>("res://green_square (1).svg"));
        ground.set_scale(Vector2 {
            x: length as f32,
            y: 1.0 + (1.0 / 16.0),
        });
        ground.set_position(Vector2 {
            x: 0.0,
            y: Self::GROUND_HEIGHT,
        });
        self.base_mut().add_child(&ground);

        let mut collision_shape = CollisionShape2D::new_alloc();
        let mut shape = RectangleShape2D::new_gd();
        shape.set_size(Vector2::new(32.0 * length as f32, 32.0));
        collision_shape.set_shape(&shape);
        collision_shape.set_position(Vector2 {
            x: 0.0,
            y: Self::GROUND_HEIGHT,
        });
        self.base_mut().add_child(&collision_shape);

        for x in 0..self.get_length() {
            if randf() > 0.5 {
                let mut sprite2d = Sprite2D::new_alloc();
                sprite2d.set_texture(&load::<Texture2D>("res://green_square (1).svg"));
                sprite2d.set_position(Vector2 {
                    x: 32.0 * x as f32,
                    y: Self::GROUND_HEIGHT + 32.0,
                });
                sprite2d.set_scale(Vector2::splat(1.0 + 1.0 / 16.0));
                self.base_mut().add_child(&sprite2d);

                let mut collision_shape = CollisionShape2D::new_alloc();
                let mut shape = RectangleShape2D::new_gd();
                shape.set_size(Vector2::new(32.0, 32.0));
                collision_shape.set_shape(&shape);
                collision_shape.set_position(Vector2 {
                    x: 32.0,
                    y: Self::GROUND_HEIGHT + 32.0,
                });
                self.base_mut().add_child(&collision_shape);
            }
        }
    }
}

impl Chunk {
    const GROUND_HEIGHT: f32 = 32.0 * 15.0; // 15 blocks below top of camera
}
