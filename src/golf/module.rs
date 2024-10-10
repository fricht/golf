use crate::{
    eadk::{Color, Rect},
    graphics::Buffer,
    math::{Vec2, Vec2i},
};

pub trait Module {
    fn render(&self, buffer: &mut Buffer, offset: Vec2, unit_size: i32);
}

// the size of 1 module tile : the ball have a radius of 1 (diameter of 2)
const TILE_SIZE: i32 = 16;

pub struct EmptyModule {
    size: Vec2i,
}

impl EmptyModule {
    pub fn new() -> Self {
        EmptyModule {
            size: Vec2i { x: 2, y: 2 },
        }
    }
}

impl Module for EmptyModule {
    fn render(&self, buffer: &mut Buffer, offset: Vec2, unit_size: i32) {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                buffer.push_rect_uniform(
                    Rect::screen_space_culling(
                        x * unit_size * TILE_SIZE - offset.x as i32,
                        y * unit_size * TILE_SIZE - offset.y as i32,
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
