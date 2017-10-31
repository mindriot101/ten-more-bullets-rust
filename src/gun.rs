use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

use entity::Entity;
use keymap::KeyMap;
use bullet::Bullet;

pub(crate) struct Gun {
    rect_geometry: Rect,
    fired_last_frame: bool,
    bullets: Vec<Bullet>,
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
            bullets: Vec::new(),
        }
    }

    fn fire(&mut self) {
        let new_bullet = Bullet::new(
            self.rect_geometry.x as _,
            (self.rect_geometry.y as f32) + ((self.rect_geometry.width() / 2) as f32),
        );
        self.bullets.push(new_bullet);
    }
}

impl Entity for Gun {
    fn update(&mut self, dt: f32, keymap: &KeyMap) {
        let fire_pressed = keymap.is_pressed(Keycode::Space);

        if fire_pressed && !self.fired_last_frame {
            self.fire();
            self.fired_last_frame = true;
        }

        if self.fired_last_frame && !fire_pressed {
            self.fired_last_frame = false;
        }

        /* TODO(srw) cloning here is a wasteful allocation. Try and find some way to scan through
         * this array initially immutably, and then mutably later */
        for (i, bullet) in self.bullets.clone().iter().enumerate() {
            if !bullet.active() {
                self.bullets.remove(i);
            }
        }

        for mut bullet in self.bullets.iter_mut() {
            bullet.update(dt, keymap);
        }
    }

    fn draw(&self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(Color::RGB(255, 210, 0));
        renderer.fill_rect(self.rect_geometry).expect(
            "cannot render gun",
        );

        for bullet in self.bullets.iter() {
            bullet.draw(renderer);
        }
    }
}
