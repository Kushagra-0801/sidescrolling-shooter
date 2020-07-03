use crate::{
    player::{PLAYER_SIZE, PLAYER_X_POS},
    Point, SCREEN_SIZE,
};
use ggez::{graphics, Context, GameResult};

pub fn draw_line(ctx: &mut Context) -> GameResult {
    let line = graphics::Mesh::new_line(
        ctx,
        &[
            Point {
                x: PLAYER_X_POS + PLAYER_SIZE,
                y: 0.0,
            },
            Point {
                x: PLAYER_X_POS + PLAYER_SIZE,
                y: SCREEN_SIZE.1,
            },
        ],
        4.0,
        graphics::WHITE,
    )?;
    graphics::draw(ctx, &line, graphics::DrawParam::default())
}
