use crate::entity::GameObject;
use graphics::polygon;
use rand;
use rand_distr::Distribution;

const RESOLUTION: usize = 1 << 4;
const RESINV: f64 = 1.0 / RESOLUTION as f64;
const COLOR: [f32; 4] = [0.5, 0.5, 0.5, 0.5];

pub struct Asteroid {
    poly: polygon::Polygon,
    points: Vec<[f64; 2]>,
}

impl Asteroid {
    pub fn new(radius: f64, x: f64, y: f64) -> Asteroid {
        let mut radiuses = [0f64; RESOLUTION];
        let mut rng = rand::thread_rng();
        let normal = rand_distr::Normal::new(2.0, 3.0).unwrap();
        radiuses[0] = radius + normal.sample(&mut rng);
        radiuses[RESOLUTION / 2] = radius + normal.sample(&mut rng);

        fn fill_points(
            rads: &mut [f64],
            rng: &mut rand::rngs::ThreadRng,
            dist: &rand_distr::Normal<f64>,
            lo: usize,
            hi: usize,
        ) {
            if lo < hi {
                let mid = (lo + hi) / 2;
                let mid_val = (rads[lo] + rads[hi % RESOLUTION]) / 2.0;
                rads[mid] = mid_val + dist.sample(rng);
                fill_points(rads, rng, dist, lo, mid);
                fill_points(rads, rng, dist, mid, hi);
            }
        }
        fill_points(&mut radiuses, &mut rng, &normal, 0, RESOLUTION);

        println!("Generated radius: {:?}", radiuses);
        let angles = (0..RESOLUTION)
            .map(|s| s as f64)
            .map(|s| s * std::f64::consts::PI * 2f64 * RESINV);
        let points = angles
            .zip(radiuses.iter())
            .map(|(a, r)| [r * a.sin(), r * a.sin()])
            .collect::<Vec<[f64; 2]>>();
        Asteroid {
            poly: polygon::Polygon::new(COLOR),
            points: points,
        }
    }
}

impl GameObject for Asteroid {
    fn update() {}
    fn render() {}
}
