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
    pub player_speed: f32,

    pub input_state: InputState,

    pub walls: Vec<Wall>,

    pub blocks: Vec<bool>, // is block active
    pub block_width: u32,
    pub block_height: u32,
    pub block_diameter: f32,

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
        player_speed: f32,
        block_width: u32,
        block_height: u32,
        block_diameter: f32,
        window_width: f32,
        window_height: f32,
        wall_width: f32,
    ) -> GameState {
        let walls = vec![
            Wall {
                position: nalgebra::Point2::new(0.0, 0.0),
                width: window_width,
                height: wall_width,
            },
            Wall {
                position: nalgebra::Point2::new(0.0, 0.0),
                width: wall_width,
                height: window_height,
            },
            Wall {
                position: nalgebra::Point2::new(window_width - 10.0, 0.0),
                width: wall_width,
                height: window_height,
            },
        ];
        let mut blocks = Vec::default();
        for _ in 0..(block_width * block_height) {
            blocks.push(true);
        }
        GameState {
            active: false,
            queue: Vec::new(),
            dt: std::time::Duration::new(0, 0),
            ball_position,
            ball_radius,
            ball_velocity,
            player_positon,
            player_radius,
            player_speed,
            input_state: InputState::default(),
            walls,
            blocks,
            block_width,
            block_height,
            block_diameter,
            window_width,
            window_height,
            start_ball_position: ball_position,
            start_ball_velocity: ball_velocity,
            start_player_positon: player_positon,
        }
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.queue.clear();
        self.dt = std::time::Duration::new(0, 0);
        self.ball_position = self.start_ball_position;
        self.ball_velocity = self.start_ball_velocity;
        self.player_positon = self.start_player_positon;
        for i in 0..self.blocks.len() {
            self.blocks[i] = true;
        }
    }
}

#[derive(Clone)]
pub enum GameMessage {
    ChangeBallVelocity(nalgebra::Vector2<f32>),
    MoveBallToPosition(nalgebra::Point2<f32>),
    StartGame,
    EndGame,
    BallFlyFromRight,
    BallFlyFromLeft,
    BallFlyFromTop,
    BallFlyFromBottom,
}

pub struct Wall {
    pub position: nalgebra::Point2<f32>,
    pub width: f32,
    pub height: f32,
}
