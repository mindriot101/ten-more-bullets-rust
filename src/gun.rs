use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

use entity::Entity;

pub(crate) struct Gun {
    rect_geometry: Rect,
}

impl Gun {
    pub(crate) fn new(screen_width: u32, screen_height: u32) -> Self {
        let (width, height) = (20, 20);

        Gun {
            rect_geometry: Rect::new(
                (screen_width / 2) as _,
                (screen_height - height) as _,
                width,
                height,
            ),
        }
    }
}

impl Entity for Gun {
    fn update(&mut self, _dt: f32) {}

    fn draw(&self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(Color::RGB(255, 210, 0));
        renderer.fill_rect(self.rect_geometry).expect(
            "cannot render gun",
        );
    }
}
