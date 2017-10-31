use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

use entity::Entity;
use keymap::KeyMap;

pub(crate) struct Gun {
    rect_geometry: Rect,
    fired_last_frame: bool,
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
            fired_last_frame: false,
        }
    }

    fn fire(&mut self, _dt: f32) {}
}

impl Entity for Gun {
    fn update(&mut self, dt: f32, keymap: &KeyMap) {
        let fire_pressed = keymap.is_pressed(Keycode::Space);

        if fire_pressed && !self.fired_last_frame {
            self.fire(dt);
            self.fired_last_frame = true;
        }

        if self.fired_last_frame && !fire_pressed {
            self.fired_last_frame = false;
        }
    }

    fn draw(&self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(Color::RGB(255, 210, 0));
        renderer.fill_rect(self.rect_geometry).expect(
            "cannot render gun",
        );
    }
}
