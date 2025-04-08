use crate::escher::text::draw_debug_text;
use core::{panic::PanicInfo, str};
use display::{SCREEN_HEIGHT, SCREEN_WIDTH};
use timing::msleep;

// heap
extern "C" {
    pub static _heap_start: usize;
    pub static _heap_end: usize;
}
// what am i doing ...
#[no_mangle]
pub extern "C" fn _critical_section_1_0_acquire() {
    // Do nothing
}
#[no_mangle]
pub extern "C" fn _critical_section_1_0_release() {
    // Do nothing
}
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    // Do nothing
}
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    let mut bytes_size = [0x20; 20];
    let mut bytes_align = [0x20; 20];
    int_to_str(layout.size(), &mut bytes_size);
    int_to_str(layout.align(), &mut bytes_align);
    // "allocation error : size = .........., align = .........."
    let mut full = [
        97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 114, 114, 111, 114, 32, 58, 32,
        115, 105, 122, 101, 32, 61, 32, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 44, 32, 97, 108,
        105, 103, 110, 32, 61, 32, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46,
    ];
    display::push_rect_uniform(
        Rect {
            x: 0,
            y: 0,
            width: 320,
            height: 240,
        },
        Color { rgb565: 0x0 },
    );
    full[26..36].copy_from_slice(&bytes_size[10..]);
    full[46..56].copy_from_slice(&bytes_align[10..]);
    draw_debug_text(str::from_utf8(&full).unwrap(), (1, 1));
    msleep(5000);
    panic!("allocation error (fuck panic macro)");
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    pub rgb565: u16,
}

/// color utilities
impl Color {
    /// get red color component (5 bits)
    pub fn get_red_raw(&self) -> u16 {
        (self.rgb565 >> 11) & 0b11111
    }

    /// set red color component (5 bits)
    pub fn set_red_raw(&mut self, red: u16) {
        self.rgb565 = (self.rgb565 & 0b0000011111111111) | ((red & 0b11111) << 11)
    }

    /// get green color component (6 bits)
    pub fn get_green_raw(&self) -> u16 {
        (self.rgb565 >> 5) & 0b111111
    }

    /// set green color component (6 bits)
    pub fn set_green_raw(&mut self, green: u16) {
        self.rgb565 = (self.rgb565 & 0b1111100000011111) | ((green & 0b111111) << 5)
    }

    /// get blue color component (5 bits)
    pub fn get_blue_raw(&self) -> u16 {
        self.rgb565 & 0b11111
    }

    /// set blue color component (5 bits)
    pub fn set_blue_raw(&mut self, blue: u16) {
        self.rgb565 = (self.rgb565 & 0b1111111111100000) | (blue & 0b11111)
    }

    /// linearely interpolate 2 colors
    pub fn lerp(self, target: Self, t: f32) -> Self {
        let mut c = Color { rgb565: 0 };
        c.set_red_raw(
            (self.get_red_raw() as f32
                + (target.get_red_raw() as f32 - self.get_red_raw() as f32) * t) as u16,
        );
        c.set_green_raw(
            (self.get_green_raw() as f32
                + (target.get_green_raw() as f32 - self.get_green_raw() as f32) * t)
                as u16,
        );
        c.set_blue_raw(
            (self.get_blue_raw() as f32
                + (target.get_blue_raw() as f32 - self.get_blue_raw() as f32) * t)
                as u16,
        );
        c
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

/// rect utilities
impl Rect {
    pub fn screen_space_culling(x: i32, y: i32, width: i32, height: i32) -> Self {
        let clamped_x = x.clamp(0, SCREEN_WIDTH as i32 - 1);
        let clamped_y = y.clamp(0, SCREEN_HEIGHT as i32 - 1);
        Rect {
            x: clamped_x as u16,
            y: clamped_y as u16,
            width: ((x + width).clamp(0, SCREEN_WIDTH as i32 - 1) - clamped_x) as u16,
            height: ((y + height).clamp(0, SCREEN_HEIGHT as i32 - 1) - clamped_y) as u16,
        }
    }
}

pub mod backlight {
    pub fn set_brightness(brightness: u8) {
        unsafe {
            eadk_backlight_set_brightness(brightness);
        }
    }
    pub fn brightness() -> u8 {
        unsafe {
            return eadk_backlight_brightness();
        }
    }

    extern "C" {
        fn eadk_backlight_set_brightness(brightness: u8);
        fn eadk_backlight_brightness() -> u8;
    }
}

pub mod display {
    use super::Color;
    use super::Rect;

    pub const SCREEN_WIDTH: u16 = 320;
    pub const SCREEN_HEIGHT: u16 = 240;
    pub const RATIO: f64 = 320. / 240.;

    pub fn push_rect(rect: Rect, pixels: &[Color]) {
        unsafe {
            eadk_display_push_rect(rect, pixels.as_ptr());
        }
    }

    pub fn push_rect_uniform(rect: Rect, color: Color) {
        unsafe {
            eadk_display_push_rect_uniform(rect, color);
        }
    }

    pub fn wait_for_vblank() {
        unsafe {
            eadk_display_wait_for_vblank();
        }
    }

    extern "C" {
        fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
        fn eadk_display_push_rect(rect: Rect, color: *const Color);
        fn eadk_display_wait_for_vblank();
    }
}

pub mod timing {
    pub fn usleep(us: u32) {
        unsafe {
            eadk_timing_usleep(us);
        }
    }

    pub fn msleep(ms: u32) {
        unsafe {
            eadk_timing_msleep(ms);
        }
    }

    pub fn millis() -> u64 {
        unsafe {
            return eadk_timing_millis();
        }
    }

    extern "C" {
        fn eadk_timing_usleep(us: u32);
        fn eadk_timing_msleep(us: u32);
        fn eadk_timing_millis() -> u64;
    }
}

pub fn random() -> u32 {
    unsafe { return eadk_random() }
}

extern "C" {
    fn eadk_random() -> u32;
}

pub mod input {
    type EadkKeyboardState = u64;

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Key {
        Left = 0,
        Up = 1,
        Down = 2,
        Right = 3,
        Ok = 4,
        Back = 5,
        Home = 6,
        OnOff = 8,
        Shift = 12,
        Alpha = 13,
        Xnt = 14,
        Var = 15,
        Toolbox = 16,
        Backspace = 17,
        Exp = 18,
        Ln = 19,
        Log = 20,
        Imaginary = 21,
        Comma = 22,
        Power = 23,
        Sine = 24,
        Cosine = 25,
        Tangent = 26,
        Pi = 27,
        Sqrt = 28,
        Square = 29,
        Seven = 30,
        Eight = 31,
        Nine = 32,
        LeftParenthesis = 33,
        RightParenthesis = 34,
        Four = 36,
        Five = 37,
        Six = 38,
        Multiplication = 39,
        Division = 40,
        One = 42,
        Two = 43,
        Three = 44,
        Plus = 45,
        Minus = 46,
        Zero = 48,
        Dot = 49,
        Ee = 50,
        Ans = 51,
        Exe = 52,
    }

    extern "C" {
        fn eadk_keyboard_scan() -> EadkKeyboardState;
    }

    #[derive(Clone, Copy)]
    pub struct KeyboardState(EadkKeyboardState);

    impl KeyboardState {
        pub fn scan() -> Self {
            Self::from_raw(unsafe { eadk_keyboard_scan() })
        }

        pub fn from_raw(state: EadkKeyboardState) -> Self {
            Self(state)
        }

        pub fn key_down(&self, key: Key) -> bool {
            (self.0 >> (key as u8)) & 1 != 0
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u16)]
    pub enum Event {
        Left = 0,
        Up = 1,
        Down = 2,
        Right = 3,
        Ok = 4,
        Back = 5,
        Shift = 12,
        Alpha = 13,
        Xnt = 14,
        Var = 15,
        Toolbox = 16,
        Backspace = 17,
        Exp = 18,
        Ln = 19,
        Log = 20,
        Imaginary = 21,
        Comma = 22,
        Power = 23,
        Sine = 24,
        Cosine = 25,
        Tangent = 26,
        Pi = 27,
        Sqrt = 28,
        Square = 29,
        Seven = 30,
        Eight = 31,
        Nine = 32,
        LeftParenthesis = 33,
        RightParenthesis = 34,
        Four = 36,
        Five = 37,
        Six = 38,
        Multiplication = 39,
        Division = 40,
        One = 42,
        Two = 43,
        Three = 44,
        Plus = 45,
        Minus = 46,
        Zero = 48,
        Dot = 49,
        Ee = 50,
        Ans = 51,
        Exe = 52,
        ShiftLeft = 54,
        ShiftUp = 55,
        ShiftDown = 56,
        ShiftRight = 57,
        AlphaLock = 67,
        Cut = 68,
        Copy = 69,
        Paste = 70,
        Clear = 71,
        LeftBracket = 72,
        RightBracket = 73,
        LeftBrace = 74,
        RightBrace = 75,
        Underscore = 76,
        Sto = 77,
        Arcsine = 78,
        Arccosine = 79,
        Arctangent = 80,
        Equal = 81,
        Lower = 82,
        Greater = 83,
        Colon = 122,
        Semicolon = 123,
        DoubleQuotes = 124,
        Percent = 125,
        LowerA = 126,
        LowerB = 127,
        LowerC = 128,
        LowerD = 129,
        LowerE = 130,
        LowerF = 131,
        LowerG = 132,
        LowerH = 133,
        LowerI = 134,
        LowerJ = 135,
        LowerK = 136,
        LowerL = 137,
        LowerM = 138,
        LowerN = 139,
        LowerO = 140,
        LowerP = 141,
        LowerQ = 142,
        LowerR = 144,
        LowerS = 145,
        LowerT = 146,
        LowerU = 147,
        LowerV = 148,
        LowerW = 150,
        LowerX = 151,
        LowerY = 152,
        LowerZ = 153,
        Space = 154,
        Question = 156,
        Exclamation = 157,
        UpperA = 180,
        UpperB = 181,
        UpperC = 182,
        UpperD = 183,
        UpperE = 184,
        UpperF = 185,
        UpperG = 186,
        UpperH = 187,
        UpperI = 188,
        UpperJ = 189,
        UpperK = 190,
        UpperL = 191,
        UpperM = 192,
        UpperN = 193,
        UpperO = 194,
        UpperP = 195,
        UpperQ = 196,
        UpperR = 198,
        UpperS = 199,
        UpperT = 200,
        UpperU = 201,
        UpperV = 202,
        UpperW = 204,
        UpperX = 205,
        UpperY = 206,
        UpperZ = 207,
    }

    impl Event {
        pub fn is_digit(&self) -> bool {
            matches!(
                self,
                Event::Zero
                    | Event::One
                    | Event::Two
                    | Event::Three
                    | Event::Four
                    | Event::Five
                    | Event::Six
                    | Event::Seven
                    | Event::Eight
                    | Event::Nine
            )
        }

        pub fn to_digit(&self) -> Option<u8> {
            match self {
                Event::Zero => Some(0),
                Event::One => Some(1),
                Event::Two => Some(2),
                Event::Three => Some(3),
                Event::Four => Some(4),
                Event::Five => Some(5),
                Event::Six => Some(6),
                Event::Seven => Some(7),
                Event::Eight => Some(8),
                Event::Nine => Some(9),
                _ => None,
            }
        }
    }

    extern "C" {
        fn eadk_event_get(timeout: &i32) -> Event;
    }

    pub fn event_get(timeout: i32) -> Event {
        unsafe { eadk_event_get(&timeout) }
    }
}

pub fn int_to_str(int: usize, string: &mut [u8]) {
    let mut num = int;
    let mut pos = 19;
    loop {
        let c = b'0' + (num % 10) as u8;
        string[pos] = c;
        pos -= 1;
        num /= 10;
        if num == 0 {
            break;
        }
    }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo<'_>) -> ! {
    display::push_rect_uniform(
        Rect {
            x: 0,
            y: 0,
            width: 320,
            height: 240,
        },
        Color { rgb565: 0xF800 },
    );
    let (_, mut y) = draw_debug_text(
        panic_info.message().as_str().unwrap_or("no error message"),
        (1, 1),
    );
    let mut x = 1;
    y += 11;
    (x, y) = draw_debug_text("File : ", (x, y));
    let (mut bytes_row, mut bytes_col) = ([0x20; 20], [0x20; 20]);
    let (file, row, col) = match panic_info.location() {
        None => ("no file", "no row", "no col"),
        Some(loc) => (
            loc.file(),
            {
                int_to_str(loc.line() as usize, &mut bytes_row);
                str::from_utf8(&bytes_row[10..]).unwrap()
            },
            {
                int_to_str(loc.column() as usize, &mut bytes_col);
                str::from_utf8(&bytes_col[10..]).unwrap()
            },
        ),
    };
    (x, y) = draw_debug_text(file, (x, y));
    (x, y) = draw_debug_text(" ", (x, y));
    (x, y) = draw_debug_text(row, (x, y));
    (x, y) = draw_debug_text(":", (x, y));
    draw_debug_text(col, (x, y));
    loop {}
}
