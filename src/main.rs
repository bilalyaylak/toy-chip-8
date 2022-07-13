mod chip8;
mod display_device;
mod platform;

use std::{env, process};

use platform::Platform;

struct Config {
    rom_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let rom_path = args[1].clone();

        Ok(Config { rom_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    let mut platform = Platform::new();

    platform.load_rom(&config.rom_path);
    platform.start();
}
