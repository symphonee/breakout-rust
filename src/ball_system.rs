// Creates, updates and destroys the ball.

use crate::game_data::{GameMessage, GameState};
use ggez::nalgebra;

pub fn handle_ball_velocity_change(
    game_state: &mut GameState,
    new_velocity: nalgebra::Vector2<f32>,
) {
    game_state.ball_velocity = new_velocity;
}

pub fn tick(game_state: &mut GameState) -> Vec<GameMessage> {
    let mut game_messages: Vec<GameMessage> = Default::default();
    game_state.ball_position = nalgebra::Point2::new(
        game_state.ball_position.x + game_state.ball_velocity.x * game_state.dt.as_secs_f32(),
        game_state.ball_position.y + game_state.ball_velocity.y * game_state.dt.as_secs_f32(),
    );
    if game_state.ball_position.y > 600.0 {
        game_messages.push(GameMessage::EndGame);
    }
    if game_state.ball_position.y < 20.0 {
        // Hit ceiling
        game_messages.push(GameMessage::ChangeBallVelocity(nalgebra::Vector2::new(
            game_state.ball_velocity.x,
            -game_state.ball_velocity.y,
        )));
    }
    if game_state.ball_position.x < 20.0
        || game_state.ball_position.x > game_state.window_width - 20.0
    {
        // Hit wall
        game_messages.push(GameMessage::ChangeBallVelocity(nalgebra::Vector2::new(
            -game_state.ball_velocity.x,
            game_state.ball_velocity.y,
        )));
    }
    return game_messages;
}
