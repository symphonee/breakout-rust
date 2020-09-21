// Raw data is defined here.

use crate::input::InputState;
use ggez::*;

pub struct State {
    pub queue: Vec<GameMessage>,
    pub dt: std::time::Duration,
    pub ball_position: nalgebra::Point2<f32>,
    pub ball_velocity: nalgebra::Vector2<f32>,
    pub player_positon: nalgebra::Point2<f32>,
    pub input_state: InputState,
}

impl State {
    pub fn new(
        ball_position: nalgebra::Point2<f32>,
        ball_velocity: nalgebra::Vector2<f32>,
        player_positon: nalgebra::Point2<f32>,
    ) -> State {
        State {
            queue: Vec::new(),
            dt: std::time::Duration::new(0, 0),
            ball_position,
            ball_velocity,
            player_positon,
            input_state: InputState::default(),
        }
    }
}

#[derive(Clone)]
pub enum GameMessage {
    ChangeBallVelocity(nalgebra::Vector2<f32>),
}
