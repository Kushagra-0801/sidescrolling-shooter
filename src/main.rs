#![allow(unused)]

use ggez;
use ggez::event::EventHandler;
use ggez::mint;
use ggez::GameResult;
use ggez::{conf, graphics, Context};

const SCREEN_SIZE: (f32, f32) = (800.0, 500.0);
const PLAYER_SPEED: f32 = SCREEN_SIZE.1 / 100.0;
const PLAYER_SIZE: f32 = 10.0;
const PLAYER_X_POS: f32 = SCREEN_SIZE.0 / 40.0 * 3.0;
const ENEMY_SPEED: f32 = SCREEN_SIZE.0 / 150.0;
const ENEMY_SIZE: f32 = 20.0;

type Point = mint::Point2<f32>;
type Vector = mint::Vector2<f32>;

#[derive(Debug, Clone)]
struct State {
    player: Shooter,
    enemies: Vec<Enemy>,
    score: u32,
}

impl State {
    fn new() -> Self {
        Self {
            player: Shooter {
                pos: Point {
                    x: PLAYER_X_POS,
                    y: SCREEN_SIZE.1 / 2.0,
                },
                dir: Dir::None,
            },
            enemies: vec![Enemy {
                pos: Point {
                    x: SCREEN_SIZE.0 - 1.0,
                    y: SCREEN_SIZE.1 / 2.0,
                },
            }],
            score: 0,
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

fn draw_line(ctx: &mut Context) -> GameResult {
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

#[derive(Debug, Clone)]
enum Dir {
    Up,
    Down,
    None,
}

#[derive(Debug, Clone)]
struct Shooter {
    pos: Point,
    dir: Dir,
}

impl Shooter {
    fn update(&mut self, ctx: &mut Context) {
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
    fn draw(&self, ctx: &mut Context) -> GameResult {
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

#[derive(Debug, Clone)]
struct Enemy {
    pos: Point,
}

impl Enemy {
    fn update(&mut self, ctx: &mut Context) {
        self.pos.x -= ENEMY_SPEED;
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
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

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("Shooter", "Kushagra")
        .window_setup(conf::WindowSetup::default().title("Shooter"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;
    let state = &mut State::new();
    ggez::event::run(&mut ctx, &mut event_loop, state)
}
