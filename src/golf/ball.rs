use crate::{eadk::Color, graphics::Buffer, math::Vec2};

const BALL_MASS: f32 = 10.;

pub struct Ball {
    pub pos: Vec2,
    pub height: f32,
    pub velocity: Vec2,
}

impl Ball {
    pub fn update(&mut self) {
        self.pos = self.pos + self.velocity;
    }

    pub fn render(&self, buffer: &mut Buffer, offset: Vec2, unit_size: f32) {
        buffer.circle(
            self.pos * unit_size - offset,
            unit_size,
            Color { rgb565: 0xFFFF },
        );
    }
}
