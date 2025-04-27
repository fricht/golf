extern crate alloc;

use super::{
    ball::Ball,
    module::{BallInteraction, Module},
    popup::PopupState,
};
use crate::{PopMessage, game::game_over::GameOverState, utils::vec::Vec2};
use alloc::{boxed::Box, format, string::ToString, vec::Vec};
use graphmgr::{StackAction, State};
use libnw::{
    display::{self, Color, Rect, SCREEN_HEIGHT, SCREEN_WIDTH},
    keyboard::RawKey,
};

pub const CLEAR_COLOR: Color = Color::GREEN;

pub struct GameState {
    is_moving: bool,
    modules: Vec<Box<dyn Module>>,
    cam_pos: Vec2<f32>,
    spawn_pos: Vec2<f32>,
    ball: Ball,
    attempts: u8,
    unit_size: u8,
}

const CAM_OFFSET: Vec2<f32> = Vec2 {
    x: (SCREEN_WIDTH / 2) as f32,
    y: (SCREEN_HEIGHT / 2) as f32,
};
const CAM_MOVE_SPEED: f32 = 0.04;

impl GameState {
    pub fn new(modules: Vec<Box<dyn Module>>, ball: Ball) -> Self {
        Self {
            is_moving: false,
            modules,
            cam_pos: &ball.pos - &CAM_OFFSET,
            spawn_pos: ball.pos.clone(),
            ball,
            attempts: 0,
            unit_size: 3,
        }
    }
}

impl State<PopMessage> for GameState {
    fn create(&mut self) -> StackAction<PopMessage> {
        display::eadk::push_rect_uniform(Rect::SCREEN, CLEAR_COLOR);
        StackAction::Nop
    }

    fn resume(&mut self, pop_message: PopMessage) -> StackAction<PopMessage> {
        display::eadk::push_rect_uniform(Rect::SCREEN, CLEAR_COLOR);
        if let PopMessage::OkBackPopupIsOk(true) = pop_message {
            StackAction::Pop(PopMessage::None)
        } else {
            StackAction::Nop
        }
    }

    fn update(
        &mut self,
        keyboard_state: &libnw::keyboard::KeyboardTimedState,
    ) -> StackAction<PopMessage> {
        // change zoom
        let delta_zoom = (keyboard_state.is_key_just_pressed(RawKey::Plus) as i8)
            - (keyboard_state.is_key_just_pressed(RawKey::Minus) as i8);
        self.unit_size = (self.unit_size as i8 + delta_zoom).clamp(1, 8) as u8;

        let mut is_outside = true;
        let mut drags = Vec::<f32>::new();
        'modules_update: for m in self.modules.iter_mut() {
            match m.update(&mut self.ball) {
                BallInteraction::Win => {
                    return StackAction::Replace(Box::new(GameOverState(self.attempts)));
                }
                BallInteraction::Dead => {
                    // if ball is dead, act as if it was outside
                    is_outside = true;
                    break 'modules_update;
                }
                BallInteraction::In(drag) => {
                    drags.push(drag);
                    is_outside = false;
                }
                _ => (),
            }
        }
        if is_outside {
            self.attempts += 1;
            self.ball.reset(self.spawn_pos);
            self.is_moving = false;
        } else {
            // apply mean drag
            let mut drag = 0f32;
            for d in drags.iter() {
                drag += d;
            }
            drag /= drags.len() as f32;
            self.ball.update(drag);
        }

        // move cam
        self.cam_pos.lerp_to(
            &(&(&self.ball.pos * self.unit_size as f32) - &CAM_OFFSET),
            CAM_MOVE_SPEED,
        );

        match self.is_moving {
            true => {
                if self.ball.velocity.norm_sqd() < 0.01 {
                    self.ball.velocity = Vec2::<f32> { x: 0., y: 0. };
                    self.is_moving = false;
                }
            }
            false => {
                let move_speed = if keyboard_state.is_key_pressed(RawKey::Shift) {
                    0.015
                } else {
                    0.03
                };
                self.ball.move_launch(Vec2::<f32> {
                    x: ((keyboard_state.is_key_pressed(RawKey::Right) as i8)
                        - (keyboard_state.is_key_pressed(RawKey::Left) as i8))
                        as f32
                        * move_speed,
                    y: ((keyboard_state.is_key_pressed(RawKey::Down) as i8)
                        - (keyboard_state.is_key_pressed(RawKey::Up) as i8))
                        as f32
                        * move_speed,
                });
                if (keyboard_state.is_key_just_pressed(RawKey::Ok)
                    || keyboard_state.is_key_just_pressed(RawKey::Exe))
                    && self.ball.launch_vec.norm_sqd() > 0.01
                {
                    self.attempts += 1;
                    self.ball.launch();
                    self.is_moving = true;
                }
            }
        }

        if keyboard_state.is_key_just_pressed(RawKey::Back) {
            StackAction::Push(Box::new(PopupState("Leave ???".to_string())))
        } else {
            StackAction::Nop
        }
    }

    fn render(&mut self) {
        display::eadk::wait_for_vblank();
        // erase only (old) necessary stuff (to avoid re-drawing useless stuff (bg))
        // erase score
        const SCORE_RECT: Rect = Rect {
            x: 0,
            y: 0,
            width: 12 * display::CHAR_WIDTH,
            height: display::CHAR_HEIGHT,
        };
        display::eadk::push_rect_uniform(SCORE_RECT, CLEAR_COLOR);
        // erase ball
        self.ball.erase();
        // erase modules
        for m in self.modules.iter() {
            m.erase();
        }
        // draw modules
        for m in self.modules.iter_mut() {
            m.render(&self.cam_pos, self.unit_size as i32);
        }
        // draw ball
        self.ball
            .render(&self.cam_pos, self.unit_size as i32, !self.is_moving);
        // draw score
        display::draw_string(
            &format!("score : {}", self.attempts),
            0,
            0,
            false,
            Color::BLACK,
            Color::GREEN,
        );
    }
}
