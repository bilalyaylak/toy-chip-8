use std::{fs::File, io::Read, time::Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::chip8::Chip8;
use crate::display_device::DisplayDevice;

pub struct Platform {
    chip8: Chip8,
    display_device: DisplayDevice,
    event_pump: EventPump,
    rom_loaded: bool,
    previous_tick: Instant,
}

impl Platform {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        Platform {
            chip8: Chip8::new(),
            display_device: DisplayDevice::new(&sdl_context),
            event_pump: sdl_context.event_pump().unwrap(),
            rom_loaded: false,
            previous_tick: Instant::now(),
        }
    }

    pub fn load_rom(&mut self, rom_path: &String) {
        let mut rom_file = File::open(rom_path).expect("Couldn't find rom file");
        let mut rom_data = Vec::new();
        rom_file.read_to_end(&mut rom_data).unwrap();

        self.chip8.load_rom(&rom_data);
        self.rom_loaded = true;
    }

    pub fn start(&mut self) {
        if !self.rom_loaded {
            panic!("No rom loaded!");
        }

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
                    _ => (),
                }
            }
        }
    }
}
