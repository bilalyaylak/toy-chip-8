mod chip8;
mod display_device;
mod platform;

use platform::Platform;

fn main() {
    let mut platform = Platform::new();

    platform.load_rom(&String::from("roms/IBM Logo.ch8"));
    platform.start();
}
