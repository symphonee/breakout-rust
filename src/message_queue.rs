// Directs game messages to the interested systems.

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
        }
        message_index += 1;
    }
    state.queue.clear();
}
