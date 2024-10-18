use crate::{
    eadk::input::{Key, KeyboardState},
    escher::{
        components::{BoxContainer, MarginType},
        text::Label,
        Component, TopLevel,
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
            text: unsafe {
                let txt_ptr: *const [u8] = &txt;
                &*txt_ptr as &'static [u8]
            },
            selected: true,
        };
        // core::mem::forget(txt);
        let mut container = BoxContainer {
            child: unsafe {
                let label_ptr: *mut Label = &mut label;
                core::mem::forget(label);
                &mut *label_ptr as &'static mut dyn Component
            },
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
        menu
    }
}

impl<'a> TopLevel for Menu<'a> {
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
