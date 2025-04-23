extern crate alloc;

use crate::game::module::{EmptyModule, SquareEndModule};
use crate::game::{ball::Ball, game::GameState};

use crate::PopMessage;
use crate::utils::vec::Vec2;
use alloc::boxed::Box;
use alloc::vec;
use graphmgr::*;
use libnw::{
    display::{self, Color, LARGE_CHAR_HEIGHT, LARGE_CHAR_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH},
    keyboard::{KeyboardTimedState, RawKey},
};

pub struct MainMenuState;

impl State<PopMessage> for MainMenuState {
    fn update(&mut self, keyboard_state: &KeyboardTimedState) -> StackAction<PopMessage> {
        if keyboard_state.is_key_just_pressed(RawKey::Ok)
            || keyboard_state.is_key_just_pressed(RawKey::Exe)
        {
            StackAction::Push(Box::new(GameState::new(
                vec![
                    Box::new(EmptyModule::new_rect(Vec2 { x: -8, y: -8 }, (8, 4))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 24, y: -8 }, (2, 7))),
                    Box::new(SquareEndModule::new_4x4(Vec2 { x: 20, y: 20 })),
                ],
                Ball::new(Vec2 { x: 0., y: 0. }),
            )))
        } else if keyboard_state.is_key_just_pressed(RawKey::Back) {
            StackAction::Pop(PopMessage::None)
        } else {
            StackAction::Nop
        }
    }

    fn render(&mut self) {
        const MSG: &str = "Press OK to play !";
        display::eadk::wait_for_vblank();
        display::clear_screen(Color::GREEN);
        display::draw_string(
            MSG,
            (SCREEN_WIDTH - (MSG.len() as u16 * LARGE_CHAR_WIDTH)) / 2,
            (SCREEN_HEIGHT - LARGE_CHAR_HEIGHT) / 2,
            true,
            Color::BLACK,
            Color::GREEN,
        );
    }
}
