use super::{scene::Scene, ui::Menu};
use crate::{eadk::Color, escher::TopLevel, graphics::Buffer};

pub enum GameState<'a, 'b, 'c> {
    InMenu(Menu<'a>),
    InGame(Scene<'b, 'c>),
}

pub struct Game<'a, 'b, 'c> {
    pub state: GameState<'a, 'b, 'c>,
}

impl<'a, 'b, 'c> Game<'a, 'b, 'c> {
    pub fn update(&mut self) {
        match &mut self.state {
            GameState::InMenu(menu) => {
                menu.update();
            }
            GameState::InGame(scene) => {
                scene.update();
            }
        }
    }

    pub fn render(&self, buffer: &mut Buffer) {
        match &self.state {
            GameState::InMenu(menu) => {
                buffer.clear(Color { rgb565: 0x001F });
                // menu.render(buffer);
            }
            GameState::InGame(scene) => {
                scene.render(buffer);
            }
        }
    }
}
