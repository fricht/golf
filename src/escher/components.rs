use super::{Component, UiEvent, IDLE_COLOR, SELECT_COLOR};
use crate::{
    eadk::{
        display::{SCREEN_HEIGHT, SCREEN_WIDTH},
        Color, Rect,
    },
    graphics::Buffer,
    math::Vec2i,
};

pub enum MarginType {
    Margin(u16),
    Extend, // draw scree size
}

pub struct BoxContainer<'a> {
    pub child: &'a mut dyn Component,
    pub margin_top: MarginType,
    pub margin_bottom: MarginType,
    pub margin_left: MarginType,
    pub margin_right: MarginType,
}

impl<'a> BoxContainer<'a> {
    pub fn new(
        child: &'a mut dyn Component,
        margin_top: MarginType,
        margin_bottom: MarginType,
        margin_left: MarginType,
        margin_right: MarginType,
    ) -> Self {
        BoxContainer {
            child,
            margin_top,
            margin_bottom,
            margin_left,
            margin_right,
        }
    }
}

impl<'a> Component for BoxContainer<'a> {
    fn requested_size(&self) -> Vec2i {
        let child_size = self.child.requested_size();
        let x = match (&self.margin_left, &self.margin_right) {
            (MarginType::Margin(m1), MarginType::Margin(m2)) => {
                child_size.x + *m1 as i32 + *m2 as i32
            }
            _ => SCREEN_WIDTH as i32,
        };
        let y = match (&self.margin_top, &self.margin_bottom) {
            (MarginType::Margin(m1), MarginType::Margin(m2)) => {
                child_size.y + *m1 as i32 + *m2 as i32
            }
            _ => SCREEN_HEIGHT as i32,
        };
        Vec2i { x, y }
    }

    fn update(&mut self, signal: UiEvent) -> UiEvent {
        self.child.update(signal)
    }

    fn render(&self, buffer: &mut Buffer, offset: Vec2i) {
        let child_size = self.child.requested_size();
        let mut child_offset = Vec2i { x: 0, y: 0 };
        let size_x;
        match (&self.margin_left, &self.margin_right) {
            (MarginType::Margin(ml), MarginType::Margin(mr)) => {
                size_x = child_size.x as u16 + *ml + *mr;
                child_offset.x = *ml as i32;
            }
            (MarginType::Margin(ml), MarginType::Extend) => {
                size_x = SCREEN_WIDTH;
                child_offset.x = *ml as i32;
            }
            (MarginType::Extend, MarginType::Margin(mr)) => {
                size_x = SCREEN_WIDTH;
                child_offset.x = SCREEN_WIDTH as i32 - *mr as i32 - child_size.x;
            }
            (MarginType::Extend, MarginType::Extend) => {
                size_x = SCREEN_WIDTH;
                child_offset.x = (SCREEN_WIDTH as i32 - child_size.x) / 2;
            }
        }
        let size_y;
        match (&self.margin_top, &self.margin_bottom) {
            (MarginType::Margin(mt), MarginType::Margin(mb)) => {
                size_y = child_size.y as u16 + *mt + *mb;
                child_offset.y = *mt as i32;
            }
            (MarginType::Margin(mt), MarginType::Extend) => {
                size_y = SCREEN_WIDTH;
                child_offset.y = *mt as i32;
            }
            (MarginType::Extend, MarginType::Margin(mb)) => {
                size_y = SCREEN_WIDTH;
                child_offset.y = SCREEN_WIDTH as i32 - *mb as i32 - child_size.y;
            }
            (MarginType::Extend, MarginType::Extend) => {
                size_y = SCREEN_WIDTH;
                child_offset.y = (SCREEN_WIDTH as i32 - child_size.y) / 2;
            }
        }
        buffer.push_rect_uniform(
            Rect::screen_space_culling(offset.x, offset.y, size_x as i32, size_y as i32),
            Color {
                rgb565: if self.child.get_selected() {
                    SELECT_COLOR
                } else {
                    IDLE_COLOR
                },
            },
        );
        self.child.render(buffer, offset + child_offset);
    }

    fn set_selected(&mut self, selected: bool) {
        // forward select state
        self.child.set_selected(selected);
    }

    fn get_selected(&self) -> bool {
        // forward select state
        self.child.get_selected()
    }
}
