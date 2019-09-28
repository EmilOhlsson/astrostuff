use opengl_graphics::GlGraphics;
use piston::RenderArgs;

#[derive(Debug)]
pub struct Tangeable {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub dir: f64,
}

pub trait GameObject {
    fn update();
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs);
}
