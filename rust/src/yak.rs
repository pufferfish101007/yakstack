use crate::level::Level;
use crate::resource::load_texture;
use godot::builtin::math::ApproxEq;
use godot::classes::rigid_body_2d::CcdMode;
use godot::classes::{
    CollisionShape2D, ConvexPolygonShape2D, IRigidBody2D, PhysicsMaterial, RectangleShape2D,
    RigidBody2D, Shape2D, Sprite2D, Texture2D, VisibleOnScreenEnabler2D,
};
use godot::prelude::*;

#[derive(Copy, Clone)]
pub enum YakCostume {
    Walk,
    Crouch,
}

impl YakCostume {
    fn asset_name(&self) -> &str {
        match self {
            Self::Walk => "walk1",
            Self::Crouch => "crouch1",
        }
    }

    pub fn asset_path(&self) -> String {
        format!("res://assets/yak/{}", self.asset_name())
    }

    fn shape(&self) -> Gd<Shape2D> {
        match self {
            Self::Walk => {
                let mut shape = RectangleShape2D::new_gd();
                shape.set_size(Vector2::new(96.0, 96.0));
                shape.upcast()
            }
            Self::Crouch => {
                let mut shape = ConvexPolygonShape2D::new_gd();
                shape.set_points(&PackedVector2Array::from([
                    Vector2 { x: 32.0, y: 93.0 },
                    Vector2 { x: 139.0, y: 59.0 },
                    Vector2 { x: 130.0, y: 159.0 },
                    Vector2 { x: 32.0, y: 159.0 },
                ]));
                shape.upcast()
            }
        }
    }

    fn shape_offset(&self) -> Vector2 {
        match self {
            Self::Walk => Vector2 { x: -12.0, y: 32.0 },
            Self::Crouch => Vector2 { x: -90.0, y: -81.0 },
        }
    }
}

#[derive(GodotClass)]
#[class(init, base=RigidBody2D)]
pub struct Yak {
    base: Base<RigidBody2D>,
    id: u32,
    sprite2d: Option<Gd<Sprite2D>>,
    collision_shape: Option<Gd<CollisionShape2D>>,
}

#[godot_api]
impl IRigidBody2D for Yak {
    fn physics_process(&mut self, _delta: f32) {
        if self.is_stuck() && self.base().get_linear_velocity().y.is_zero_approx() {
            self.base_mut()
                .set_linear_velocity(Vector2 { x: 0.0, y: -10.0 });
        }
    }
}

#[godot_api]
impl Yak {
    #[signal]
    pub fn screen_exited(yak: Gd<Yak>);
}

impl Yak {
    pub fn setup(&mut self, position: Vector2, id: u32) {
        self.base_mut().set_position(position);
        // if yaks can rotate, they'll start spiralling everywhere if they hit a corner - not fun
        self.base_mut().set_lock_rotation_enabled(true);
        // because the ground is moving and not the yaks, yaks may sleep and then end up hovering in mid air
        self.base_mut().set_can_sleep(false);
        // try to stop falling yaks from sinking into the floor too much
        self.base_mut()
            .set_continuous_collision_detection_mode(CcdMode::CAST_SHAPE);
        self.base_mut().set_contact_monitor(true);
        self.base_mut().set_max_contacts_reported(32);

        let mut physics_material = PhysicsMaterial::new_gd();
        // yaks must be frictionless so that if a base yak stops but the yaks above it are unimpeded,
        // they will continue moving (relative to the ground) ass normal, rather than being stuck to the ground
        physics_material.set_friction(0.0);
        physics_material.set_bounce(0.0);
        self.base_mut()
            .set_physics_material_override(&physics_material);

        self.id = id;

        let sprite2d = Sprite2D::new_alloc();
        self.sprite2d = Some(Gd::clone(&sprite2d));

        let mut collision_shape = CollisionShape2D::new_alloc();
        self.collision_shape = Some(Gd::clone(&collision_shape));

        self.set_costume(YakCostume::Walk);
        self.base_mut().add_child(&sprite2d);

        let mut visibility_notifier = VisibleOnScreenEnabler2D::new_alloc();
        visibility_notifier.set_rect(Rect2 {
            position: Vector2 { x: 0.0, y: 0.0 },
            size: Vector2 { x: 200.0, y: 96.0 },
        });
        visibility_notifier
            .signals()
            .screen_exited()
            .connect_other(&self.to_gd(), |this| {
                let this_gd = this.to_gd();
                this.signals().screen_exited().emit(&this_gd);
            });
        collision_shape.add_child(&visibility_notifier);

        self.base_mut().add_child(&collision_shape);
    }

    pub fn set_costume(&mut self, costume: YakCostume) {
        self.sprite2d
            .clone()
            .unwrap()
            .set_texture(&load_texture::<_, Texture2D>(costume.asset_path()));
        let shape = costume.shape();
        let shape_offset = costume.shape_offset();

        self.collision_shape.clone().unwrap().set_shape(&shape);
        self.collision_shape
            .clone()
            .unwrap()
            .set_position(shape_offset);
    }

    /// All yaks are very supportive of whatever struggles you're going through
    /// right now. You got this <3
    ///
    /// But this particular function determines if this yak is actively supporting other
    /// yaks (physically).
    pub fn is_supportive(&self) -> bool {
        let my_position = self.base().get_position();
        self.base()
            .get_colliding_bodies()
            .iter_shared()
            .any(|friend| {
                // remember that godot's y coordinates are upside down from what we'd expect!
                friend.get_position().y < my_position.y
                    // a yak could be higher up than this one, but not be supported by it,
                    // so make sure that the x coordinates overlap too
                    && f32::abs(friend.get_position().x - my_position.x) < 96.0
            })
    }

    pub fn is_stuck(&self) -> bool {
        self.base()
            .get_linear_velocity()
            .x
            .approx_eq(&(-Level::SPEED as f32))
    }
}
