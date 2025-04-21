mod module_bindings;
mod player;
mod spacetime_manager;
use godot::prelude::*;

struct SpaceTagExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SpaceTagExtension {}
