#[derive(Debug)]
pub struct Tangeable {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub dir: f64,
}

pub trait GameObject {
    fn update() {}

    fn render() {}
}
