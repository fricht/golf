#![no_std]

extern crate alloc;

pub mod game;
mod menu;
pub mod utils;

use alloc::boxed::Box;
use graphmgr::StateManager;
use menu::MainMenuState;

/// How to handle the end of the main app.
pub enum ExitBehaviour {
    Exit,
    Hang,
    Restart,
}

pub enum PopMessage {
    OkBackPopupIsOk(bool),
    None,
}

/// The core of the application logic
pub fn main() -> ExitBehaviour {
    let mut state_mgr = StateManager::<PopMessage>::new();
    state_mgr.run(Box::new(MainMenuState), 60);
    ExitBehaviour::Exit
}
