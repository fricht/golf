#![no_std]
#![no_main]

pub mod eadk;
pub mod escher;
pub mod golf;
pub mod graphics;
pub mod math;

use game::{Game, GameState};
use golf::*;
use graphics::Buffer;
use math::{Vec2, Vec2i};
use module::{EmptyModule, Module, TILE_SIZE};
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
    let mut empty_module1 = EmptyModule::new(Vec2i { x: 0, y: 0 });
    let mut empty_module2 = EmptyModule::new(Vec2i {
        x: 2 * TILE_SIZE,
        y: 0,
    });
    let mut empty_module3 = EmptyModule::new(Vec2i {
        x: 0,
        y: 2 * TILE_SIZE,
    });
    let mut empty_module4 = EmptyModule::new(Vec2i {
        x: 2 * TILE_SIZE,
        y: 2 * TILE_SIZE,
    });
    let mut modules = [
        &mut empty_module1 as &mut dyn Module,
        &mut empty_module2 as &mut dyn Module,
        &mut empty_module3 as &mut dyn Module,
        &mut empty_module4 as &mut dyn Module,
    ];
    let mut scene = Scene::new(&mut modules, Vec2 { x: 1., y: 1. }, 3.);
    let mut game = Game {
        state: GameState::InGame(&mut scene),
    };
    loop {
        game.update();
        game.render(&mut buffer);
        buffer.render();
    }
}
