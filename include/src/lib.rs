// Set up all files in the library to use the logging facade.
// Note that configuration is done in the main executable, not in the library.
#[macro_use]
extern crate log;
extern crate specs;

// Define the modules this library exports.
pub mod entity;
pub mod event;
pub mod input_handlers;
pub mod render_functions;
pub mod map;
pub mod map_utils;
pub mod game_state;

// ECS
pub mod component;
pub mod system;
