use crate::{
    eadk::input::{Key, KeyboardState},
    escher::{
        components::{BoxContainer, MarginType},
        text::Label,
        Component, TopLevel, UiEvent,
    },
    math::Vec2i,
};

pub struct Menu<'a> {
    child: &'a mut dyn Component,
}

impl<'a> Menu<'a> {
    pub fn new() -> Self {
        let txt = [0, 0, 0, 0, 0];
        let mut label = Label {
            text: &txt,
            selected: true,
        };
        let mut container = BoxContainer {
            child: &mut label,
            margin_top: MarginType::Margin(10),
            margin_left: MarginType::Margin(10),
            margin_bottom: MarginType::Extend,
            margin_right: MarginType::Extend,
        };
        let menu = Menu {
            child: unsafe {
                let container_ptr: *mut BoxContainer = &mut container;
                core::mem::forget(container);
                &mut *container_ptr as &'static mut dyn Component
            },
        };
        core::mem::forget(txt);
        core::mem::forget(label);
        // core::mem::forget(container);
        menu
    }
}

impl<'a> TopLevel for Menu<'a> {
    fn update(&mut self) {
        self.child.update(UiEvent::None);
        let keys = KeyboardState::scan();
        if keys.key_down(Key::Exe) {
            todo!("launch level")
        }
    }

    fn render(&self, buffer: &mut crate::graphics::Buffer) {
        self.child.render(buffer, Vec2i { x: 0, y: 0 });
    }
}
