use godot::classes::{ISprite2D, Input, InputEvent, Sprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Yak {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Yak {
    fn init(base: Base<Sprite2D>) -> Self {
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

    fn input(&mut self, event: Gd<InputEvent>) {
        godot_print!("{}", event.to_string());

        if event.is_action("ui_down") {
            self.increase_speed(-2.0);
            godot_print!("down");
        }
        if event.is_action("ui_up") {
            self.increase_speed(2.0);
            godot_print!("up");
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        // godot_print!("{:?}", self.base().get_process_mode());

        // godot_print!("{}", input.is_anything_pressed());

        // if input.is_action_pressed("ui_down") {
        //     self.increase_speed(-2.0);
        //     godot_print!("down");
        // }
        // if input.is_action_pressed("ui_up") {
        //     self.increase_speed(2.0);
        //     godot_print!("up");
        // }

        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);

        let rotation = self.base().get_rotation();
        let velocity = Vector2::RIGHT.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);
    }
}

#[godot_api]
impl Yak {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.base_mut().emit_signal("speed_increased", &[]);
    }

    #[signal]
    fn speed_increased();
}
