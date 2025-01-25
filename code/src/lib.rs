mod components;
mod entities;
mod levels;
mod spawner;

use godot::prelude::*;

struct Sustain;

#[gdextension]
unsafe impl ExtensionLibrary for Sustain {}
