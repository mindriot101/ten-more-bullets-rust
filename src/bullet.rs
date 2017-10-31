use keymap::KeyMap;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

#[derive(Debug, Clone)]
pub(crate) struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    rect_geometry: Rect,
}

impl Bullet {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Bullet {
            x: x,
            y: y,
            vx: 0.0,
            vy: -100.0,
            rect_geometry: Rect::new(x as _, y as _, 5, 5),
        }
    }

    pub(crate) fn active(&self) -> bool {
        true
    }
}

impl ::entity::Entity for Bullet {
    fn update(&mut self, dt: f32, _keymap: &KeyMap) {
        let newx = self.x + dt * self.vx;
        let newy = self.y + dt * self.vy;
        self.x = newx;
        self.y = newx;
        self.rect_geometry.x = self.x as _;
        self.rect_geometry.y = self.y as _;
    }

    fn draw(&self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.fill_rect(self.rect_geometry).expect(
            "cannot render bullet",
        );
    }
}
