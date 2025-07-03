use godot::prelude::*;

pub mod yak;

struct YackStack;

#[gdextension]
unsafe impl ExtensionLibrary for YackStack {}