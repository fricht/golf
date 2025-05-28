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
                    // pt 1
                    Box::new(EmptyModule::new_rect(Vec2 { x: 80, y: 84 }, (5, 5))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 100, y: 88 }, (2, 3))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 108, y: 84 }, (5, 5))),
                    // pt 2
                    Box::new(EmptyModule::new_rect(Vec2 { x: 128, y: 100 }, (1, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 124, y: 104 }, (1, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 128, y: 104 }, (6, 2))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 148, y: 72 }, (1, 8))),
                    // pt 3
                    Box::new(EmptyModule::new_rect(Vec2 { x: 148, y: 64 }, (2, 2))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 36, y: 68 }, (28, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 72, y: 72 }, (1, 12))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 76, y: 116 }, (21, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 156, y: 72 }, (1, 11))),
                    // pt4
                    Box::new(EmptyModule::new_rect(Vec2 { x: 48, y: 72 }, (1, 2))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 40, y: 72 }, (1, 4))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 44, y: 84 }, (2, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 56, y: 72 }, (1, 4))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 60, y: 80 }, (2, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 64, y: 84 }, (1, 4))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 44, y: 92 }, (5, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 52, y: 96 }, (1, 2))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 32, y: 80 }, (2, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 20, y: 84 }, (1, 2))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 24, y: 88 }, (2, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 32, y: 84 }, (1, 3))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 36, y: 92 }, (1, 3))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 40, y: 100 }, (2, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 60, y: 48 }, (1, 5))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 48, y: 52 }, (3, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 48, y: 56 }, (1, 3))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 16, y: 60 }, (8, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 24, y: 44 }, (5, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 24, y: 48 }, (1, 2))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 40, y: 48 }, (1, 2))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 32, y: 48 }, (1, 3))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 8, y: 48 }, (3, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 12, y: 56 }, (2, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 16, y: 52 }, (1, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 28, y: 64 }, (1, 2))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 24, y: 68 }, (1, 3))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 12, y: 72 }, (3, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 4, y: 56 }, (1, 5))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 0, y: 80 }, (1, 3))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 0, y: 76 }, (4, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 12, y: 80 }, (1, 5))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 8, y: 92 }, (1, 3))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 16, y: 96 }, (3, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 20, y: 100 }, (1, 1))),
                    // pt 5
                    Box::new(EmptyModule::new_rect(Vec2 { x: 4, y: 8 }, (1, 10))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 8, y: 8 }, (8, 2))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 40, y: 0 }, (6, 6))),
                    // pt 6
                    Box::new(EmptyModule::new_rect(Vec2 { x: 64, y: 8 }, (1, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 68, y: 12 }, (1, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 72, y: 8 }, (1, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 76, y: 12 }, (1, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 80, y: 8 }, (1, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 84, y: 12 }, (4, 1))),
                    Box::new(EmptyModule::new_rect(Vec2 { x: 100, y: 8 }, (4, 1))),
                    // finish
                    Box::new(SquareEndModule::new_4x4(Vec2 { x: 116, y: 0 })),
                ],
                // vec![Box::new(SquareEndModule::new_4x4(Vec2 { x: -1, y: -1 }))], // test : only the end
                Ball::new(Vec2 { x: 90., y: 94. }),
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
