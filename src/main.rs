#![no_std]
#![no_main]

pub mod eadk;
pub mod golf;
pub mod graphics;
pub mod math;

use ball::Ball;
use camera::Camera;
use golf::*;
use graphics::Buffer;
use math::Vec2;
use module::Module;

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
    let modules = [Module::new()];
    let camera = Camera::new(
        &modules,
        Ball {
            pos: Vec2 { x: 0., y: 0. },
            height: 0.,
        },
    );

    loop {
        camera.render(&mut buffer);
        buffer.render();
    }
}
