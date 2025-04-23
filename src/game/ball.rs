use crate::utils::vec::Vec2;
use libnw::display::{self, Color, Rect, SCREEN_WIDTH};

const BALL_LAUNCH_SPEED: f32 = -1.2;
const CLUB_DISTANCE: f32 = 60.;

pub struct Ball {
    pub pos: Vec2<f32>,
    pub height: f32,
    pub velocity: Vec2<f32>,
    pub launch_vec: Vec2<f32>,
}

impl Ball {
    pub fn new(pos: Vec2<f32>) -> Self {
        Ball {
            pos,
            height: 0.,
            velocity: Vec2 { x: 0., y: 0. },
            launch_vec: Vec2 { x: 0., y: 0. },
        }
    }

    pub fn reset(&mut self, pos: Vec2<f32>) {
        self.pos = pos;
        self.height = 0.;
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

    pub fn render(&self, offset: &Vec2<f32>, unit_size: i32, render_launch: bool) {
        // How to render a circle, 2 methods
        // 1 - Pull the rect (AABB) where it will be drawn
        // then re-color some part and draw it again
        // 2 - draw pixel by pixel (can be optimized by
        // drawing rectangles)

        // using method 2 bc easyer & faster to implement
        // ball radius : 1 unit
        let ball_pos = &(&(&self.pos * unit_size as f32) - offset).to_int()
            - &Vec2 {
                x: unit_size / 2,
                y: unit_size / 2,
            };
        for x in (ball_pos.x).max(0)..(ball_pos.x + unit_size).min(SCREEN_WIDTH as i32) {
            for y in (ball_pos.y).max(0)..(ball_pos.y + unit_size).min(SCREEN_WIDTH as i32) {
                display::set_pixel(x as u16, y as u16, Color::WHITE);
            }
        }
        if render_launch && self.launch_vec.norm_sqd() > 0.01 {
            let club_pos = (&(&(&self.pos * unit_size as f32) - offset)
                + &(&self.launch_vec * CLUB_DISTANCE))
                .to_int();
            display::eadk::push_rect_uniform(
                Rect::screen_space_clipping(club_pos.x - 1, club_pos.y - 1, 3, 3),
                Color::BLUE,
            );
        }
    }
}
