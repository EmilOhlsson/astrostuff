use opengl_graphics::GlGraphics;
use piston::input::*;

use crate::asteroid::Asteroid;
use crate::entity::{collides, GameObject};
use crate::player::Player;

use rand::prelude::*;
use rand_distr::StandardNormal;

const UNIT_MOVE: f64 = 2.5;
const UNIT_TURN: f64 = std::f64::consts::PI / 16.0;

const SPACE: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

pub enum GameOutcome {
    Win,
    Lose,
    Ongoing,
}

pub struct App {
    gl: GlGraphics,
    player: Player,
    asteroids: Vec<Asteroid>,
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        App {
            gl,
            player: Player::new(200.0, 200.0),
            asteroids: (0..10)
                .map(|_| {
                    Asteroid::new(
                        40.0,
                        100.0 + thread_rng().sample::<f64, _>(StandardNormal) * 100.0,
                        100.0 + thread_rng().sample::<f64, _>(StandardNormal) * 100.0,
                    )
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        graphics::clear(SPACE, &mut self.gl);
        for asteroid in &mut self.asteroids {
            asteroid.render(&mut self.gl, args);
        }
        self.player.render(&mut self.gl, args);
    }

    pub fn update(&mut self, args: UpdateArgs) -> GameOutcome {
        self.player.update(args);
        for asteroid in &mut self.asteroids {
            asteroid.update(args);
            if collides(asteroid, &self.player) {
                return GameOutcome::Lose;
            }
        }
        GameOutcome::Ongoing
    }

    pub fn input(&mut self, button: &Button) {
        if let Button::Keyboard(key) = *button {
            match key {
                Key::Up => {
                    self.player.thrust(UNIT_MOVE);
                }
                Key::Down => {
                    self.player.thrust(-UNIT_MOVE);
                }
                Key::Left => self.player.turn(-UNIT_TURN),
                Key::Right => self.player.turn(UNIT_TURN),
                Key::Space => {
                    println!("pew!");
                }
                _ => (),
            }
        }
    }
}
