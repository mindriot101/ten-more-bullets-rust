use entity::Entity;
use sdl2::render::WindowCanvas;

use keymap::KeyMap;

pub(crate) trait Enemy: Entity {}

#[allow(dead_code)]
pub(crate) struct NormalEnemy;

impl NormalEnemy {}

impl Entity for NormalEnemy {
    fn update(&mut self, _dt: f32, _keymap: &KeyMap) {}

    fn draw(&self, _renderer: &mut WindowCanvas) {}

    fn cleanup(&mut self) {}
}

impl Enemy for NormalEnemy {}
