use std::sync::Mutex;

lazy_static! {
    pub static ref TTF_CONTEXT: ::sdl2::ttf::Sdl2TtfContext = ::sdl2::ttf::init().unwrap();
    pub static ref DEVEL_MODE: Mutex<bool> = Mutex::new(true);
}
