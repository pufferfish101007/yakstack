use godot::prelude::*;

pub mod yak;
pub mod terrain;
pub mod level;

pub use terrain::chunk;

struct YackStack;

#[gdextension]
unsafe impl ExtensionLibrary for YackStack {}