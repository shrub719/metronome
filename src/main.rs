#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[macro_use]
mod eadk;

configure_app!(b"Metronome\0", 10, "../target/icon.nwi", 1654);

setup_allocator!();

#[unsafe(no_mangle)]
fn main() {
    init_heap!(); 

    crate::eadk::keyboard::wait_until_pressed(eadk::keyboard::Key::Ok);
}
