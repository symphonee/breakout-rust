// Directs game messages to the interested systems.
// In the future, there could be a message broker/router dividing messages into different queues that the interested systems can go through.
// In that case, there would need to be a check to see that all queues have been cleared at the end of a frame.

use crate::{
    ball_system,
    game_data::{GameMessage, GameState},
};
use ggez::nalgebra;

pub fn tick(state: &mut GameState) {
    let mut message_index = 0;
    loop {
        if message_index >= state.queue.len() {
            break;
        }
        let game_message = state.queue[message_index].clone();
        match game_message {
            GameMessage::ChangeBallVelocity(velocity) => {
                ball_system::handle_ball_velocity_change(state, velocity);
            }
            GameMessage::MoveBallToPosition(position) => {
                ball_system::handle_ball_move(state, position);
            }
            GameMessage::StartGame => {
                state.active = true;
            }
            GameMessage::EndGame => {
                state.reset();
            }
            GameMessage::BallFlyFromRight => {
                state.ball_position.y = state.window_height * 0.5;
                state.ball_position.x = 100.0;
                state
                    .queue
                    .push(GameMessage::ChangeBallVelocity(nalgebra::Vector2::new(
                        200.0, 0.0,
                    )));
            }
            GameMessage::BallFlyFromLeft => {
                state.ball_position.y = state.window_height * 0.5;
                state.ball_position.x = state.window_width - 100.0;
                state
                    .queue
                    .push(GameMessage::ChangeBallVelocity(nalgebra::Vector2::new(
                        -200.0, 0.0,
                    )));
            }
            GameMessage::BallFlyFromTop => {
                state.ball_position.y = 100.0;
                state.ball_position.x = state.window_width * 0.5;
                state
                    .queue
                    .push(GameMessage::ChangeBallVelocity(nalgebra::Vector2::new(
                        0.0, 200.0,
                    )));
            }
            GameMessage::BallFlyFromBottom => {
                state.ball_position.y = state.window_height - 100.0;
                state.ball_position.x = state.window_width * 0.5;
                state
                    .queue
                    .push(GameMessage::ChangeBallVelocity(nalgebra::Vector2::new(
                        0.0, -200.0,
                    )));
            }
        }
        message_index += 1;
    }
    state.queue.clear();
}
