use crate::utils::vec::Vec2;
use libnw::display::{self, Color, Rect, SCREEN_WIDTH};

use super::game::CLEAR_COLOR;

const BALL_LAUNCH_SPEED: f32 = -1.2;
const CLUB_DISTANCE: f32 = 60.;

pub struct Ball {
    pub pos: Vec2<f32>,
    pub velocity: Vec2<f32>,
    pub launch_vec: Vec2<f32>,
    old_pos: Rect,
    old_launch_indicator: Option<(Vec2<i32>, Vec2<i32>)>,
}

impl Ball {
    pub fn new(pos: Vec2<f32>) -> Self {
        Ball {
            pos,
            velocity: Vec2 { x: 0., y: 0. },
            launch_vec: Vec2 { x: 0., y: 0. },
            old_pos: Rect::new(0, 0, 0, 0),
            old_launch_indicator: None,
        }
    }

    pub fn reset(&mut self, pos: Vec2<f32>) {
        self.pos = pos;
        self.velocity = Vec2 { x: 0., y: 0. };
        self.launch_vec = Vec2 { x: 0., y: 0. };
    }

    pub fn update(&mut self, drag: f32) {
        self.velocity.scale(drag);
        self.pos = &self.pos + &self.velocity;
    }

    pub fn launch(&mut self) {
        self.velocity = &self.launch_vec * BALL_LAUNCH_SPEED;
        self.launch_vec = Vec2 { x: 0., y: 0. };
    }

    pub fn move_launch(&mut self, movment: Vec2<f32>) {
        self.launch_vec = &self.launch_vec + &movment;
        if self.launch_vec.norm() > 1. {
            self.launch_vec.normalize();
        }
    }

    pub fn render(&mut self, offset: &Vec2<f32>, unit_size: i32, render_launch: bool) {
        // How to render a circle, 2 methods
        // 1 - Pull the rect (AABB) where it will be drawn
        // then re-color some part and draw it again
        // 2 - draw pixel by pixel (can be optimized by
        // drawing rectangles)

        // using method 2 bc easyer & faster to implement
        // ball radius : 1 unit
        let raw_ball_pos = &(&self.pos * unit_size as f32) - offset;
        let ball_pos = raw_ball_pos.to_int();
        let squared_unit_size = unit_size * unit_size;
        self.old_pos = Rect::screen_space_clipping(
            ball_pos.x - unit_size,
            ball_pos.y - unit_size,
            2 * unit_size as u16 + 1,
            2 * unit_size as u16 + 1,
        );
        for x in
            (ball_pos.x - unit_size).max(0)..(ball_pos.x + unit_size + 1).min(SCREEN_WIDTH as i32)
        {
            for y in (ball_pos.y - unit_size).max(0)
                ..(ball_pos.y + unit_size + 1).min(SCREEN_WIDTH as i32)
            {
                if ((x - ball_pos.x).pow(2) + (y - ball_pos.y).pow(2)) <= squared_unit_size {
                    display::set_pixel(x as u16, y as u16, Color::WHITE);
                }
            }
        }
        if render_launch && self.launch_vec.norm_sqd() > 0.01 {
            // draw launch range
            let ref_club_pos =
                (&raw_ball_pos + &(&self.launch_vec.normalized() * CLUB_DISTANCE)).to_int();
            display::eadk::push_rect_uniform(
                Rect::screen_space_clipping(ref_club_pos.x - 1, ref_club_pos.y - 1, 3, 3),
                Color::new(0xb5b6), // gray
            );
            // draw club
            let club_pos = (&raw_ball_pos + &(&self.launch_vec * CLUB_DISTANCE)).to_int();
            display::eadk::push_rect_uniform(
                Rect::screen_space_clipping(club_pos.x - 1, club_pos.y - 1, 3, 3),
                Color::MAGENTA,
            );
            self.old_launch_indicator = Some((ref_club_pos, club_pos));
        } else {
            self.old_launch_indicator = None;
        }
    }

    pub fn erase(&self) {
        display::eadk::push_rect_uniform(self.old_pos, CLEAR_COLOR);
        if let Some((ref_club_pos, club_pos)) = self.old_launch_indicator {
            display::eadk::push_rect_uniform(
                Rect::screen_space_clipping(ref_club_pos.x - 1, ref_club_pos.y - 1, 3, 3),
                CLEAR_COLOR, // gray
            );
            display::eadk::push_rect_uniform(
                Rect::screen_space_clipping(club_pos.x - 1, club_pos.y - 1, 3, 3),
                CLEAR_COLOR,
            );
        }
    }
}
