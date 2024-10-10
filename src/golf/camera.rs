use super::{ball::Ball, module::Module};
use crate::{
    eadk::{
        display::{SCREEN_HEIGHT, SCREEN_WIDTH},
        Color,
    },
    graphics::Buffer,
    math::Vec2,
};

const CAM_MOVE_SPEED: f32 = 0.04;
const CLEAR_COLOR: u16 = 0x07E0;

pub struct Camera<'a, 'b> {
    pub scene: &'a [&'b dyn Module],
    pub ball: Ball,
    pos: Vec2,
    unit_size: f32,
}

impl<'a, 'b> Camera<'a, 'b> {
    pub fn new(scene: &'a [&'b dyn Module], ball: Ball) -> Self {
        Camera {
            scene,
            pos: ball.pos,
            ball,
            unit_size: 2.,
        }
    }

    pub fn update(&mut self, unit_size_request: f32) {
        self.unit_size =
            self.unit_size + (unit_size_request.clamp(1., 8.) - self.unit_size) * CAM_MOVE_SPEED;
        self.pos = self.pos.lerp(
            self.ball.pos * self.unit_size
                - Vec2 {
                    x: (SCREEN_WIDTH / 2) as f32,
                    y: (SCREEN_HEIGHT / 2) as f32,
                },
            CAM_MOVE_SPEED,
        );
    }

    pub fn render(&self, buffer: &mut Buffer) {
        // clear screen
        buffer.clear(Color {
            rgb565: CLEAR_COLOR,
        });
        // draw modules
        for m in self.scene {
            m.render(buffer, self.pos, self.unit_size as i32);
        }
    }
}
