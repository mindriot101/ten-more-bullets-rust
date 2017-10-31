extern crate sdl2;
extern crate time;

mod gun;
mod entity;
mod game;
mod keymap;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
