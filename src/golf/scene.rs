use super::{
    ball::{Ball, BALL_STOP_THRESHOLD},
    module::{BallInteraction, Module},
};
use crate::{
    eadk::{
        display::{SCREEN_HEIGHT, SCREEN_WIDTH},
        input::{Key, KeyboardState},
        Color,
    },
    graphics::Buffer,
    math::Vec2,
};

const CAM_MOVE_SPEED: f32 = 0.04;
const CLEAR_COLOR: u16 = 0x07E0;

pub enum GameState {
    Moving,
    Idle,
    Dead,
    Won,
}

pub struct Scene<'a, 'b> {
    pub modules: &'a mut [&'b mut dyn Module],
    pub game_state: GameState,
    cam_pos: Vec2,
    pub spawn_pos: Vec2,
    pub ball: Ball,
    unit_size: f32, // px / unit
    pub attempts: u8,
}

impl<'a, 'b> Scene<'a, 'b> {
    pub fn new(scene: &'a mut [&'b mut dyn Module], spawn_pos: Vec2, unit_size: f32) -> Self {
        let unit_size = unit_size.clamp(1., 8.);
        let ball = Ball::new(spawn_pos);
        Scene {
            modules: scene,
            game_state: GameState::Idle,
            cam_pos: ball.pos * unit_size
                - Vec2 {
                    x: (SCREEN_WIDTH / 2) as f32,
                    y: (SCREEN_HEIGHT / 2) as f32,
                },
            spawn_pos,
            ball,
            unit_size,
            attempts: 0,
        }
    }

    pub fn update(&mut self) {
        let unit_size_request: f32 = 3.;
        // update the scene
        if let GameState::Idle | GameState::Moving = self.game_state {
            // update ball
            self.ball.update();
            // update modules
            let mut in_region = false;
            for m in self.modules.iter_mut() {
                match m.update(&mut self.ball) {
                    BallInteraction::Dead => {
                        in_region = true;
                        self.game_state = GameState::Dead;
                        break;
                    }
                    BallInteraction::Win => {
                        in_region = true;
                        self.game_state = GameState::Won;
                        break;
                    }
                    BallInteraction::None => {
                        in_region = true;
                    }
                    _ => (),
                };
            }
            if !in_region {
                self.game_state = GameState::Dead;
            }
        }
        // move the camera
        self.unit_size =
            self.unit_size + (unit_size_request.clamp(1., 8.) - self.unit_size) * CAM_MOVE_SPEED;
        self.cam_pos = self.cam_pos.lerp(
            self.ball.pos * self.unit_size
                - Vec2 {
                    x: (SCREEN_WIDTH / 2) as f32,
                    y: (SCREEN_HEIGHT / 2) as f32,
                },
            CAM_MOVE_SPEED,
        );
        // handle game state
        let keys = KeyboardState::scan();
        match self.game_state {
            GameState::Moving => {
                if self.ball.velocity.norm() < BALL_STOP_THRESHOLD {
                    self.ball.velocity = Vec2 { x: 0., y: 0. };
                    self.game_state = GameState::Idle;
                }
            }
            GameState::Idle => {
                self.ball.move_launch(
                    Vec2 {
                        x: (keys.key_down(Key::Right) as i8 - keys.key_down(Key::Left) as i8)
                            as f32,
                        y: (keys.key_down(Key::Down) as i8 - keys.key_down(Key::Up) as i8) as f32,
                    } * 0.06,
                );
                if keys.key_down(Key::Ok) {
                    self.attempts += 1;
                    self.ball.launch();
                    self.game_state = GameState::Moving;
                }
            }
            GameState::Dead => {
                self.attempts += 1;
                self.ball.reset(self.spawn_pos);
                self.game_state = GameState::Idle;
            }
            GameState::Won => (),
        };
    }

    pub fn render(&self, buffer: &mut Buffer) {
        // clear screen
        buffer.clear(Color {
            rgb565: CLEAR_COLOR,
        });
        // draw modules
        for m in self.modules.iter() {
            m.render(buffer, self.cam_pos, self.unit_size as i32);
        }
        // draw ball
        self.ball.render(
            buffer,
            self.cam_pos,
            self.unit_size,
            if let GameState::Idle = self.game_state {
                true
            } else {
                false
            },
        );
    }
}
