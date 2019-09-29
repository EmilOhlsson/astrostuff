use crate::entity::GameObject;
use crate::utils::wrap;
use graphics;
use graphics::polygon;
use opengl_graphics::GlGraphics;
use piston::input::*;
use piston::RenderArgs;

const COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct Player {
    poly: polygon::Polygon,
    points: Vec<[f64; 2]>,
    position: (f64, f64),
    speed: (f64, f64),
    direction: f64,
    radius: f64,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Player {
        let poly = polygon::Polygon::new(COLOR);
        let points = vec![[0.0, -5.0], [10.0, -5.0], [5.0, 15.0]];

        Player {
            poly,
            points,
            position: (x, y),
            speed: (0.0, 0.0),
            direction: 0.0,
            radius: 10.0,
        }
    }

    pub fn thrust(&mut self, power: f64) {
        let (dx, dy) = self.direction.sin_cos();
        self.speed.0 -= dx * power;
        self.speed.1 += dy * power;
    }

    pub fn turn(&mut self, dir: f64) {
        self.direction += dir;
    }
}

impl GameObject for Player {
    fn update(&mut self, args: UpdateArgs) {
        self.position.0 += self.speed.0 * args.dt;
        self.position.1 += self.speed.1 * args.dt;
    }

    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;
        self.position.0 = wrap(self.position.0, 0.0, args.window_size[0]);
        self.position.1 = wrap(self.position.1, 0.0, args.window_size[1]);
        gl.draw(args.viewport(), |c, g| {
            let (x, y) = (self.position.0, self.position.1);
            let transform = c.transform.trans(x, y).rot_rad(self.direction);
            self.poly.draw(&self.points, &c.draw_state, transform, g);
        });
    }

    fn radius(&self) -> f64 {
        self.radius
    }

    fn position(&self) -> (f64, f64) {
        self.position
    }
}
