use sdl2::render::WindowCanvas;

use keymap::KeyMap;

pub(crate) trait Entity {
    fn update(&mut self, dt: f32, keymap: &KeyMap);
    fn draw(&self, renderer: &mut WindowCanvas);
    fn cleanup(&mut self);
}
