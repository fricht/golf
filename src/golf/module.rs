use crate::{
    eadk::{Color, Rect},
    graphics::Buffer,
    math::{Vec2, Vec2i},
};

use super::ball::Ball;

pub enum BallInteraction {
    OutOfRegion,
    None,
    Dead,
    Win,
}

pub trait Module {
    fn update(&mut self, ball: &mut Ball) -> BallInteraction;
    fn render(&self, buffer: &mut Buffer, offset: Vec2, unit_size: i32);
}

// the size of 1 module tile : the ball have a radius of 1 (diameter of 2)
pub const TILE_SIZE: i32 = 8;

pub struct EmptyModule {
    pub offset: Vec2i,
    pub size: Vec2i,
}

impl EmptyModule {
    pub fn new(offset: Vec2i) -> Self {
        EmptyModule {
            offset,
            size: Vec2i { x: 2, y: 2 },
        }
    }
}

impl Module for EmptyModule {
    // AABB collision check
    fn update(&mut self, ball: &mut Ball) -> BallInteraction {
        if ball.pos.x + 1. <= self.offset.x as f32
            || ball.pos.x - 1. >= (self.offset.x + self.size.x * TILE_SIZE) as f32
            || ball.pos.y + 1. <= self.offset.y as f32
            || ball.pos.y - 1. >= (self.offset.y + self.size.y * TILE_SIZE) as f32
        {
            return BallInteraction::OutOfRegion;
        }
        // apply drag
        ball.velocity = ball.velocity * 0.98;
        BallInteraction::None
    }

    fn render(&self, buffer: &mut Buffer, offset: Vec2, unit_size: i32) {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                buffer.push_rect_uniform(
                    Rect::screen_space_culling(
                        x * unit_size * TILE_SIZE - offset.x as i32
                            + self.offset.x as i32 * unit_size,
                        y * unit_size * TILE_SIZE - offset.y as i32
                            + self.offset.y as i32 * unit_size,
                        unit_size * TILE_SIZE,
                        unit_size * TILE_SIZE,
                    ),
                    Color {
                        rgb565: if (x + y) % 2 == 0 { 0x0640 } else { 0x0580 },
                    },
                );
            }
        }
    }
}
