use crate::PopMessage;
use alloc::string::String;
use graphmgr::{StackAction, State};
use libnw::{
    display::{self, Color, LARGE_CHAR_HEIGHT, LARGE_CHAR_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH},
    keyboard::{KeyboardTimedState, RawKey},
};

pub struct PopupState(pub String);

impl State<PopMessage> for PopupState {
    fn update(&mut self, keyboard_state: &KeyboardTimedState) -> StackAction<PopMessage> {
        if keyboard_state.is_key_just_pressed(RawKey::Back) {
            StackAction::Pop(PopMessage::OkBackPopupIsOk(false))
        } else if keyboard_state.is_key_just_pressed(RawKey::Ok)
            || keyboard_state.is_key_just_pressed(RawKey::Exe)
        {
            StackAction::Pop(PopMessage::OkBackPopupIsOk(true))
        } else {
            StackAction::Nop
        }
    }

    fn render(&mut self) {
        const OK_BACK_MSG: &str = "Ok        Back";
        display::eadk::wait_for_vblank();
        display::clear_screen(Color::GREEN);
        display::draw_string(
            &self.0,
            (SCREEN_WIDTH - (self.0.len() as u16 * LARGE_CHAR_WIDTH)) / 2,
            SCREEN_HEIGHT / 2 - 2 * LARGE_CHAR_HEIGHT,
            true,
            Color::BLACK,
            Color::GREEN,
        );
        display::draw_string(
            OK_BACK_MSG,
            (SCREEN_WIDTH - (OK_BACK_MSG.len() as u16 * LARGE_CHAR_WIDTH)) / 2,
            SCREEN_HEIGHT / 2 + LARGE_CHAR_HEIGHT,
            true,
            Color::BLACK,
            Color::GREEN,
        );
    }
}
