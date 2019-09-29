use opengl_graphics::GlGraphics;
use piston::input::UpdateArgs;
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
    fn update(&mut self, args: UpdateArgs);
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs);
    fn radius(&self) -> f64;
    fn position(&self) -> (f64, f64);
}

pub fn collides(a: &dyn GameObject, b: &dyn GameObject) -> bool {
    let p0 = a.position();
    let p1 = b.position();
    let dx = (p0.0 - p1.0).abs();
    let dy = (p0.1 - p1.1).abs();
    let dist = dx * dx + dy * dy;
    let mindist = a.radius() + b.radius();
    dist < mindist * mindist
}
