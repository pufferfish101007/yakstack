use crate::resource;
use crate::{level::Level, yak::YakCostume};
use godot::{
    classes::{AtlasTexture, Texture2D, TextureButton, texture_button::StretchMode},
    prelude::*,
};

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

impl Level {
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

        button.set_position(Vector2 {
            x: 24.0 + 128.0 * idx as f32,
            y: 24.0,
        });
        self.base_mut().add_child(&button);
    }

    fn yak_config(&mut self, config: YakConfig) {
        match config {
            YakConfig::Conga => {
                for yak in &mut self.yaks {
                    yak.bind_mut().set_costume(YakCostume::Crouch);
                }
            }
            YakConfig::Stack => {
                for yak in &mut self.yaks {
                    yak.bind_mut().set_costume(YakCostume::Walk);
                }
            }
        }
    }
}
