use keymap::KeyMap;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

const WIDTH: i32 = 5;

#[derive(Debug, Clone)]
pub(crate) struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Bullet {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        let pos_x = x - ((WIDTH as f32) / 2.0);

        Bullet {
            x: pos_x,
            y: y,
            vx: 0.0,
            vy: -100.0,
        }
    }

    pub(crate) fn active(&self) -> bool {
        self.y >= 0.0
    }
}

impl ::entity::Entity for Bullet {
    fn update(&mut self, dt: f32, _keymap: &KeyMap) {
        let newx = self.x + dt * self.vx;
        let newy = self.y + dt * self.vy;
        self.x = newx;
        self.y = newy;
    }

    fn draw(&self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(Color::RGB(0, 0, 0));

        let rect_geometry = rect!(self.x, self.y, WIDTH, WIDTH);
        renderer.fill_rect(rect_geometry).expect(
            "cannot render bullet",
        );
    }

    fn cleanup(&mut self) {}
}
