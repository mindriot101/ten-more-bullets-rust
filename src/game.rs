use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

use gun::Gun;
use entity::Entity;
use keymap::KeyMap;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const SIMULATION_DT: f32 = 0.016;
const CLEAR_COLOUR: Color = Color {
    r: 0x32,
    g: 0x4C,
    b: 0x32,
    a: 0xFF,
};

pub struct Game {
    canvas: Canvas<Window>,
    sdl_context: Sdl,
    running: bool,
    gun: Box<Entity>,
    keymap: KeyMap,
}

impl Game {
    pub fn new() -> Self {
        let sdl_context = ::sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("SDL2", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().present_vsync().build().unwrap();

        Game {
            canvas: canvas,
            sdl_context: sdl_context,
            running: true,
            gun: Box::new(Gun::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
            keymap: KeyMap::new(),
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut start_time = ::time::precise_time_s();
        let mut accumulator = 0.0f32;
        loop {
            if !self.running {
                break;
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


            accumulator += dt;
            while accumulator >= SIMULATION_DT {
                self.update(accumulator);
                accumulator -= SIMULATION_DT;
            }
            self.draw();
            self.cleanup();
        }
    }

    fn update(&mut self, dt: f32) {
        self.gun.update(dt, &self.keymap);
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => self.running = false,
            Event::KeyDown { keycode: Some(keycode), .. } => self.keymap.mark(keycode.into()),
            Event::KeyUp { keycode: Some(keycode), .. } => self.keymap.clear(keycode.into()),
            _ => {}
        }
    }

    fn draw(&mut self) {
        self.clear();
        self.gun.draw(&mut self.canvas);
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
}
