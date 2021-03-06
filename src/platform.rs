use std::{fs::File, io::Read, time::Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::chip8::Chip8;
use crate::config::Config;
use crate::display_device::DisplayDevice;

pub struct Platform {
    chip8: Chip8,
    display_device: DisplayDevice,
    event_pump: EventPump,
    previous_tick: Instant,
}

impl Platform {
    pub fn new(config: Config) -> Self {
        let sdl_context = sdl2::init().unwrap();

        let mut platform = Self {
            chip8: Chip8::new(),
            display_device: DisplayDevice::new(
                &sdl_context,
                config.background_color,
                config.sprite_color,
            ),
            event_pump: sdl_context.event_pump().unwrap(),
            previous_tick: Instant::now(),
        };
        platform.load_rom(&config.rom_path);
        platform
    }

    pub fn load_rom(&mut self, rom_path: &String) {
        let mut rom_file = File::open(rom_path).expect("Couldn't find rom file");
        let mut rom_data = Vec::new();
        rom_file.read_to_end(&mut rom_data).unwrap();

        self.chip8.load_rom(&rom_data);
    }

    pub fn start(&mut self) {
        self.previous_tick = Instant::now();

        'mainloop: loop {
            let delta_time = self.previous_tick.elapsed();
            self.previous_tick = Instant::now();

            let tick_result = self.chip8.tick(delta_time);

            if tick_result.vram_changed {
                self.display_device.draw(&self.chip8.vram);
            }

            for evt in self.event_pump.poll_iter() {
                match evt {
                    Event::Quit { .. } => {
                        break 'mainloop;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'mainloop;
                    }
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        if let Some(chip8_key) = Self::get_chip8_key(key) {
                            self.chip8.change_key_state(chip8_key, true);
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => {
                        if let Some(chip8_key) = Self::get_chip8_key(key) {
                            self.chip8.change_key_state(chip8_key, false);
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn get_chip8_key(key: Keycode) -> Option<u8> {
        match key {
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0xC),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xD),
            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xE),
            Keycode::Z => Some(0xA),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xB),
            Keycode::V => Some(0xF),
            _ => None,
        }
    }
}
