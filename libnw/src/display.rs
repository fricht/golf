//! Interface with the display.
//!
//! This module offers high-level, safe access to screen rendering functionalities
//! on the NumWorks calculator. It includes:
//!
//! - Pixel-level manipulation (`set_pixel`, `get_pixel`)
//! - Rectangle drawing and filling (`Rect`)
//! - Text rendering (`draw_string`)
//! - RGB color handling with `Color`
//!
//! For low-level control, the internal `eadk` submodule exposes raw FFI bindings.
//! Prefer using the safe API unless you have specific performance or control needs.

extern crate alloc;

use alloc::{borrow::Cow, string::String, vec::Vec};

/// The width of the screen in pixels.
pub const SCREEN_WIDTH: u16 = 320;
/// The height of the screen in pixels.
pub const SCREEN_HEIGHT: u16 = 240;
/// The number of pixels on the screen.
pub const SCREEN_AREA: usize = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize;
/// The width of one char of the standard font in pixels.
pub const CHAR_WIDTH: u16 = 7;
/// The height of one char of the standard font in pixels.
pub const CHAR_HEIGHT: u16 = 14;
/// The width of one char of the large font in pixels.
pub const LARGE_CHAR_WIDTH: u16 = 10;
/// The height of one char of the large font in pixels.
pub const LARGE_CHAR_HEIGHT: u16 = 16;

/// A rectangle on the screen.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    /// Creates a new `Rect` from raw components.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn screen_space_clipping(x: i32, y: i32, width: u16, height: u16) -> Self {
        let x = x.clamp(0, SCREEN_WIDTH as i32) as u16;
        let y = y.clamp(0, SCREEN_HEIGHT as i32) as u16;
        let ex = (x + width).clamp(0, SCREEN_WIDTH);
        let ey = (y + height).clamp(0, SCREEN_HEIGHT);
        Self {
            x,
            y,
            width: ex - x,
            height: ey - y,
        }
    }

    /// Creates a square.
    pub fn new_square(x: u16, y: u16, width: u16) -> Self {
        Self::new(x, y, width, width)
    }

    /// Creates a square with side-length 1, i.e, a pixel.
    pub fn new_pixel(x: u16, y: u16) -> Self {
        Self::new_square(x, y, 1)
    }

    /// The number of pixels covered by the rectangle.
    pub fn area(&self) -> u32 {
        self.width as u32 * self.height as u32
    }

    /// Fills the rect on the screen with the given color.
    pub fn fill(self, color: Color) {
        eadk::push_rect_uniform(self, color);
    }

    /// Fills the rect on the screen with the given pixel colors.
    ///
    /// # Panics
    /// Panics in debug builds if the slice length doesn't match the area of the rect.
    /// In release builds, this won't be checked and may lead to visual bugs on the screen.
    pub fn fill_with_buf(self, pixels: &[Color]) {
        // The user must ensure that the sizes match.
        debug_assert!(self.area() as usize == pixels.len());
        unsafe {
            // SAFETY: we assert that the `pixels` slice's length
            // matches the area of the `self`.
            eadk::push_rect(self, pixels.as_ptr());
        };
    }

    /// Returns the pixels' color in the given rect.
    pub fn get_pixels(self) -> Vec<Color> {
        get_rect(self)
    }

    /// Centers the rectangle on the screen
    pub fn center(&mut self) {
        self.x = (SCREEN_WIDTH - self.width) / 2;
        self.y = (SCREEN_HEIGHT - self.height) / 2;
    }

    /// The rectangle the size of the screen.
    pub const SCREEN: Self = Self {
        x: 0,
        y: 0,
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
    };
}

/// An RGB 5-6-5 color: 5 bits for red, 6 bits for green and 5 bits for blue.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Color(u16);

impl Color {
    /// Creates a color directly from a u16.
    pub fn new(rgb565: u16) -> Self {
        Self(rgb565)
    }

    /// Creates a color from distinct red, green and blue channels.
    ///
    /// Converts 8-bit per channel RGB values into RGB565 format.
    /// Precision is lost in the conversion: red and blue use 5 bits, green uses 6 bits.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let r = (r & 0b11111000) as u16;
        let g = (g & 0b11111100) as u16;
        let b = b as u16;
        Self((r << 8) | (g << 3) | (b >> 3))
    }

    /// Separates the color into 3 channels (0 to 255).
    pub fn separate_rgb(&self) -> (u8, u8, u8) {
        let mut r = ((self.0 >> 8) & 0b11111000) as u8;
        r = r | (r >> 5);
        let mut g = ((self.0 >> 3) & 0b11111100) as u8;
        g = g | (g >> 6);
        let mut b = ((self.0 & 0b11111) << 3) as u8;
        b = b | (b >> 5);
        (r, g, b)
    }

    // Basic colors
    /// Pure black (0, 0, 0)
    pub const BLACK: Self = Self(0x0);
    /// Pure white (255, 255, 255)
    pub const WHITE: Self = Self(0xFFFF);
    /// Pure red (255, 0, 0)
    pub const RED: Self = Self(0xF800);
    /// Pure green (0, 255, 0)
    pub const GREEN: Self = Self(0x7E0);
    /// Pure blue (0, 0, 255)
    pub const BLUE: Self = Self(0x1F);
}

/// Returns the pixels' color in the given rect.
///
/// The screen is read from left to right then top to bottom.
pub fn get_rect(rect: Rect) -> Vec<Color> {
    let buffer_size = rect.area();
    let mut pixels = Vec::with_capacity(buffer_size as usize);
    unsafe {
        // SAFETY: the `pixels` Vec is created with the same capacity as
        // the area covered by `rect`.
        eadk::pull_rect(rect, pixels.as_mut_ptr());
        // SAFETY: we need to manually adjust the vec length, otherwise
        // vec.len() will return 0 even though it is fully filled.
        pixels.set_len(buffer_size as usize);
    };
    pixels
}

/// Retrieves the color of a pixel.
pub fn get_pixel(x: u16, y: u16) -> Color {
    Rect::new_pixel(x, y).get_pixels()[0]
}

/// Colors a pixel.
pub fn set_pixel(x: u16, y: u16, color: Color) {
    Rect::new_pixel(x, y).fill(color);
}

/// Fills the screen with the given color.
pub fn clear_screen(color: Color) {
    Rect::SCREEN.fill(color);
}

/// Draws a string on screen.
///
/// The x and y coordinates corresponds to the top-left corner of the string's bounding-box.
///
/// Two font sizes are available and the choice is made using the `large_font` boolean parameter.
pub fn draw_string(
    string: &str,
    x: u16,
    y: u16,
    large_font: bool,
    text_color: Color,
    background_color: Color,
) {
    let patched_string = terminate_str(string);
    unsafe {
        // SAFETY: a null byte is added if necessary with the `terminate_str` function above.
        eadk::draw_string(
            patched_string.as_ptr(),
            x,
            y,
            large_font,
            text_color,
            background_color,
        )
    };
}

/// Adds a null byte at the end of an str if needed.
fn terminate_str(s: &str) -> Cow<'_, str> {
    if s.ends_with('\0') {
        Cow::Borrowed(s)
    } else {
        let mut owned = String::with_capacity(s.len() + 1);
        owned.push_str(s);
        owned.push('\0');
        Cow::Owned(owned)
    }
}

/// Interface with the raw `eadk` C api.
///
/// If you don't know what you are doing, use the safe rust implementations.
pub mod eadk {
    use super::{Color, Rect};

    /// Draws a str to the screen.
    ///
    /// # Safety
    /// It is your responsibility to ensure that the str is terminated with a null byte.
    pub unsafe fn draw_string(
        text: *const u8,
        x: u16,
        y: u16,
        large_font: bool,
        text_color: Color,
        background_color: Color,
    ) {
        unsafe {
            eadk_display_draw_string(
                text,
                Point { x, y },
                large_font,
                text_color,
                background_color,
            );
        };
    }

    unsafe extern "C" {
        fn eadk_display_draw_string(
            text: *const u8,
            point: Point,
            large_font: bool,
            text_color: Color,
            background_color: Color,
        );

        /// Pushes a slice of colors onto the screen.
        ///
        /// The screen is filled from left to right then top to bottom.
        ///
        /// # Safety
        /// It is your responsibility to ensure that the rect and the slice's length match.
        #[link_name = "eadk_display_push_rect"]
        pub fn push_rect(rect: Rect, pixels: *const Color);

        /// Pull pixels from the screen into a slice of colors.
        ///
        /// The screen is read from left to right then top to bottom.
        ///
        /// # Safety
        /// It is your responsibility to ensure that the rect and the slice's length match.
        #[link_name = "eadk_display_pull_rect"]
        pub fn pull_rect(rect: Rect, pixels: *mut Color);

        /// Draws a rect with the given color.
        #[link_name = "eadk_display_push_rect_uniform"]
        pub safe fn push_rect_uniform(rect: Rect, color: Color);

        /// Waits for the screen to finish refreshing.
        #[link_name = "eadk_display_wait_for_vblank"]
        pub safe fn wait_for_vblank() -> bool;
    }

    /// A point on the screen.
    ///
    /// This is only needed for the eadk_display_draw_string and should not be used elsewhere.
    #[repr(C)]
    struct Point {
        pub x: u16,
        pub y: u16,
    }
}
