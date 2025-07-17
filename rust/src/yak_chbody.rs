use crate::level::Level;
use core::convert::identity;
use godot::classes::{
    CharacterBody2D, CollisionShape2D, ICharacterBody2D, Input, RectangleShape2D, Shape2D,
    Sprite2D, Texture2D, VisibleOnScreenEnabler2D,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Yak {
    base: Base<CharacterBody2D>,
    id: u32,
    has_landed: bool,
}

#[godot_api]
impl ICharacterBody2D for Yak {
    fn physics_process(&mut self, delta: f64) {
        let is_on_floor = self.base().is_on_floor();

        let grav = self.base().get_gravity();

        let mut vel = self.base().get_velocity();
        // if !is_on_floor {
            vel += grav * (delta as f32);
        // }

        if Input::singleton().is_action_just_pressed("ui_up") && is_on_floor {
            vel.y = Self::JUMP_VELOCITY;
        }

        if !self.has_landed && is_on_floor {
            self.has_landed = true;
        }

        let touching_floor = (0..self.base().get_slide_collision_count())
            .filter_map(|i| self.base().clone().get_slide_collision(i))
            .filter_map(|collision| collision.get_collider())
            .any(|collider| {
                let class = collider.get_class();
                godot_print!("[{}] {class}", self.id);
                class == "Chunk".into()
            });
        godot_print!("[{}] {touching_floor}", self.id);
        // if touching_floor {
        //     vel.x = Level::SPEED as f32;
        //     godot_print!("[{}] set horizontal speed", self.id)
        // }
        if is_on_floor && !touching_floor {
            vel.x = 0.0;
        } else {
            vel.x = Level::SPEED as f32;
        }

        self.base_mut().set_velocity(vel);
        self.base_mut().move_and_slide();

        // let collision = self.base_mut().move_and_collide(vel * delta as f32);
        // if let Some(col) = collision
        //     && let Some(collider) = col.get_collider()
        //     { godot_print!("{}", collider.get_class());
        //     if collider.is_class("Chunk")
        // {
        //     self.base_mut().set_velocity(Vector2 { x: Level::SPEED as f32, y: grav.length() });
        // }}
        // godot_print!("[{}] {is_on_floor}", self.id);
        // if is_on_floor {
        //     self.base_mut().set_velocity(Vector2 { x: Level::SPEED as f32, y: grav.length() })
        // }
    }
}

#[godot_api]
impl Yak {
    #[func]
    pub fn setup(&mut self, position: Vector2, id: u32) {
        self.base_mut().set_position(position);
        self.base_mut().set_floor_stop_on_slope_enabled(false);

        self.id = id;

        let mut sprite2d = Sprite2D::new_alloc();
        sprite2d.set_texture(&load::<Texture2D>("res://yak1.svg"));
        sprite2d.set_scale(Vector2 { x: 0.422, y: 0.422 });
        self.base_mut().add_child(&sprite2d);

        let mut collision_shape = CollisionShape2D::new_alloc();
        let mut shape = RectangleShape2D::new_gd();
        shape.set_size(Vector2::new(41.0, 44.0));
        collision_shape.set_shape(&shape);
        collision_shape.set_position(Vector2 { x: -1.0, y: 12.0 });

        let mut visibility_notifier = VisibleOnScreenEnabler2D::new_alloc();
        visibility_notifier.set_rect(Rect2 {
            position: Vector2 { x: -10.0, y: 0.0 },
            size: Vector2 { x: 10.0, y: 44.0 },
        });
        visibility_notifier
            .signals()
            .screen_exited()
            .connect_other(&self.to_gd(), |this| this.signals().screen_exited().emit());
        collision_shape.add_child(&visibility_notifier);

        self.base_mut().add_child(&collision_shape);
    }

    #[signal]
    pub fn screen_exited();
}

impl Yak {
    const JUMP_VELOCITY: f32 = -600.0;
}
