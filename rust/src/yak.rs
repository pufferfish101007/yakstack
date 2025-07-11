use godot::classes::{CharacterBody2D, ICharacterBody2D, Input, InputEvent};
use godot::global::move_toward;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Yak {
    speed: f64,
    angular_speed: f64,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Yak {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI / 1.5,
            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_process_unhandled_key_input(true);
    }

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

        let direction = Input::singleton().get_axis("ui_left", "ui_right");

        if direction != 0.0 {
            vel.x = Self::SPEED * direction;
        } else {
            vel.x = move_toward(vel.x as f64, 0.0, Self::SPEED as f64 / 20.0) as f32;
        }

        self.base_mut().set_velocity(vel);

        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Yak {}

impl Yak {
    const JUMP_VELOCITY: f32 = -600.0;
    const SPEED: f32 = 300.0;
}
