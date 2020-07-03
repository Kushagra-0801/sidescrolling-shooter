use ggez::{graphics, Context, GameResult};

use super::{Point, SCREEN_SIZE};

const ENEMY_SPEED: f32 = SCREEN_SIZE.0 / 150.0;
pub const ENEMY_SIZE: f32 = 20.0;
pub const ENEMY_SPAWN_RATE: f64 = 0.4;

#[derive(Debug, Clone)]
pub struct Enemy {
    pub pos: Point,
}

impl Enemy {
    pub fn new(pos: Point) -> Self {
        Self { pos }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.pos.x -= ENEMY_SPEED;
    }
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let enemy = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: self.pos.x - ENEMY_SIZE / 2.0,
                y: self.pos.y - ENEMY_SIZE / 2.0,
                w: ENEMY_SIZE,
                h: ENEMY_SIZE,
            },
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &enemy, graphics::DrawParam::default())
    }
}
