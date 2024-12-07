use super::{scene::Scene, ui::Menu};
use crate::{eadk::Color, escher::TopLevel, graphics::Buffer};

pub enum GameState<'a, 'b> {
    InMenu(Menu),
    InGame(Scene<'a, 'b>),
}

pub struct Game<'a, 'b> {
    pub state: GameState<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
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
