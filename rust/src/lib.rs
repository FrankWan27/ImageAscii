use godot::prelude::*;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

mod ascii;
mod draw;
mod image;
mod utils;
