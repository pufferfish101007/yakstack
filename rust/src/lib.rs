use godot::prelude::*;

pub mod camera;
pub mod level;
pub mod terrain;
pub mod yak;
mod yak_config;
mod resource;

pub use terrain::chunk;

struct YackStack;

#[gdextension]
unsafe impl ExtensionLibrary for YackStack {}
