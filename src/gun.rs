use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

use entity::Entity;
use keymap::KeyMap;
use bullet::Bullet;
use globals::DEVEL_MODE;

pub(crate) struct Gun {
    rect_geometry: Rect,
    fired_last_frame: bool,
    bullets: Vec<Bullet>,
    dead_bullet_indices: Vec<usize>,
    bullets_fired: u32,
    nbullets_max: Option<u32>,
}

impl Gun {
    pub(crate) fn new(screen_width: u32, screen_height: u32, nbullets_max: u32) -> Self {
        let (width, height) = (20, 20);

        Gun {
            rect_geometry: rect!((screen_width / 2), (screen_height - height), width, height),
            fired_last_frame: false,
            bullets: Vec::new(),
            dead_bullet_indices: Vec::new(),
            bullets_fired: 0,
            nbullets_max: if nbullets_max == 0 {
                None
            } else {
                Some(nbullets_max)
            },
        }
    }

    fn fire(&mut self) {
        self.bullets_fired += 1;
        if let Some(nbullets_max) = self.nbullets_max {
            let mut devel_mode = DEVEL_MODE.lock().unwrap();
            if !*devel_mode && self.bullets_fired >= (nbullets_max as _) {
                return;
            }
        }

        let center_x = (self.rect_geometry.x as f32) + (self.rect_geometry.h as f32) / 2.0;
        let center_y = (self.rect_geometry.y as f32) + ((self.rect_geometry.width() as f32) / 2.0);
        let new_bullet = Bullet::new(
            center_x as _, center_y as _
        );
        self.bullets.push(new_bullet);
    }

    pub(crate) fn game_over(&self) -> bool {
        let devel_mode = DEVEL_MODE.lock().unwrap();
        !*devel_mode && (self.bullets_fired >= 10) && (self.bullets.len() == 0)
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

        for (i, mut bullet) in self.bullets.iter_mut().enumerate() {
            if !bullet.active() {
                self.dead_bullet_indices.push(i);
                continue;
            }
            bullet.update(dt, keymap);
        }
    }

    fn draw(&self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(Color::RGB(255, 210, 0));
        renderer.fill_rect(self.rect_geometry).expect(
            "cannot render gun",
        );

        for bullet in self.bullets.iter() {
            if !bullet.active() {
                continue;
            }

            bullet.draw(renderer);
        }
    }

    fn cleanup(&mut self) {
        for index in self.dead_bullet_indices.iter() {
            self.bullets.remove(*index);
        }
        self.dead_bullet_indices.clear();

        for bullet in self.bullets.iter_mut() {
            bullet.cleanup();
        }
    }
}
