use alloc::format;
use graphmgr::{StackAction, State};
use libnw::{
    display::{self, Color, LARGE_CHAR_HEIGHT, LARGE_CHAR_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH},
    keyboard::{KeyboardTimedState, RawKey},
};

use crate::PopMessage;

pub struct GameOverState(pub u8);

impl State<PopMessage> for GameOverState {
    fn update(&mut self, keyboard_state: &KeyboardTimedState) -> StackAction<PopMessage> {
        if keyboard_state.is_key_just_pressed(RawKey::Back)
            || keyboard_state.is_key_just_pressed(RawKey::Ok)
            || keyboard_state.is_key_just_pressed(RawKey::Exe)
        {
            StackAction::Pop(PopMessage::None)
        } else {
            StackAction::Nop
        }
    }

    fn render(&mut self) {
        display::eadk::wait_for_vblank();
        display::clear_screen(Color::GREEN);
        let msg = format!("GG, done in {} hits", self.0);
        display::draw_string(
            &msg,
            (SCREEN_WIDTH - (msg.len() as u16 * LARGE_CHAR_WIDTH)) / 2,
            (SCREEN_HEIGHT - LARGE_CHAR_HEIGHT) / 2,
            true,
            Color::BLACK,
            Color::GREEN,
        );
    }
}
