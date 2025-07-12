use crate::level::Level;
use godot::classes::{
    CharacterBody2D, CollisionShape2D, ICharacterBody2D, Input, RectangleShape2D, Shape2D,
    Sprite2D, Texture2D,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Yak {
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Yak {
    fn physics_process(&mut self, delta: f64) {
        let is_on_floor = self.base().is_on_floor();

        let mut vel = self.base().get_velocity();
        if !self.base().is_on_floor() {
            let grav = self.base().get_gravity();
            vel += grav * (delta as f32);
        }

        if Input::singleton().is_action_just_pressed("ui_up") && is_on_floor {
            vel.y = Self::JUMP_VELOCITY;
        }

        vel.x = Level::SPEED as f32;

        self.base_mut().set_velocity(vel);

        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Yak {
    #[func]
    pub fn setup(&mut self) {
        self.base_mut().set_position(Vector2 { x: 400.0, y: 120.0 });

        let mut sprite2d = Sprite2D::new_alloc();
        sprite2d.set_texture(&load::<Texture2D>("res://yak1.svg"));
        sprite2d.set_scale(Vector2 { x: 0.422, y: 0.422 });
        self.base_mut().add_child(&sprite2d);

        let mut collision_shape = CollisionShape2D::new_alloc();
        let mut shape = RectangleShape2D::new_gd();
        shape.set_size(Vector2::new(41.0, 44.0));
        collision_shape.set_shape(&shape);
        collision_shape.set_position(Vector2 { x: -1.0, y: 12.0 });
        self.base_mut().add_child(&collision_shape);
    }
}

impl Yak {
    const JUMP_VELOCITY: f32 = -600.0;
}
