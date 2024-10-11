use crate::{eadk::Color, graphics::Buffer};

use super::scene::Scene;

pub enum GameState<'a, 'b, 'c> {
    InMenu,
    InGame(&'c mut Scene<'a, 'b>),
}

pub struct Game<'a, 'b, 'c> {
    pub state: GameState<'a, 'b, 'c>,
}

impl<'a, 'b, 'c> Game<'a, 'b, 'c> {
    pub fn update(&mut self) {
        match &mut self.state {
            GameState::InMenu => {
                //
            }
            GameState::InGame(scene) => {
                scene.update();
            }
        }
    }

    pub fn render(&self, buffer: &mut Buffer) {
        match &self.state {
            GameState::InMenu => {
                buffer.clear(Color { rgb565: 0x001F });
            }
            GameState::InGame(scene) => {
                scene.render(buffer);
            }
        }
    }
}
