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
        nalgebra::Point2::new(400.0, 0.0),
        nalgebra::Vector2::new(0.0, 0.0),
        nalgebra::Point2::new(200.0, 200.0),
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
                delta_move.x += -1.0;
            }

            if self.input_state.is_key_pressed(Key::D)
                || self.input_state.is_key_pressed(Key::Right)
            {
                delta_move.x += 1.0
            }

            let speed = 100.0;
            self.player_positon += delta_move * self.dt.as_secs_f32() * speed;

            let mut game_messages = ball_system::tick(self);
            self.queue.append(&mut game_messages);
        }
        message_queue::tick(self);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Draw text first
        draw_text(ctx, "Bjandra Breakout", 0.0, 100.0)?;

        // Then reset
        graphics::set_transform(ctx, graphics::DrawParam::default().to_matrix());
        graphics::apply_transformations(ctx)?;

        // Then draw other drawables
        draw_ball(ctx, &self.ball_position)?;
        draw_player(ctx, &self.player_positon)?;

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

fn draw_ball(ctx: &mut Context, ball_position: &nalgebra::Point2<f32>) -> GameResult<()> {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        *ball_position,
        10.0,
        2.0,
        graphics::WHITE,
    )
    .unwrap();
    graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
    Ok(())
}

fn draw_player(ctx: &mut Context, player_positon: &nalgebra::Point2<f32>) -> GameResult<()> {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        *player_positon,
        30.0,
        2.0,
        graphics::WHITE,
    )
    .unwrap();
    graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
    Ok(())
}
