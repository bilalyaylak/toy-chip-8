mod chip8;
mod config;
mod display_device;
mod platform;

use clap::Parser;
use config::Config;
use platform::Platform;

fn main() {
    let config = Config::parse();
    let mut platform = Platform::new(config);

    platform.start();
}
