//extern crate alloc;

use super::{ball::Ball, module::Module};
use crate::{
    eadk::{display::SCREEN_HEIGHT, Color},
    graphics::Buffer,
    math::Vec2,
};
//use alloc::vec::Vec;

const CAM_MOVE_SPEED: f32 = 0.1;
const CLEAR_COLOR: u16 = 0x07E0;

pub struct Camera<'a> {
    pub scene: &'a [Module],
    pub ball: Ball,
    pos: Vec2,
    zoom_level: f32,
}

impl<'a> Camera<'a> {
    pub fn new(scene: &'a [Module], ball: Ball) -> Self {
        Camera {
            scene,
            pos: ball.pos,
            ball,
            zoom_level: 1.,
        }
    }

    pub fn update(&mut self, zoom_request: f32) {
        self.pos = self.pos.lerp(self.ball.pos, CAM_MOVE_SPEED);
        self.zoom_level = self.zoom_level + (zoom_request - self.zoom_level) * CAM_MOVE_SPEED;
    }

    pub fn render(&self, buffer: &mut Buffer) {
        // clear screen
        buffer.clear(Color {
            rgb565: CLEAR_COLOR,
        });
        // draw modules
        for m in self.scene {
            m.render(buffer, Vec2::default(), 16);
        }
    }
}
