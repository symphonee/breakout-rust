// Creates, updates and destroys the ball.

use crate::game_data;
use ggez::nalgebra;

pub fn handle_ball_velocity_change(
    game_state: &mut game_data::State,
    new_velocity: nalgebra::Vector2<f32>,
) {
    game_state.ball_velocity = new_velocity;
}

pub fn tick(game_state: &mut game_data::State) {
    game_state.ball_position = nalgebra::Point2::new(
        game_state.ball_position.x + game_state.ball_velocity.x * game_state.dt.as_secs_f32(),
        game_state.ball_position.y + game_state.ball_velocity.y * game_state.dt.as_secs_f32(),
    );
}
