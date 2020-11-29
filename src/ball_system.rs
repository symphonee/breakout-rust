// Creates, updates and destroys the ball.

use crate::game_data::{GameMessage, GameState};
use ggez::nalgebra;

pub fn handle_ball_velocity_change(
    game_state: &mut GameState,
    new_velocity: nalgebra::Vector2<f32>,
) {
    game_state.ball_velocity = new_velocity;
}

pub fn handle_ball_move(game_state: &mut GameState, new_position: nalgebra::Point2<f32>) {
    game_state.ball_position = new_position;
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

    let ball_player_dist =
        nalgebra::distance(&game_state.ball_position, &game_state.player_positon);
    let collision_distance = game_state.ball_radius + game_state.player_radius;

    if ball_player_dist <= collision_distance {
        // Ball and Player collision
        // find normal
        let normal = nalgebra::Matrix::normalize(&nalgebra::Vector2::new(
            game_state.ball_position.x - game_state.player_positon.x,
            game_state.ball_position.y - game_state.player_positon.y,
        ));

        // find angle to normal
        let angle = (-game_state.ball_velocity).angle(&normal);

        // find new direction with angle on other side of normal
        let rot = nalgebra::Rotation2::new(angle);
        let magnitude = nalgebra::Matrix::magnitude(&game_state.ball_velocity);
        let new_dir = rot * normal * magnitude;

        // move out ball
        let new_pos: nalgebra::Point2<f32> =
            game_state.player_positon + normal * collision_distance;
        game_messages.push(GameMessage::MoveBallToPosition(new_pos));

        game_messages.push(GameMessage::ChangeBallVelocity(new_dir));

        /*println!(
            "ball {} player {} normal {} angle {} in dir {} out dir {}.",
            game_state.ball_position,
            game_state.player_positon,
            normal,
            angle,
            game_state.ball_velocity,
            new_dir
        );*/
    }

    return game_messages;
}
