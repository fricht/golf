#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;

pub mod eadk;
pub mod escher;
pub mod golf;
pub mod graphics;
pub mod math;

use alloc::boxed::Box;
use core::str;
use eadk::{_heap_end, _heap_start, int_to_str};
use embedded_alloc::TlsfHeap as Heap;
use escher::text::draw_debug_text;
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
pub static EADK_APP_ICON: [u8; 4250] = [0; 4250]; // *include_bytes!("../target/icon.nwi");

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[no_mangle]
pub fn main() {
    // initialize the memory allocator
    unsafe {
        let heap_size = _heap_end - _heap_start;
        {
            // display heap info
            eadk::display::push_rect_uniform(
                eadk::Rect {
                    x: 0,
                    y: 0,
                    width: 320,
                    height: 240,
                },
                eadk::Color { rgb565: 0x0 },
            );
            let (mut bytes_start, mut bytes_end, mut bytes_size) =
                ([0x20; 20], [0x20; 20], [0x20; 20]);
            int_to_str(_heap_start, &mut bytes_start);
            int_to_str(_heap_end, &mut bytes_end);
            int_to_str(heap_size, &mut bytes_size);
            draw_debug_text("start :", (1, 1));
            draw_debug_text(str::from_utf8(&bytes_start).unwrap(), (49, 1));
            draw_debug_text("end :", (1, 12));
            draw_debug_text(str::from_utf8(&bytes_end).unwrap(), (49, 12));
            draw_debug_text("size :", (1, 23));
            draw_debug_text(str::from_utf8(&bytes_size).unwrap(), (49, 23));
            eadk::timing::msleep(5000);
        }
        HEAP.init(_heap_start, heap_size);
    }

    let mut buffer = Buffer::new();

    let empty_module1 = EmptyModule::new(Vec2i { x: 0, y: 0 });
    let empty_module2 = EmptyModule::new(Vec2i {
        x: 2 * TILE_SIZE,
        y: 0,
    });
    let empty_module3 = EmptyModule::new(Vec2i {
        x: 0,
        y: 2 * TILE_SIZE,
    });
    let empty_module4 = EmptyModule::new(Vec2i {
        x: 2 * TILE_SIZE,
        y: 2 * TILE_SIZE,
    });
    let modules = vec![
        Box::new(empty_module1) as Box<dyn Module>,
        Box::new(empty_module2),
        Box::new(empty_module3),
        Box::new(empty_module4),
    ];
    let scene = Scene::new(modules, Vec2 { x: 1., y: 1. }, 3.);

    let mut game = Game {
        state: GameState::InGame(scene),
    };

    // game loop
    loop {
        game.update();
        game.render(&mut buffer);
        buffer.render();
    }
}
