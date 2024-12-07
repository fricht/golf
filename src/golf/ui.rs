use crate::alloc::boxed::Box;
use crate::{
    eadk::input::{Key, KeyboardState},
    escher::{
        components::{BoxContainer, MarginType},
        text::Label,
        Component, TopLevel,
    },
    math::Vec2i,
};

pub struct Menu {
    child: Box<dyn Component>,
}

impl Menu {
    pub fn new() -> Self {
        let txt = [0, 0, 0, 0, 0];
        let label = Label {
            text: Box::new(txt),
            selected: true,
        };
        let container = BoxContainer {
            child: Box::new(label),
            margin_top: MarginType::Margin(10),
            margin_left: MarginType::Margin(10),
            margin_bottom: MarginType::Extend,
            margin_right: MarginType::Extend,
        };
        let menu = Menu {
            child: Box::new(container),
        };
        menu
    }
}

impl TopLevel for Menu {
    fn update(&mut self) {
        // self.child.update(UiEvent::None);
        let keys = KeyboardState::scan();
        if keys.key_down(Key::Exe) {
            todo!("launch level")
        }
    }

    fn render(&self, buffer: &mut crate::graphics::Buffer) {
        self.child.render(buffer, Vec2i { x: 0, y: 0 });
    }
}
