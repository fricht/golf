use crate::{
    eadk::{Color, Rect},
    graphics::Buffer,
    math::{Vec2, Vec2i},
};

pub struct Module {
    size: Vec2i,
}

impl Module {
    pub fn new() -> Self {
        Module {
            size: Vec2i { x: 2, y: 2 },
        }
    }

    pub fn render(&self, buffer: &mut Buffer, offset: Vec2, tile_size: i32) {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                buffer.push_rect_uniform(
                    Rect::screen_space_culling(
                        offset.x as i32 + x * tile_size,
                        offset.y as i32 + y * tile_size,
                        tile_size,
                        tile_size,
                    ),
                    Color {
                        rgb565: if (x + y) % 2 == 0 { 0x0640 } else { 0x0580 },
                    },
                );
            }
        }
    }
}
