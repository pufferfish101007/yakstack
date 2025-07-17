use godot::prelude::*;

pub mod yak_chbody;
pub mod terrain;
pub mod level;
pub mod camera;

pub use terrain::chunk_staticbody;

struct YackStack;

#[gdextension]
unsafe impl ExtensionLibrary for YackStack {}