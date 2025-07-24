use godot::classes::rigid_body_2d::CcdMode;
use godot::classes::{
    CollisionShape2D, PhysicsMaterial, RectangleShape2D, RigidBody2D, Sprite2D, Texture2D,
    VisibleOnScreenEnabler2D,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=RigidBody2D)]
pub struct Yak {
    base: Base<RigidBody2D>,
    id: u32,
}

#[godot_api]
impl Yak {
    #[func]
    pub fn setup(&mut self, position: Vector2, id: u32) {
        self.base_mut().set_position(position);
        // if yaks can rotate, they'll start spiralling everywhere if they hit a corner - not fun
        self.base_mut().set_lock_rotation_enabled(true);
        // because the ground is moving and not the yaks, yaks may sleep and then end up hovering in mid air
        self.base_mut().set_can_sleep(false);
        self.base_mut().set_continuous_collision_detection_mode(CcdMode::CAST_SHAPE);

        let mut physics_material = PhysicsMaterial::new_gd();
        // yaks must be frictionless so that if a base yak stops but the yaks above it are unimpeded,
        // they will continue moving (relative to the ground) ass normal, rather than being stuck to the ground
        physics_material.set_friction(0.0);
        physics_material.set_bounce(0.0);
        self.base_mut()
            .set_physics_material_override(&physics_material);

        self.id = id;

        let mut sprite2d = Sprite2D::new_alloc();
        sprite2d.set_texture(&load::<Texture2D>("res://assets/yak/walk1.svg"));
        sprite2d.set_scale(Vector2 { x: 2.0, y: 2.0 });
        self.base_mut().add_child(&sprite2d);

        let mut collision_shape = CollisionShape2D::new_alloc();
        let mut shape = RectangleShape2D::new_gd();
        shape.set_size(Vector2::new(96.0, 96.0));
        collision_shape.set_shape(&shape);
        collision_shape.set_position(Vector2 { x: -12.0, y: 32.0 });

        let mut visibility_notifier = VisibleOnScreenEnabler2D::new_alloc();
        visibility_notifier.set_rect(Rect2 {
            position: Vector2 { x: 0.0, y: 0.0 },
            size: Vector2 { x: 96.0, y: 96.0 },
        });
        visibility_notifier
            .signals()
            .screen_exited()
            .connect_other(&self.to_gd(), |this| {
                let name = &this.base().get_name();
                this.signals().screen_exited().emit(name);
            });
        collision_shape.add_child(&visibility_notifier);

        self.base_mut().add_child(&collision_shape);
    }

    #[signal]
    pub fn screen_exited(name: StringName);
}
