#![no_std]
#![no_main]

pub mod eadk;
pub mod golf;
pub mod graphics;
pub mod math;

use ball::Ball;
use game::{Game, GameState};
use golf::*;
use graphics::Buffer;
use math::{Vec2, Vec2i};
use module::{EmptyModule, Module};
use scene::Scene;

#[used]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 5] = *b"Golf\0";

#[used]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

#[no_mangle]
pub fn main() {
    let mut buffer = Buffer::new();
    let mut empty_module = EmptyModule::new(Vec2i { x: 0, y: 0 });
    let mut modules: [&mut dyn Module; 1] = [&mut empty_module];
    let mut scene = Scene::new(
        &mut modules,
        Ball {
            pos: Vec2 { x: 0., y: 0. },
            height: 0.,
            velocity: Vec2 { x: 1., y: 2.0 },
        },
        3.,
    );
    let mut game = Game {
        state: GameState::InGame(&mut scene),
    };
    loop {
        game.update();
        game.render(&mut buffer);
        buffer.render();
    }
}
