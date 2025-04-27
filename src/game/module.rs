use libnw::display::{self, Color, Rect};

use super::ball::Ball;
use crate::utils::vec::Vec2;

pub enum BallInteraction {
    Out,
    /// ball is inside (drag, dist_to_center)
    ///
    /// I can't directly apply drag, bc if ball is on edge,
    /// drag will be applied multiple times (for each module it touches)
    In(f32),
    Dead,
    Win,
}

pub trait Module {
    fn update(&mut self, ball: &mut Ball) -> BallInteraction;
    fn render(&self, cam_pos: &Vec2<f32>, unit_size: i32, is_first_half: bool);
}

// the size of 1 module tile : the ball have a radius of 1 (diameter of 2)
pub const TILE_SIZE: u16 = 4;

pub struct EmptyModule {
    pos: Vec2<i32>,
    size: Vec2<i32>,
}

impl EmptyModule {
    pub fn new_rect(offset: Vec2<i32>, (size_x, size_y): (i32, i32)) -> Self {
        EmptyModule {
            pos: offset,
            size: Vec2 {
                x: size_x,
                y: size_y,
            },
        }
    }
}

impl Module for EmptyModule {
    // AABB collision check
    fn update(&mut self, ball: &mut Ball) -> BallInteraction {
        if ball.pos.x + 1. < self.pos.x as f32
            || ball.pos.x - 1. > (self.pos.x + self.size.x * TILE_SIZE as i32) as f32
            || ball.pos.y + 1. < self.pos.y as f32
            || ball.pos.y - 1. > (self.pos.y + self.size.y * TILE_SIZE as i32) as f32
        {
            return BallInteraction::Out;
        }
        BallInteraction::In(0.98)
    }

    fn render(&self, offset: &Vec2<f32>, unit_size: i32, is_first_half: bool) {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                display::eadk::push_rect_uniform(
                    Rect::half_screen_space_clipping(
                        x * unit_size * TILE_SIZE as i32 - offset.x as i32
                            + self.pos.x as i32 * unit_size,
                        y * unit_size * TILE_SIZE as i32 - offset.y as i32
                            + self.pos.y as i32 * unit_size,
                        unit_size as u16 * TILE_SIZE,
                        unit_size as u16 * TILE_SIZE,
                        is_first_half,
                    ),
                    Color::new(if (x + y) % 2 == 0 { 0x0640 } else { 0x0580 }),
                );
            }
        }
    }
}

/// 4 × 4 Module (× TILE_SIZE (4))
pub struct SquareEndModule {
    pos: Vec2<i32>,
}

impl SquareEndModule {
    pub fn new_4x4(offset: Vec2<i32>) -> Self {
        SquareEndModule { pos: offset }
    }
}

impl Module for SquareEndModule {
    // AABB collision check
    fn update(&mut self, ball: &mut Ball) -> BallInteraction {
        if ball.pos.x + 1. < self.pos.x as f32
            || ball.pos.x - 1. > (self.pos.x + 4 * TILE_SIZE as i32) as f32
            || ball.pos.y + 1. < self.pos.y as f32
            || ball.pos.y - 1. > (self.pos.y + 4 * TILE_SIZE as i32) as f32
        {
            return BallInteraction::Out;
        }
        if ball.pos.x + 1. < (self.pos.x + 2 * TILE_SIZE as i32 - 1) as f32
            || ball.pos.x - 1. > (self.pos.x + 2 * TILE_SIZE as i32 + 1) as f32
            || ball.pos.y + 1. < (self.pos.y + 2 * TILE_SIZE as i32 - 1) as f32
            || ball.pos.y - 1. > (self.pos.y + 2 * TILE_SIZE as i32 + 1) as f32
        {
            BallInteraction::In(0.98)
        } else {
            BallInteraction::Win
        }
    }

    fn render(&self, offset: &Vec2<f32>, unit_size: i32, is_first_half: bool) {
        for x in 0..4 {
            for y in 0..4 {
                display::eadk::push_rect_uniform(
                    Rect::half_screen_space_clipping(
                        x * unit_size * TILE_SIZE as i32 - offset.x as i32
                            + self.pos.x as i32 * unit_size,
                        y * unit_size * TILE_SIZE as i32 - offset.y as i32
                            + self.pos.y as i32 * unit_size,
                        unit_size as u16 * TILE_SIZE,
                        unit_size as u16 * TILE_SIZE,
                        is_first_half,
                    ),
                    Color::new(if (x + y) % 2 == 0 { 0x0640 } else { 0x0580 }),
                );
            }
        }
        display::eadk::push_rect_uniform(
            Rect::screen_space_clipping(
                2 * unit_size * TILE_SIZE as i32 - offset.x as i32 + self.pos.x as i32 * unit_size
                    - unit_size,
                2 * unit_size * TILE_SIZE as i32 - offset.y as i32 + self.pos.y as i32 * unit_size
                    - unit_size,
                unit_size as u16 * 2,
                unit_size as u16 * 2,
            ),
            Color::BLACK,
        );
    }
}
