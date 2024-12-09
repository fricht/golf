use super::{Component, UiEvent};
use crate::alloc::boxed::Box;
use crate::eadk::display::push_rect;
use crate::{
    eadk::{display::SCREEN_HEIGHT, Color, Rect},
    graphics::Buffer,
    math::Vec2i,
};

const W: u16 = 0xFFFF;
const B: u16 = 0x0;

const CHARS_COUNT: usize = 44;
const CHARS_RAW_DATA: [u16; CHARS_COUNT * CHAR_SIZE] = [
    // NULL
    B, B, W, B, B, //
    B, W, W, W, B, //
    W, W, W, W, W, //
    W, W, W, W, W, //
    W, W, W, W, W, //
    W, W, W, W, W, //
    W, W, W, W, W, //
    W, W, W, W, W, //
    B, W, W, W, B, //
    B, B, W, B, B, //
    // a
    B, B, W, B, B, //
    B, W, B, W, B, //
    B, W, B, W, B, //
    B, W, B, W, B, //
    W, W, W, W, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    // b
    W, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, W, W, W, B, //
    // c
    B, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, W, //
    B, W, W, W, B, //
    // d
    W, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, W, W, W, B, //
    // e
    W, W, W, W, W, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, W, W, W, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, W, W, W, W, //
    // f
    W, W, W, W, W, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, W, W, W, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    // g
    B, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, W, W, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    B, W, W, W, B, //
    // h
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, W, W, W, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    // i
    W, W, W, W, W, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    W, W, W, W, W, //
    // j
    W, W, W, W, W, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    W, W, B, B, B, //
    // k
    W, B, B, B, W, //
    W, B, B, W, B, //
    W, B, W, B, B, //
    W, B, W, B, B, //
    W, W, B, B, B, //
    W, B, W, B, B, //
    W, B, W, B, B, //
    W, B, B, W, B, //
    W, B, B, W, B, //
    W, B, B, B, W, //
    // l
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, W, W, W, W, //
    // m
    W, B, B, B, W, //
    W, W, B, W, W, //
    W, W, B, W, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    // n
    W, W, B, B, W, //
    W, W, B, B, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    W, B, B, W, W, //
    W, B, B, W, W, //
    // o
    B, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    B, W, W, W, B, //
    // p
    W, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, W, W, W, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    // q
    B, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    B, W, W, W, B, //
    // r
    W, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, W, W, W, B, //
    W, B, B, B, B, //
    W, W, B, B, B, //
    W, B, W, B, B, //
    W, B, B, W, B, //
    W, B, B, B, W, //
    // s
    B, W, W, W, W, //
    W, B, B, B, B, //
    B, W, B, B, B, //
    B, W, B, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, B, W, B, //
    B, B, B, W, B, //
    B, B, B, B, W, //
    W, W, W, W, W, //
    // t
    W, W, W, W, W, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    // u
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    B, W, W, W, B, //
    // v
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    B, W, B, W, B, //
    B, W, B, W, B, //
    B, W, B, W, B, //
    B, W, B, W, B, //
    B, W, B, W, B, //
    B, B, W, B, B, //
    // w
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    B, W, B, W, B, //
    // x
    W, B, B, B, W, //
    W, B, B, B, W, //
    B, W, B, W, B, //
    B, W, B, W, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, W, B, W, B, //
    B, W, B, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    // y
    W, B, B, B, W, //
    W, B, B, B, W, //
    B, W, B, W, B, //
    B, W, B, W, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    // z
    W, W, W, W, W, //
    W, B, B, B, B, //
    B, W, B, B, B, //
    B, W, B, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, B, W, B, //
    B, B, B, W, B, //
    B, B, B, B, W, //
    W, W, W, W, W, //
    // _
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    // .
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, W, W, B, //
    B, B, W, W, B, //
    // :
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    // 0
    B, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, W, B, W, //
    W, B, W, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    B, W, W, W, B, //
    // 1
    B, B, B, B, W, //
    B, B, B, W, W, //
    B, W, W, B, W, //
    W, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    // 2
    B, W, W, W, B, //
    B, W, B, B, W, //
    W, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, W, B, //
    B, B, W, B, B, //
    B, W, B, B, B, //
    W, B, B, B, B, //
    W, W, W, W, W, //
    // 3
    B, W, W, W, B, //
    W, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    B, B, W, W, B, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    W, B, B, B, W, //
    B, W, W, W, B, //
    // 4
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, W, W, W, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    // 5
    W, W, W, W, W, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, W, W, W, B, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    W, B, B, B, W, //
    B, W, W, W, B, //
    // 6
    B, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    B, W, W, W, B, //
    // 7
    W, W, W, W, W, //
    W, B, B, B, B, //
    B, W, B, B, B, //
    B, W, B, B, B, //
    B, B, W, W, W, //
    B, B, W, B, B, //
    B, B, B, W, B, //
    B, B, B, W, B, //
    B, B, B, B, W, //
    B, B, B, B, W, //
    // 8
    B, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    B, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    B, W, W, W, B, //
    // 9
    B, W, W, W, B, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    W, B, B, B, W, //
    B, W, W, W, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, W, //
    B, W, W, W, B, //
    // (
    B, B, B, W, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, W, B, B, B, //
    B, W, B, B, B, //
    B, W, B, B, B, //
    B, W, B, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, B, W, B, //
    // )
    B, W, B, B, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, B, B, W, B, //
    B, B, B, W, B, //
    B, B, B, W, B, //
    B, B, B, W, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, W, B, B, B, //
    // /
    B, B, B, B, W, //
    B, B, B, B, W, //
    B, B, B, W, B, //
    B, B, B, W, B, //
    B, B, W, B, B, //
    B, B, W, B, B, //
    B, W, B, B, B, //
    B, W, B, B, B, //
    W, B, B, B, B, //
    W, B, B, B, B, //
    // _
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    B, B, B, B, B, //
    W, W, W, W, W, //
];

const CHAR_COLORKEY: Color = Color { rgb565: 0xFFFF };
const CHAR_SIZE: usize = 5 * 10;
const CHARS: [Color; CHARS_COUNT * CHAR_SIZE] = build_colors(CHARS_RAW_DATA);

const fn build_colors(data: [u16; CHARS_COUNT * CHAR_SIZE]) -> [Color; CHARS_COUNT * CHAR_SIZE] {
    let mut colors = [Color { rgb565: 0 }; CHARS_COUNT * CHAR_SIZE];
    let mut i = 0;
    while i < data.len() {
        colors[i] = Color { rgb565: data[i] };
        i += 1;
    }
    colors
}

pub fn get_char_data(char: &u8) -> &[Color] {
    let i = match char {
        b'a' | b'A' => 1,
        b'b' | b'B' => 2,
        b'c' | b'C' => 3,
        b'd' | b'D' => 4,
        b'e' | b'E' => 5,
        b'f' | b'F' => 6,
        b'g' | b'G' => 7,
        b'h' | b'H' => 8,
        b'i' | b'I' => 9,
        b'j' | b'J' => 10,
        b'k' | b'K' => 11,
        b'l' | b'L' => 12,
        b'm' | b'M' => 13,
        b'n' | b'N' => 14,
        b'o' | b'O' => 15,
        b'p' | b'P' => 16,
        b'q' | b'Q' => 17,
        b'r' | b'R' => 18,
        b's' | b'S' => 19,
        b't' | b'T' => 20,
        b'u' | b'U' => 21,
        b'v' | b'V' => 22,
        b'w' | b'W' => 23,
        b'x' | b'X' => 24,
        b'y' | b'Y' => 25,
        b'z' | b'Z' => 26,
        b' ' => 27,
        b'.' => 28,
        b':' => 29,
        b'0' => 30,
        b'1' => 31,
        b'2' => 32,
        b'3' => 33,
        b'4' => 34,
        b'5' => 35,
        b'6' => 36,
        b'7' => 37,
        b'8' => 38,
        b'9' => 39,
        b'(' => 40,
        b')' => 41,
        b'/' => 42,
        b'_' => 43,
        _ => 0,
    };
    &CHARS[(i * CHAR_SIZE)..((i + 1) * CHAR_SIZE)]
}

/// draw ugly text to the screen (for debug purposes) and returns new cursor position
pub fn draw_debug_text(txt: &str, offset: (u16, u16)) -> (u16, u16) {
    let (mut x, mut y) = offset;
    for c in txt.chars() {
        let c = c as u8;
        push_rect(
            Rect {
                x,
                y,
                width: 5,
                height: 10,
            },
            get_char_data(&c),
        );
        x += 6;
        if x > 314 {
            x = 0;
            y += 11;
        }
    }
    (x, y)
}

pub struct Label {
    pub text: Box<[u8]>,
    pub selected: bool,
}

impl Component for Label {
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
