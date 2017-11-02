extern crate sdl2;
extern crate time;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
mod macros;
mod globals;
mod gun;
mod entity;
mod game;
mod keymap;
mod bullet;
mod game_config;

use std::env::args;

fn main() {
    let config_filename = args().nth(1).unwrap_or("gameconfig.toml".to_string());
    let mut game = game::Game::new(config_filename);
    game.run();
}
