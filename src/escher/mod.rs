pub mod components;
pub mod text;

use crate::{graphics::Buffer, math::Vec2i};

/*

Architecture :
Top-level :
    - Container
        - ...
            - Child


Events / Inputs handling :

The top-level window (or popup, you get it) handles the inputs
It sends the commands to it's child.
The signal goes up to the most inner cildren, that handles it.
Then it sends another signal to it's parent (eg : the same signal if it hasen't do anything with it)

*/

/// not selected color
pub const IDLE_COLOR: u16 = 57083;
/// selected color
pub const SELECT_COLOR: u16 = 48631;

/// ui events
pub enum UiEvent {
    None, // this allows to re-draw everything
    Ok,   // Ok or Exe
    Up,
    Down,
    Left,
    Right,
    Back,
    Number(u8),
    Backspace,
}

pub trait TopLevel {
    fn update(&mut self);
    fn render(&self, buffer: &mut Buffer);
}

pub trait Component {
    fn update(&mut self, signal: UiEvent) -> UiEvent;
    fn render(&self, buffer: &mut Buffer, offset: Vec2i);
    fn requested_size(&self) -> Vec2i;
    fn set_selected(&mut self, selected: bool);
    fn get_selected(&self) -> bool;
}
