use crate::level::Level;
use godot::classes::rigid_body_2d::DampMode;
use godot::classes::{
    CollisionShape2D, IRigidBody2D, Input, PhysicsMaterial, RectangleShape2D, RigidBody2D, Sprite2D, Texture2D, VisibleOnScreenEnabler2D
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=RigidBody2D)]
pub struct Yak {
    base: Base<RigidBody2D>,
    has_landed: bool,
}

#[godot_api]
impl IRigidBody2D for Yak {
    fn physics_process(&mut self, delta: f64) {
        // let is_on_floor = self.base().is_on_floor();

        // let mut vel = self.base().get_velocity();
        // if !is_on_floor {
        //     let grav = self.base().get_gravity();
        //     vel += grav * (delta as f32);
        // }

        // if Input::singleton().is_action_just_pressed("ui_up") && is_on_floor {
        //     vel.y = Self::JUMP_VELOCITY;
        // }

        // if !self.has_landed && is_on_floor {
        //     self.has_landed = true;
        // }

        // if self.has_landed {
        //     vel.x = Level::SPEED as f32;
        // }

        // self.base_mut().set_velocity(vel);

        // self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Yak {
    #[func]
    pub fn setup(&mut self, position: Vector2) {
        self.base_mut().set_position(position);
        self.base_mut().set_linear_damp(0.0);
        self.base_mut().set_linear_damp_mode(DampMode::COMBINE);

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
