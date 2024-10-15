use crate::graphics::Buffer;

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

/// ui events
pub enum UiEvent {
    Ok,
    Up,
    Down,
    Left,
}

pub trait TopLevel {
    fn render(&self, buffer: &mut Buffer);
}

pub trait Component {
    fn render(&self, buffer: &mut Buffer);
}
