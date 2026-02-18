#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[macro_use]
mod eadk;

mod constants;
mod game;
mod maps;

configure_app!(b"Metronome\0", 10, "../target/icon.nwi", 1654);

setup_allocator!();

#[unsafe(no_mangle)]
fn main() {
    init_heap!(); 

    game::test();

    crate::eadk::utils::wait_back_pressed();
}

