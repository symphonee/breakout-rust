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

    // Wall collision
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

    // Ball and Player collision
    let ball_player_dist =
        nalgebra::distance(&game_state.ball_position, &game_state.player_positon);
    let collision_distance = game_state.ball_radius + game_state.player_radius;
    if ball_player_dist <= collision_distance {
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
    }

    // Block collision
    let block_boundary_start_x =
        (game_state.window_width - game_state.block_width as f32 * game_state.block_diameter) * 0.5;
    let block_boundary_end_x =
        (game_state.window_width + game_state.block_width as f32 * game_state.block_diameter) * 0.5;
    let block_boundary_start_y = (game_state.window_height
        - game_state.block_height as f32 * game_state.block_diameter)
        * 0.5;
    let block_boundary_end_y = (game_state.window_height
        + game_state.block_height as f32 * game_state.block_diameter)
        * 0.5;
    // Is ball inside block grid?
    if game_state.ball_position.x + game_state.ball_radius > block_boundary_start_x
        && game_state.ball_position.x - game_state.ball_radius < block_boundary_end_x
        && game_state.ball_position.y + game_state.ball_radius > block_boundary_start_y
        && game_state.ball_position.y - game_state.ball_radius < block_boundary_end_y
    {
        // What cell is ball entering?
        let x_ball = game_state.ball_position.x - block_boundary_start_x;
        let y_ball = game_state.ball_position.y - block_boundary_start_y;
        let x_cell = (x_ball / game_state.block_diameter).floor() as u32;
        let y_cell = (y_ball / game_state.block_diameter).floor() as u32;
        if x_cell < game_state.block_width && y_cell < game_state.block_height {
            if game_state.blocks[(game_state.block_width * y_cell + x_cell) as usize] {
                game_state.blocks[(game_state.block_width * y_cell + x_cell) as usize] = false;
                // which side did we hit from? x or y?
                let block_pos_x =
                    block_boundary_start_x + (x_cell as f32 + 0.5) * game_state.block_diameter;
                let block_pos_y =
                    block_boundary_start_y + (y_cell as f32 + 0.5) * game_state.block_diameter;
                let x_diff = (game_state.ball_position.x - block_pos_x).abs();
                let y_diff = (game_state.ball_position.y - block_pos_y).abs();
                if x_diff > y_diff {
                    game_messages.push(GameMessage::ChangeBallVelocity(nalgebra::Vector2::new(
                        -game_state.ball_velocity.x,
                        game_state.ball_velocity.y,
                    )));
                } else {
                    game_messages.push(GameMessage::ChangeBallVelocity(nalgebra::Vector2::new(
                        game_state.ball_velocity.x,
                        -game_state.ball_velocity.y,
                    )));
                }
            }
        }
    }

    return game_messages;
}
