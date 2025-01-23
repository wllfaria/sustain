mod components;
mod entities;
mod levels;

use godot::prelude::*;

struct Sustain;

#[gdextension]
unsafe impl ExtensionLibrary for Sustain {}
