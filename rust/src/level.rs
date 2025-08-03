use crate::camera::Camera;
use crate::resource;
use crate::terrain::Terrain;
use crate::yak::{Yak, YakCostume};
use godot::classes::{
    AtlasTexture, INode2D, Input, Node2D, Texture2D, TextureButton, texture_button::StretchMode,
};
use godot::prelude::*;

#[derive(Clone, Copy)]
pub enum YakConfig {
    Stack,
    Conga,
}

impl ToString for YakConfig {
    fn to_string(&self) -> String {
        use YakConfig::*;
        match self {
            Stack => "stack",
            Conga => "conga",
        }
        .to_owned()
    }
}

impl Default for YakConfig {
    fn default() -> Self {
        Self::Stack
    }
}

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Level {
    base: Base<Node2D>,
    yak_count: u32,
    yaks: Vec<Gd<Yak>>,
    yak_config: Option<YakConfig>,
    configing_yaks: Vec<Gd<Yak>>,
    buttons: Vec<Gd<TextureButton>>,
}

#[godot_api]
impl INode2D for Level {
    fn ready(&mut self) {
        let mut terrain = self
            .base()
            .try_get_node_as::<Terrain>("Terrain")
            .expect("`Level` node needs a `Terrain` child");
        terrain.bind_mut().setup();

        let mut camera = self
            .base()
            .try_get_node_as::<Camera>("Camera")
            .expect("`Level` must have Camera as child");
        camera.set_position(Vector2 { x: 578.0, y: 323.0 });

        let config_buttons = {
            use YakConfig as YC;
            [YC::Stack, YC::Conga]
        };
        for (i, config) in config_buttons.into_iter().enumerate() {
            self.make_button(config, i);
        }

        self.spawn_yak();
    }

    fn physics_process(&mut self, _delta: f64) {
        if Input::singleton().is_action_just_pressed("ui_accept") {
            self.spawn_yak();
        }

        if let Some(config) = self.yak_config {
            match config {
                YakConfig::Conga => {
                    self.configing_yaks.retain_mut(|yak| {
                        // TODO: we shouldn't have to check if the instance is valid!!
                        // just remove it from `configing_yaks` when it's dropped.
                        yak.is_instance_valid()
                            && if yak.bind().is_supportive() && !yak.bind().is_stuck() {
                                yak.bind_mut().set_costume(YakCostume::Crouch);
                                true
                            } else {
                                yak.bind_mut().set_costume(YakCostume::Walk);
                                let mut velocity = yak.get_linear_velocity();
                                velocity.x = 0.0;
                                yak.set_linear_velocity(velocity);
                                false
                            }
                    });
                }
                YakConfig::Stack => {
                    for yak in &mut self.yaks {
                        yak.bind_mut().set_costume(YakCostume::Walk);
                    }
                    godot_warn!("todo: stack config");
                }
            }
            if self.configing_yaks.is_empty() {
                self.yak_config = None;
                self.set_buttons_disabled(false);
            }
        }
    }
}

impl Level {
    pub const SPEED: f64 = 300.0;

    fn spawn_yak(&mut self) {
        let mut yak = Yak::new_alloc();
        self.yaks.push(Gd::clone(&yak));
        let camera_pos = self.get_camera().get_position();
        yak.bind_mut().setup(
            Vector2 {
                x: camera_pos.x - 100.0,
                y: 120.0,
            },
            self.yak_count,
        );
        self.yak_count += 1;
        yak.signals()
            .screen_exited()
            .connect_other(self, |this, mut yak| {
                // let mut camera = this.get_camera();
                // let zoom = camera.bind().get_target_zoom();
                // camera.bind_mut().set_target_zoom(zoom * 0.92);
                this.yaks.retain(|el| el != &yak);
                this.base_mut().remove_child(&yak);
                yak.queue_free();
            });
        // yak.set_linear_velocity(Vector2 { x: 300.0, y: 0.0 });
        self.base_mut().add_child(&yak);
    }

    fn get_camera(&self) -> Gd<Camera> {
        self.base()
            .try_get_node_as::<Camera>("Camera")
            .expect("`Level` must have Camera as child")
    }

    pub fn make_button(&mut self, config: YakConfig, idx: usize) {
        let mut button = TextureButton::new_alloc();

        button.set_custom_minimum_size(Vector2 { x: 96.0, y: 96.0 });
        button.set_stretch_mode(StretchMode::SCALE);

        let texture = resource::load_texture::<_, Texture2D>(format!(
            "res://assets/buttons/{}",
            config.to_string()
        ));

        let mut normal_atlas = AtlasTexture::new_gd();
        normal_atlas.set_atlas(&texture);
        normal_atlas.set_region(Rect2 {
            position: Vector2 { x: 0.0, y: 0.0 },
            size: Vector2 { x: 96.0, y: 96.0 },
        });
        button.set_texture_normal(&normal_atlas);

        let mut disabled_atlas = AtlasTexture::new_gd();
        disabled_atlas.set_atlas(&texture);
        disabled_atlas.set_region(Rect2 {
            position: Vector2 { x: 96.0, y: 0.0 },
            size: Vector2 { x: 96.0, y: 96.0 },
        });
        button.set_texture_disabled(&disabled_atlas);

        let mut hover_atlas = AtlasTexture::new_gd();
        hover_atlas.set_atlas(&texture);
        hover_atlas.set_region(Rect2 {
            position: Vector2 { x: 0.0, y: 96.0 },
            size: Vector2 { x: 96.0, y: 96.0 },
        });
        button.set_texture_hover(&hover_atlas);

        let mut pressed_atlas = AtlasTexture::new_gd();
        pressed_atlas.set_atlas(&texture);
        pressed_atlas.set_region(Rect2 {
            position: Vector2 { x: 96.0, y: 96.0 },
            size: Vector2 { x: 96.0, y: 96.0 },
        });
        button.set_texture_pressed(&pressed_atlas);

        button
            .signals()
            .pressed()
            .connect_other(self, move |this| this.yak_config(config));

        button
            .signals()
            .pressed()
            .connect_self(|this| this.release_focus());

        button.set_position(Vector2 {
            x: 24.0 + 128.0 * idx as f32,
            y: 24.0,
        });

        self.base_mut().add_child(&button);

        self.buttons.push(button);
    }

    fn set_buttons_disabled(&mut self, disabled: bool) {
        for button in &mut self.buttons {
            button.set_disabled(disabled);
        }
    }

    fn yak_config(&mut self, config: YakConfig) {
        self.set_buttons_disabled(true);

        self.yak_config = Some(config);

        match config {
            YakConfig::Conga => {
                for yak in &mut self.yaks {
                    if yak.bind().is_supportive() {
                        yak.bind_mut().set_costume(YakCostume::Crouch);
                        self.configing_yaks.push(Gd::clone(yak));
                    }
                }
            }
            YakConfig::Stack => {
                for yak in &mut self.yaks {
                    yak.bind_mut().set_costume(YakCostume::Walk);
                }
                self.configing_yaks.clear();
                godot_warn!("todo: stack config");
            }
        }
    }
}
