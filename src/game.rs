use opengl_graphics::GlGraphics;
use piston::input::*;

use crate::entity::Entity;

const UNIT_MOVE: f64 = 0.5;
const UNIT_TURN: f64 = std::f64::consts::PI / 16.0;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

pub struct App {
    gl: GlGraphics,
    player: Entity,
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        App {
            gl,
            player: Entity {
                x: 100.0,
                y: 100.0,
                dx: 0.0,
                dy: 0.0,
                dir: 0.0,
            },
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        if self.player.x > args.window_size[0] {
            self.player.x -= args.window_size[0];
        }
        if self.player.y > args.window_size[1] {
            self.player.y -= args.window_size[1];
        }
        if self.player.x < 0.0 {
            self.player.x += args.window_size[0];
        }
        if self.player.y < 0.0 {
            self.player.y += args.window_size[1];
        }
        let player_shape = polygon::Polygon::new(WHITE);
        let player_points = [[0.0, -10.0], [20.0, -10.0], [10.0, 30.0]];

        let dir = self.player.dir;
        let (x, y) = (self.player.x, self.player.y);

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(BLACK, gl);
            let transform = c.transform.trans(x, y).rot_rad(dir).trans(-25.0, -25.0);
            player_shape.draw(&player_points, &c.draw_state, transform, gl);
        });
    }

    pub fn update(&mut self, _args: UpdateArgs) {
        self.player.x += self.player.dx;
        self.player.y += self.player.dy;
    }

    pub fn input(&mut self, button: &Button) {
        if let Button::Keyboard(key) = *button {
            match key {
                Key::Up => {
                    let (dx, dy) = self.player.dir.sin_cos();
                    self.player.dx -= dx * UNIT_MOVE;
                    self.player.dy += dy * UNIT_MOVE;
                }
                Key::Down => {
                    let (dx, dy) = self.player.dir.sin_cos();
                    self.player.dx += dx * UNIT_MOVE;
                    self.player.dy -= dy * UNIT_MOVE;
                }
                Key::Left => self.player.dir -= UNIT_TURN,
                Key::Right => self.player.dir += UNIT_TURN,
                _ => (),
            }
            println!("player: {:?}", self.player);
        }
    }
}