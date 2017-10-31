use sdl2::render::WindowCanvas;

pub(crate) trait Entity {
    fn update(&mut self, dt: f32);
    fn draw(&self, renderer: &mut WindowCanvas);
}
