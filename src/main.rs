use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use astrostuff::game::App;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("AstroStuff", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(i) = e.press_args() {
            app.input(&i);
        }
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(u);
        }
    }
}
