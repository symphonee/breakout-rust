// Raw data is defined here.

use crate::input::InputState;
use ggez::*;

pub struct GameState {
    pub active: bool,
    pub queue: Vec<GameMessage>,
    pub dt: std::time::Duration,

    pub ball_position: nalgebra::Point2<f32>,
    pub ball_radius: f32,
    pub ball_velocity: nalgebra::Vector2<f32>,

    pub player_positon: nalgebra::Point2<f32>,
    pub player_radius: f32,
    pub input_state: InputState,
    pub walls: Vec<Wall>,

    pub window_width: f32,
    pub window_height: f32,

    start_ball_position: nalgebra::Point2<f32>,
    start_ball_velocity: nalgebra::Vector2<f32>,
    start_player_positon: nalgebra::Point2<f32>,
}

impl GameState {
    pub fn new(
        ball_position: nalgebra::Point2<f32>,
        ball_velocity: nalgebra::Vector2<f32>,
        ball_radius: f32,
        player_positon: nalgebra::Point2<f32>,
        player_radius: f32,
        window_width: f32,
        window_height: f32,
    ) -> GameState {
        let walls = vec![
            Wall {
                position: nalgebra::Point2::new(0.0, 0.0),
                width: window_width,
                height: 10.0,
            },
            Wall {
                position: nalgebra::Point2::new(0.0, 0.0),
                width: 10.0,
                height: window_height,
            },
            Wall {
                position: nalgebra::Point2::new(window_width - 10.0, 0.0),
                width: 10.0,
                height: window_height,
            },
        ];
        GameState {
            active: false,
            queue: Vec::new(),
            dt: std::time::Duration::new(0, 0),
            ball_position,
            ball_radius,
            ball_velocity,
            player_positon,
            player_radius,
            input_state: InputState::default(),
            start_ball_position: ball_position,
            start_ball_velocity: ball_velocity,
            start_player_positon: player_positon,
            walls,
            window_width,
            window_height,
        }
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.queue.clear();
        self.dt = std::time::Duration::new(0, 0);
        self.ball_position = self.start_ball_position;
        self.ball_velocity = self.start_ball_velocity;
        self.player_positon = self.start_player_positon;
    }
}

#[derive(Clone)]
pub enum GameMessage {
    ChangeBallVelocity(nalgebra::Vector2<f32>),
    MoveBallToPosition(nalgebra::Point2<f32>),
    StartGame,
    EndGame,
}

pub struct Wall {
    pub position: nalgebra::Point2<f32>,
    pub width: f32,
    pub height: f32,
}
