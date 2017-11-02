use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use gun::Gun;
use entity::Entity;
use keymap::KeyMap;
use game_config::GameConfig;

const SIMULATION_DT: f32 = 0.016;
const CLEAR_COLOUR: Color = Color {
    r: 0x32,
    g: 0x4C,
    b: 0x32,
    a: 0xFF,
};

lazy_static! {
    static ref TTF_CONTEXT: ::sdl2::ttf::Sdl2TtfContext = ::sdl2::ttf::init().unwrap();
}

pub struct Game<'a, 'b> {
    canvas: Canvas<Window>,
    sdl_context: Sdl,
    running: bool,
    gun: Gun,
    keymap: KeyMap,
    debug_mode: bool,
    texture_creator: ::sdl2::render::TextureCreator<::sdl2::video::WindowContext>,
    font: ::sdl2::ttf::Font<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(config_filename: String) -> Self {
        let game_config = GameConfig::parse(config_filename);
        let sdl_context = ::sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("SDL2", game_config.screen_width, game_config.screen_height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().present_vsync().build().unwrap();
        let texture_creator: ::sdl2::render::TextureCreator<::sdl2::video::WindowContext> =
            canvas.texture_creator();

        let arial = TTF_CONTEXT
            .load_font("run_tree/fonts/Roboto-Regular.ttf", 32)
            .expect("cannot load arial.ttf");

        Game {
            canvas: canvas,
            sdl_context: sdl_context,
            running: true,
            gun: Gun::new(
                game_config.screen_width,
                game_config.screen_height,
                game_config.n_bullets,
            ),
            keymap: KeyMap::new(),
            debug_mode: true,
            texture_creator: texture_creator,
            font: arial,
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut start_time = ::time::precise_time_s();
        loop {
            if !self.running {
                break;
            }

            if self.gun.game_over() {
                self.show_game_over_screen();
            }

            let now = ::time::precise_time_s();
            let dt = {
                let raw_dt = (now - start_time) as f32;
                if raw_dt > 0.1f32 {
                    SIMULATION_DT
                } else {
                    raw_dt
                }
            };

            start_time = now;

            for event in event_pump.poll_iter() {
                self.handle_event(event);
            }


            self.update(dt);
            self.draw();
            self.cleanup();
        }
    }

    fn show_game_over_screen(&mut self) {
        /* TODO(srw) finish implementing this */
        self.running = false;
    }

    fn update(&mut self, dt: f32) {
        self.gun.update(dt, &self.keymap);
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => self.running = false,
            Event::KeyDown { keycode: Some(Keycode::T), .. } => self.toggle_debug(),
            Event::KeyDown { keycode: Some(keycode), .. } => self.keymap.mark(keycode.into()),
            Event::KeyUp { keycode: Some(keycode), .. } => self.keymap.clear(keycode.into()),
            _ => {}
        }
    }

    fn draw(&mut self) {
        self.clear();
        self.gun.draw(&mut self.canvas);

        if self.debug_mode {
            self.render_debug_ui();
        }

        self.blit();
    }

    fn cleanup(&mut self) {
        self.gun.cleanup();
    }

    fn blit(&mut self) {
        self.canvas.present();
    }

    fn clear(&mut self) {
        self.canvas.set_draw_color(CLEAR_COLOUR);
        self.canvas.clear();
    }

    fn toggle_debug(&mut self) {
        self.debug_mode = !self.debug_mode;
    }

    fn render_debug_ui(&mut self) {
        self.render_text("DEBUG", 0, 0);
    }

    fn render_text(&mut self, text: &str, x: i32, y: i32) {
        let surface = self.font
            .render(text)
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let texture = self.texture_creator
            .create_texture_from_surface(&surface)
            .expect("cannot create texture from font surface");

        let TextureQuery { width, height, .. } = texture.query();

        let target = rect!(x, y, width, height);
        self.canvas.copy(&texture, None, Some(target)).expect(
            "cannot render text",
        );
    }
}
