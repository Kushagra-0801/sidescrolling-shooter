use super::{Point, SCREEN_SIZE};
use ggez::{graphics, Context, GameResult};

pub const PLAYER_SPEED: f32 = SCREEN_SIZE.1 / 100.0;
pub const PLAYER_SIZE: f32 = 10.0;
pub const PLAYER_X_POS: f32 = SCREEN_SIZE.0 / 40.0 * 3.0;

#[derive(Debug, Clone)]
pub enum Dir {
    Up,
    Down,
    None,
}

#[derive(Debug, Clone)]
pub struct Shooter {
    pub pos: Point,
    pub dir: Dir,
}

impl Shooter {
    pub fn new() -> Self {
        Self {
            pos: Point {
                x: PLAYER_X_POS,
                y: SCREEN_SIZE.1 / 2.0,
            },
            dir: Dir::None,
        }
    }
    pub fn update(&mut self, ctx: &mut Context) {
        match self.dir {
            Dir::Up => self.pos.y -= PLAYER_SPEED,
            Dir::Down => self.pos.y += PLAYER_SPEED,
            Dir::None => (),
        }
        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
        } else if self.pos.y >= SCREEN_SIZE.1 {
            self.pos.y = SCREEN_SIZE.1 - 1.0;
        }
    }

    // Change to a triangle
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos,
            PLAYER_SIZE,
            0.1,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())
    }
}
