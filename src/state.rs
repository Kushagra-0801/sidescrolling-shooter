use ggez;
use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};

use super::{Point, SCREEN_SIZE};
use crate::{
    enemy::{Enemy, ENEMY_SIZE, ENEMY_SPAWN_RATE},
    helpers::draw_line,
    player::{Dir, Shooter},
};

#[derive(Debug, Clone)]
pub struct State {
    player: Shooter,
    enemies: Vec<Enemy>,
    score: u32,
    distrib: rand::distributions::Bernoulli,
}

impl State {
    pub fn new() -> Self {
        Self {
            player: Shooter::new(),
            enemies: vec![Enemy::new(Point {
                x: SCREEN_SIZE.0 - 1.0,
                y: SCREEN_SIZE.1 / 2.0,
            })],
            score: 0,
            distrib: rand::distributions::Bernoulli::new(ENEMY_SPAWN_RATE).unwrap(),
        }
    }

    fn draw_score(&self, ctx: &mut Context) -> GameResult {
        let score = self.score.to_string();
        let mut text = graphics::Text::new(score);
        text.set_font(graphics::Font::default(), graphics::Scale::uniform(35.0));
        let text_position = Point {
            x: SCREEN_SIZE.0 / 2.0 - (text.width(ctx) as f32 / 2.0).ceil(),
            y: 0.0,
        };
        graphics::draw(
            ctx,
            &text,
            graphics::DrawParam::default().dest(text_position),
        )
    }

    fn shoot(&mut self) {
        let bullet_height = self.player.pos.y;
        let idx = self
            .enemies
            .iter()
            .enumerate()
            .filter(
                |(
                    _,
                    Enemy {
                        pos: Point { y, .. },
                    },
                )| {
                    bullet_height >= y - ENEMY_SIZE * 1.1 && bullet_height <= y + ENEMY_SIZE * 1.1
                },
            )
            .min_by(
                |(
                    _,
                    Enemy {
                        pos: Point { x: x1, .. },
                    },
                ),
                 (
                    _,
                    Enemy {
                        pos: Point { x: x2, .. },
                    },
                )| x1.partial_cmp(x2).unwrap(),
            );
        match idx {
            None => return,
            Some((idx, _)) => {
                self.score += 1;
                self.enemies.remove(idx);
            }
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.player.update(ctx);
        self.enemies.iter_mut().for_each(|i| i.update(ctx));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        draw_line(ctx);
        self.player.draw(ctx)?;
        for enemy in self.enemies.iter() {
            enemy.draw(ctx)?;
        }
        self.draw_score(ctx);
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: ggez::input::keyboard::KeyCode,
        _keymods: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        use ggez::input::keyboard::KeyCode::{Down, Space, Up, Q};
        match keycode {
            Space => self.shoot(),
            Q => ggez::event::quit(ctx),
            Up => self.player.dir = Dir::Up,
            Down => self.player.dir = Dir::Down,
            _ => (),
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: ggez::input::keyboard::KeyCode,
        _keymods: ggez::input::keyboard::KeyMods,
    ) {
        use ggez::input::keyboard::KeyCode::{Down, Up};
        match keycode {
            Up | Down => self.player.dir = Dir::None,
            _ => (),
        }
    }
}
