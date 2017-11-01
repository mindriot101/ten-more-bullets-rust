extern crate sdl2;
extern crate time;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;
mod gun;
mod entity;
mod game;
mod keymap;
mod bullet;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
