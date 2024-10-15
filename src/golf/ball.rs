use super::module::TILE_SIZE;
use crate::{eadk::Color, graphics::Buffer, math::Vec2};

pub const BALL_MASS: f32 = 10.;
pub const BALL_STOP_THRESHOLD: f32 = 0.1;
const BALL_LAUNCH_SPEED: f32 = -1.2;

pub struct Ball {
    pub pos: Vec2,
    pub height: f32,
    pub velocity: Vec2,
    pub launch_vec: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Ball {
            pos,
            height: 0.,
            velocity: Vec2 { x: 0., y: 0. },
            launch_vec: Vec2 { x: 0., y: 0. },
        }
    }

    pub fn reset(&mut self, pos: Vec2) {
        self.pos = pos;
        self.height = 0.;
        self.velocity = Vec2 { x: 0., y: 0. };
        self.launch_vec = Vec2 { x: 0., y: 0. };
    }

    pub fn update(&mut self) {
        self.pos = self.pos + self.velocity;
    }

    pub fn launch(&mut self) {
        self.velocity = self.launch_vec * BALL_LAUNCH_SPEED;
        self.launch_vec = Vec2 { x: 0., y: 0. };
    }

    pub fn move_launch(&mut self, movment: Vec2) {
        self.launch_vec = self.launch_vec + movment;
        if self.launch_vec.norm() > 1. {
            self.launch_vec.normalize();
        }
    }

    pub fn render(&self, buffer: &mut Buffer, offset: Vec2, unit_size: f32, render_launch: bool) {
        let ball_pos = self.pos * unit_size - offset;
        buffer.circle(ball_pos, unit_size, Color { rgb565: 0xFFFF });
        if render_launch {
            buffer.line(
                ball_pos,
                ball_pos + self.launch_vec * unit_size * TILE_SIZE as f32,
                Color { rgb565: 0 },
            );
        }
    }
}
