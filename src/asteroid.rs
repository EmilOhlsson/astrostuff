use crate::entity::GameObject;
use crate::utils::wrap;
use graphics;
use graphics::polygon;
use opengl_graphics::GlGraphics;
use piston::input::*;
use piston::RenderArgs;
use rand;
use rand::prelude::*;
use rand_distr::StandardNormal;

const RESOLUTION: usize = 1 << 4;
const RESINV: f64 = 1.0 / RESOLUTION as f64;
const COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

pub struct Asteroid {
    poly: polygon::Polygon,
    points: Vec<[f64; 2]>,
    position: (f64, f64),
    speed: (f64, f64),
    radius: f64,
}

impl Asteroid {
    pub fn new(radius: f64, x: f64, y: f64) -> Asteroid {
        let mut radiuses = [0f64; RESOLUTION];
        let mut rng = rand::thread_rng();
        radiuses[0] = radius + rng.sample::<f64, _>(StandardNormal);
        radiuses[RESOLUTION / 2] = radius + rng.sample::<f64, _>(StandardNormal);
        let speed: (f64, f64) = (rng.sample(StandardNormal), rng.sample(StandardNormal));

        fill_points(&mut radiuses, &mut rng, 0, RESOLUTION);

        let angles = (0..RESOLUTION)
            .map(|s| s as f64)
            .map(|s| s * std::f64::consts::PI * 2f64 * RESINV);

        let points = angles
            .zip(radiuses.iter())
            .map(|(a, r)| [r * a.sin(), r * a.cos()])
            .collect::<Vec<[f64; 2]>>();

        Asteroid {
            poly: polygon::Polygon::new(COLOR),
            points,
            position: (x, y),
            speed,
            radius,
        }
    }
}

impl GameObject for Asteroid {
    fn update(&mut self, args: UpdateArgs) {
        self.position.0 += 5.0 * self.speed.0 * args.dt;
        self.position.1 += 5.0 * self.speed.1 * args.dt;
    }

    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;
        self.position.0 = wrap(self.position.0, 0.0, args.window_size[0]);
        self.position.1 = wrap(self.position.1, 0.0, args.window_size[1]);
        gl.draw(args.viewport(), |c, g| {
            let (x, y) = (self.position.0, self.position.1);
            let transform = c.transform.trans(x, y);
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

// Private helper functions

/// Recursively fill radiuses with (rads[lo] + rads[hi]) + rnd
fn fill_points(rads: &mut [f64], rng: &mut rand::rngs::ThreadRng, lo: usize, hi: usize) {
    // This could be generalized and used to create explosions as well.
    // Just take a closure as parameter
    if hi - lo > 1 {
        let mid = (lo + hi) / 2;
        let mid_val = (rads[lo] + rads[hi % RESOLUTION]) / 2.0;
        rads[mid] = mid_val + rng.sample::<f64, _>(StandardNormal) * 10.0;
        fill_points(rads, rng, lo, mid);
        fill_points(rads, rng, mid, hi);
    }
}
