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

    use crate::game::game::Game;
    let mut game = Game::new();
    while !crate::eadk::utils::home_pressed() {
        game.update();
    }
}

