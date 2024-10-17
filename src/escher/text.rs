use super::{Component, UiEvent};
use crate::{
    eadk::{display::SCREEN_HEIGHT, Color, Rect},
    graphics::Buffer,
    math::Vec2i,
};

const CHAR_COLORKEY: Color = Color { rgb565: 0xFFFF };
const CHAR_SIZE: usize = 5 * 10;
const CHARS: [Color; 27 * CHAR_SIZE] = build_colors([0; 27 * CHAR_SIZE]);

const fn build_colors(data: [u16; 27 * CHAR_SIZE]) -> [Color; 27 * CHAR_SIZE] {
    let mut colors = [Color { rgb565: 0 }; 27 * CHAR_SIZE];
    let mut i = 0;
    while i < data.len() {
        colors[i] = Color { rgb565: data[i] };
        i += 1;
    }
    colors
}

fn get_char_data(char: &u8) -> &[Color] {
    let i = match char {
        b'a' => 1,
        b'b' => 2,
        b'c' => 3,
        b'd' => 4,
        b'e' => 5,
        b'f' => 6,
        b'g' => 7,
        b'h' => 8,
        b'i' => 9,
        b'j' => 10,
        b'k' => 11,
        b'l' => 12,
        b'm' => 13,
        b'n' => 14,
        b'o' => 15,
        b'p' => 16,
        b'q' => 17,
        b'r' => 18,
        b's' => 19,
        b't' => 20,
        b'u' => 21,
        b'v' => 22,
        b'w' => 23,
        b'x' => 24,
        b'y' => 25,
        b'z' => 26,
        _ => 0,
    };
    &CHARS[(i * CHAR_SIZE)..(i + 1 * CHAR_SIZE)]
}

pub struct Label<'a> {
    pub text: &'a [u8],
    pub selected: bool,
}

impl<'a> Component for Label<'a> {
    fn update(&mut self, signal: UiEvent) -> UiEvent {
        signal
    }

    fn render(&self, buffer: &mut Buffer, offset: Vec2i) {
        if !(offset.y >= 0 && offset.y + 10 < SCREEN_HEIGHT as i32) {
            return;
        }
        let mut i = 0;
        while i < self.text.len() {
            let pos_x = offset.x + i as i32 * 5;
            if pos_x >= 0 && pos_x < SCREEN_HEIGHT as i32 {
                buffer.push_rect_alpha(
                    Rect {
                        x: pos_x as u16,
                        y: offset.y as u16,
                        width: 5,
                        height: 10,
                    },
                    get_char_data(&self.text[i]),
                    CHAR_COLORKEY,
                );
            }
            i += 1;
        }
    }

    fn requested_size(&self) -> Vec2i {
        Vec2i {
            x: self.text.len() as i32 * 5,
            y: 10,
        }
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    fn get_selected(&self) -> bool {
        self.selected
    }
}
