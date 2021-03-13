// lib.rs is like the entry point of the game.
// Data is created and distributed here
// Handlers are ticked here
// The game is rendered here
// And input is detected here
// (Might be worth to split this up)

use ggez::event::{KeyCode, KeyMods};
use ggez::*;

mod ball_system;
mod game_data;
pub mod input;
mod key_codes;
mod message_queue;
use input::*;

pub fn run() {
    let state = &mut game_data::GameState::new(
        nalgebra::Point2::new(400.0, 100.0),
        nalgebra::Vector2::new(100.0, 200.0),
        10.0,
        nalgebra::Point2::new(400.0, 600.0),
        50.0,
        3.0,
        9,
        5,
        30.0,
        800.0,
        600.0,
        10.0,
    );
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("breakout", "bjandra")
        .conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state).unwrap();
}

impl ggez::event::EventHandler for game_data::GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.active {
            self.dt = timer::delta(ctx);

            let mut delta_move = nalgebra::Vector2::<f32>::zeros();

            if self.input_state.is_key_pressed(Key::A) || self.input_state.is_key_pressed(Key::Left)
            {
                delta_move.x += -self.player_speed;
            }

            if self.input_state.is_key_pressed(Key::D)
                || self.input_state.is_key_pressed(Key::Right)
            {
                delta_move.x += self.player_speed;
            }

            let speed = 100.0;
            self.player_positon += delta_move * self.dt.as_secs_f32() * speed;
            if self.player_positon.x > self.window_width - self.player_radius {
                self.player_positon.x = self.window_width - self.player_radius;
            } else if self.player_positon.x < self.player_radius {
                self.player_positon.x = self.player_radius;
            }

            let mut game_messages = ball_system::tick(self);
            self.queue.append(&mut game_messages);
        }
        message_queue::tick(self);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.7, 0.7, 0.7, 1.0));

        // Draw text first
        draw_text(ctx, "Bjandra Breakout", 0.0, 100.0)?;

        // Then reset
        graphics::set_transform(ctx, graphics::DrawParam::default().to_matrix());
        graphics::apply_transformations(ctx)?;

        // Then draw other drawables
        draw_ball(ctx, &self.ball_position, self.ball_radius)?;
        draw_player(ctx, &self.player_positon, self.player_radius)?;
        draw_walls(ctx, &self.walls)?;
        draw_blocks(
            ctx,
            self.block_width,
            self.block_height,
            &self.blocks,
            self.block_diameter,
            self.window_width,
            self.window_height,
        )?;

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        if !repeat {
            self.input_state.set_key_pressed(keycode, true);
        }
        match keycode {
            key_codes::START_GAME_KEYCODE => {
                self.queue.push(game_data::GameMessage::StartGame);
            }
            key_codes::END_GAME_KEYCODE => {
                self.queue.push(game_data::GameMessage::EndGame);
            }
            key_codes::BALL_FLY_FROM_RIGHT_KEYCODE => {
                self.queue.push(game_data::GameMessage::BallFlyFromRight);
            }
            key_codes::BALL_FLY_FROM_LEFT_KEYCODE => {
                self.queue.push(game_data::GameMessage::BallFlyFromLeft);
            }
            key_codes::BALL_FLY_FROM_TOP_KEYCODE => {
                self.queue.push(game_data::GameMessage::BallFlyFromTop);
            }
            key_codes::BALL_FLY_FROM_BOTTOM_KEYCODE => {
                self.queue.push(game_data::GameMessage::BallFlyFromBottom);
            }
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.input_state.set_key_pressed(keycode, false);
    }
}

fn draw_text(ctx: &mut Context, message: &str, x: f32, y: f32) -> GameResult<()> {
    let font = graphics::Font::default();
    let mut text = graphics::Text::new(message);
    text.set_font(font, graphics::Scale::uniform(40.0));
    text.set_bounds(nalgebra::Point2::new(800.0, 100.0), graphics::Align::Center);
    graphics::draw(
        ctx,
        &text,
        graphics::DrawParam::default()
            .dest(nalgebra::Point2::new(x, y))
            .color(graphics::WHITE),
    )?;
    Ok(())
}

fn draw_ball(
    ctx: &mut Context,
    ball_position: &nalgebra::Point2<f32>,
    ball_radius: f32,
) -> GameResult<()> {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        *ball_position,
        ball_radius,
        2.0,
        graphics::WHITE,
    )
    .unwrap();
    graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
    Ok(())
}

fn draw_player(
    ctx: &mut Context,
    player_positon: &nalgebra::Point2<f32>,
    player_radius: f32,
) -> GameResult<()> {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        *player_positon,
        player_radius,
        2.0,
        graphics::WHITE,
    )
    .unwrap();
    graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
    Ok(())
}

fn draw_walls(ctx: &mut Context, walls: &Vec<game_data::Wall>) -> GameResult<()> {
    for wall in walls.iter() {
        let image = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(wall.position.x, wall.position.y, wall.width, wall.height),
            graphics::Color::new(0.5, 0.5, 0.5, 1.0),
        )
        .unwrap();
        graphics::draw(ctx, &image, graphics::DrawParam::default())?;
    }
    Ok(())
}

fn draw_blocks(
    ctx: &mut Context,
    blocks_width: u32,
    blocks_height: u32,
    blocks: &Vec<bool>,
    block_diameter: f32,
    window_width: f32,
    window_height: f32,
) -> GameResult<()> {
    for i_h in 0..blocks_height {
        for i_w in 0..blocks_width {
            if blocks[(blocks_width * i_h + i_w) as usize] {
                let image = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        (i_w as f32 - blocks_width as f32 * 0.5) * block_diameter
                            + window_width * 0.5,
                        (i_h as f32 - blocks_height as f32 * 0.5) * block_diameter
                            + window_height * 0.5,
                        block_diameter,
                        block_diameter,
                    ),
                    graphics::Color::new(0.0, 0.0, 0.0, 1.0),
                )
                .unwrap();
                graphics::draw(ctx, &image, graphics::DrawParam::default())?;

                let smaller_block_diameter = block_diameter * 0.8;
                let padding = block_diameter * 0.1;
                let image = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        (i_w as f32 - blocks_width as f32 * 0.5) * block_diameter
                            + window_width * 0.5
                            + padding,
                        (i_h as f32 - blocks_height as f32 * 0.5) * block_diameter
                            + window_height * 0.5
                            + padding,
                        smaller_block_diameter,
                        smaller_block_diameter,
                    ),
                    graphics::Color::new(0.0, 1.0, 0.0, 1.0),
                )
                .unwrap();
                graphics::draw(ctx, &image, graphics::DrawParam::default())?;
            }
        }
    }
    Ok(())
}
