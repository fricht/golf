#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod eadk;
pub mod escher;
pub mod golf;
pub mod graphics;
pub mod math;

use embedded_alloc::TlsfHeap as Heap;
use game::{Game, GameState};
use golf::*;
use graphics::Buffer;
use math::{Vec2, Vec2i};
use module::{EmptyModule, Module, TILE_SIZE};
use scene::Scene;
use ui::Menu;

#[used]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 5] = *b"Golf\0";

#[used]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

#[global_allocator]
static HEAP: Heap = Heap::empty();

// what am i doing ...
#[no_mangle]
pub extern "C" fn _critical_section_1_0_acquire() {
    // Do nothing if you don't need concurrency protection
}

#[no_mangle]
pub extern "C" fn _critical_section_1_0_release() {
    // Do nothing if you don't need concurrency protection
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    // You can define custom logic here, like logging the error or resetting the device.
    panic!("allocation error: {:?}", layout);
}

#[no_mangle]
pub fn main() {
    // initialize the memory allocator
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe {
            HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE);
        }
    }

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

    let menu = Menu::new();

    let mut game = Game {
        state: GameState::InMenu(menu),
    };

    // game loop
    loop {
        game.update();
        game.render(&mut buffer);
        buffer.render();
    }
}
