#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[macro_use]
mod eadk;

mod constants;
mod map;
mod menu;
mod game;

configure_app!(b"metronome\0", 10, "../target/icon.nwi", 2659);

setup_allocator!();

#[unsafe(no_mangle)]
fn main() {
    init_heap!(); 

    let mut menu = menu::Menu::new();
    menu.main_loop();
}
