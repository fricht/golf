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
        let raw_ball_pos = &(&self.pos * unit_size as f32) - offset;
        let ball_pos = raw_ball_pos.to_int();
        let squared_unit_size = unit_size * unit_size;
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
            // // draw launch range (method 1 bc needs transparency)
            // // WARNING : might crash if circle clipping outside on screen
            // // does not work
            // let club_dist = CLUB_DISTANCE as i32;
            // let rect_length = 2 * club_dist;
            // let rect = Rect::new_square(
            //     (ball_pos.x - club_dist) as u16,
            //     (ball_pos.y - club_dist) as u16,
            //     rect_length as u16,
            // );
            // let mut background = display::get_rect(rect.clone());
            // for x in 0..(2 * club_dist) {
            //     for y in 0..(2 * club_dist) {
            //         let index = (x + y * rect_length) as usize;
            //         let mut col = background[index].separate_rgb();
            //         col.0 += 100;
            //         col.1 += 100;
            //         col.2 += 100;
            //         background[index] = Color::from_rgb(col.0, col.1, col.2);
            //     }
            // }
            // unsafe {
            //     display::eadk::push_rect(rect, background.as_ptr());
            // }
            // draw club
            let club_pos = (&raw_ball_pos + &(&self.launch_vec * CLUB_DISTANCE)).to_int();
            display::eadk::push_rect_uniform(
                Rect::screen_space_clipping(club_pos.x - 1, club_pos.y - 1, 3, 3),
                Color::MAGENTA,
            );
        }
    }
}
