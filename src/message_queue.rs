// Directs game messages to the interested systems.
// In the future, there could be a message broker/router dividing messages into different queues that the interested systems can go through.
// In that case, there would need to be a check to see that all queues have been cleared at the end of a frame.

use crate::{
    ball_system,
    game_data::{GameMessage, State},
};
use ggez::nalgebra;

pub fn add_change_ball_velocity_message(
    queue: &mut Vec<GameMessage>,
    new_velocity: nalgebra::Vector2<f32>,
) {
    queue.push(GameMessage::ChangeBallVelocity(new_velocity));
}

pub fn tick(state: &mut State) {
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
            GameMessage::StartGame => {
                state.active = true;
                add_change_ball_velocity_message(
                    &mut state.queue,
                    nalgebra::Vector2::new(0.0, 100.0),
                );
            }
            GameMessage::EndGame => {
                state.reset();
            }
        }
        message_index += 1;
    }
    state.queue.clear();
}
