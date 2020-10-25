// Raw data is defined here.

use crate::input::InputState;
use ggez::*;

pub struct GameState {
    pub active: bool,
    pub queue: Vec<GameMessage>,
    pub dt: std::time::Duration,
    pub ball_position: nalgebra::Point2<f32>,
    pub ball_velocity: nalgebra::Vector2<f32>,
    pub player_positon: nalgebra::Point2<f32>,
    pub input_state: InputState,

    start_ball_position: nalgebra::Point2<f32>,
    start_ball_velocity: nalgebra::Vector2<f32>,
    stat_player_positon: nalgebra::Point2<f32>,
}

impl GameState {
    pub fn new(
        ball_position: nalgebra::Point2<f32>,
        ball_velocity: nalgebra::Vector2<f32>,
        player_positon: nalgebra::Point2<f32>,
    ) -> GameState {
        GameState {
            active: false,
            queue: Vec::new(),
            dt: std::time::Duration::new(0, 0),
            ball_position,
            ball_velocity,
            player_positon,
            input_state: InputState::default(),
            start_ball_position: ball_position,
            start_ball_velocity: ball_velocity,
            stat_player_positon: player_positon,
        }
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.queue.clear();
        self.dt = std::time::Duration::new(0, 0);
        self.ball_position = self.start_ball_position;
        self.ball_velocity = self.start_ball_velocity;
        self.player_positon = self.stat_player_positon;
    }
}

#[derive(Clone)]
pub enum GameMessage {
    ChangeBallVelocity(nalgebra::Vector2<f32>),
    StartGame,
    EndGame,
}
