use std::time::Duration;

use rand::{prelude::ThreadRng, Rng};

const FONT: [u8; 80] = [
    //0
    0b1111_0000,
    0b1001_0000,
    0b1001_0000,
    0b1001_0000,
    0b1111_0000,
    //1
    0b0010_0000,
    0b0110_0000,
    0b0010_0000,
    0b0010_0000,
    0b0111_0000,
    //2
    0b1111_0000,
    0b0001_0000,
    0b1111_0000,
    0b1000_0000,
    0b1111_0000,
    //3
    0b1111_0000,
    0b0001_0000,
    0b1111_0000,
    0b0001_0000,
    0b1111_0000,
    //4
    0b1001_0000,
    0b1001_0000,
    0b1111_0000,
    0b0001_0000,
    0b0001_0000,
    //5
    0b1111_0000,
    0b1000_0000,
    0b1111_0000,
    0b0001_0000,
    0b1111_0000,
    //6
    0b1111_0000,
    0b1000_0000,
    0b1111_0000,
    0b1001_0000,
    0b1111_0000,
    //7
    0b1111_0000,
    0b0001_0000,
    0b0010_0000,
    0b0100_0000,
    0b0100_0000,
    //8
    0b1111_0000,
    0b1001_0000,
    0b1111_0000,
    0b1001_0000,
    0b1111_0000,
    //9
    0b1111_0000,
    0b1001_0000,
    0b1111_0000,
    0b0001_0000,
    0b1111_0000,
    //A
    0b1111_0000,
    0b1001_0000,
    0b1111_0000,
    0b1001_0000,
    0b1001_0000,
    //B
    0b1110_0000,
    0b1001_0000,
    0b1110_0000,
    0b1001_0000,
    0b1110_0000,
    //C
    0b1111_0000,
    0b1000_0000,
    0b1000_0000,
    0b1000_0000,
    0b1111_0000,
    //D
    0b1110_0000,
    0b1001_0000,
    0b1001_0000,
    0b1001_0000,
    0b1110_0000,
    //E
    0b1111_0000,
    0b1000_0000,
    0b1111_0000,
    0b1000_0000,
    0b1111_0000,
    //F
    0b1111_0000,
    0b1000_0000,
    0b1111_0000,
    0b1000_0000,
    0b1000_0000,
];

const MEMORY_SIZE: usize = 4096;
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub struct Chip8 {
    ram: [u8; MEMORY_SIZE],
    pc: u16,
    v: [u8; 16],
    i: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    pub vram: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    vram_changed: bool,
    keys: [bool; 16],
    duration_until_next_execute: Duration,
    duration_until_next_timer_interrupt: Duration,
    rng: ThreadRng,
}

pub struct Chip8TickResult {
    pub vram_changed: bool,
    pub beep: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            ram: [0; MEMORY_SIZE],
            pc: 0x200,
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            vram: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            vram_changed: false,
            keys: [false; 16],
            //Helpers
            duration_until_next_execute: Duration::ZERO,
            duration_until_next_timer_interrupt: Duration::ZERO,
            rng: rand::thread_rng(),
        };

        //Load font
        chip8.ram[0..FONT.len()].copy_from_slice(&FONT);

        chip8
    }

    pub fn change_key_state(&mut self, key: u8, pressed: bool) {
        debug_assert!(key <= 0xF);
        self.keys[key as usize] = pressed;
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        if rom_data.len() > self.ram.len() - 0x200 {
            panic!("Memory size is not enough for the rom");
        }
        self.ram[0x200..0x200 + rom_data.len()].copy_from_slice(rom_data);
    }

    pub fn tick(&mut self, delta_time: Duration) -> Chip8TickResult {
        let mut vram_changed = false;
        let mut beep = false;

        self.duration_until_next_execute =
            self.duration_until_next_execute.saturating_sub(delta_time);

        if self.duration_until_next_execute.is_zero() {
            let op = self.fetch();
            self.decode_and_execute(op);

            if self.vram_changed {
                vram_changed = true;
                self.vram_changed = false;
            }
        }

        self.duration_until_next_timer_interrupt = self
            .duration_until_next_timer_interrupt
            .saturating_sub(delta_time);

        if self.duration_until_next_timer_interrupt.is_zero() {
            self.duration_until_next_timer_interrupt = Duration::from_micros(16666);
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                beep = true;
                self.sound_timer -= 1;
            }
        }

        Chip8TickResult { vram_changed, beep }
    }

    fn fetch(&mut self) -> u16 {
        let first_byte = self.ram[self.pc as usize] as u16;
        let second_byte = self.ram[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        (first_byte << 8) | second_byte
    }

    fn decode_and_execute(&mut self, op: u16) {
        let first_nibble = ((op & 0xF000) >> 12) as u8;
        let x = ((op & 0x0F00) >> 8) as u8;
        let y = ((op & 0x00F0) >> 4) as u8;
        let n = (op & 0x000F) as u8;
        let nn = (op & 0x00FF) as u8;
        let nnn = (op & 0x0FFF) as u16;

        match (first_nibble, x, y, n) {
            // NOP
            (0x0, 0x0, 0x0, 0x0) => {
                self.duration_until_next_execute = Duration::ZERO;
            }
            // CLS
            (0x0, 0x0, 0xE, 0x0) => {
                self.vram.fill(false);
                self.vram_changed = true;
                self.duration_until_next_execute = Duration::from_micros(109);
            }
            // RET
            (0x0, 0x0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
                self.duration_until_next_execute = Duration::from_micros(105);
            }
            // JP addr
            (0x1, _, _, _) => {
                self.pc = nnn;
                self.duration_until_next_execute = Duration::from_micros(105);
            }
            // CALL addr
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
                self.duration_until_next_execute = Duration::from_micros(105);
            }
            // SE Vx, byte
            (0x3, _, _, _) => {
                if self.v[x as usize] == nn {
                    self.pc += 2;
                }
                self.duration_until_next_execute = Duration::from_micros(55);
            }
            // SNE Vx, byte
            (0x4, _, _, _) => {
                if self.v[x as usize] != nn {
                    self.pc += 2;
                }
                self.duration_until_next_execute = Duration::from_micros(55);
            }
            // SE Vx, Vy
            (0x5, _, _, 0x0) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
                self.duration_until_next_execute = Duration::from_micros(73);
            }
            // LD Vx, byte
            (0x6, _, _, _) => {
                self.v[x as usize] = nn;
                self.duration_until_next_execute = Duration::from_micros(27);
            }
            // ADD Vx, byte
            (0x7, _, _, _) => {
                self.v[x as usize] = self.v[x as usize].wrapping_add(nn);
                self.duration_until_next_execute = Duration::from_micros(45);
            }
            // LD Vx, Vy
            (0x8, _, _, 0x0) => {
                self.v[x as usize] = self.v[y as usize];
                self.duration_until_next_execute = Duration::from_micros(200);
            }
            // OR Vx, Vy
            (0x8, _, _, 0x1) => {
                self.v[x as usize] |= self.v[y as usize];
                self.v[0xF] = 0x0;
                self.duration_until_next_execute = Duration::from_micros(200);
            }
            // AND Vx, Vy
            (0x8, _, _, 0x2) => {
                self.v[x as usize] &= self.v[y as usize];
                self.v[0xF] = 0x0;
                self.duration_until_next_execute = Duration::from_micros(200);
            }
            // XOR Vx, Vy
            (0x8, _, _, 0x3) => {
                self.v[x as usize] ^= self.v[y as usize];
                self.v[0xF] = 0x0;
                self.duration_until_next_execute = Duration::from_micros(200);
            }
            // ADD Vx, Vy
            (0x8, _, _, 0x4) => {
                let (sum, carry) = self.v[x as usize].overflowing_add(self.v[y as usize]);
                self.v[x as usize] = sum;
                self.v[0xF] = if carry { 0x1 } else { 0x0 };
                self.duration_until_next_execute = Duration::from_micros(200);
            }
            // SUB Vx, Vy
            (0x8, _, _, 0x5) => {
                let (diff, borrow) = self.v[x as usize].overflowing_sub(self.v[y as usize]);
                self.v[x as usize] = diff;
                self.v[0xF] = if !borrow { 0x1 } else { 0x0 };
                self.duration_until_next_execute = Duration::from_micros(200);
            }
            // SHR Vx {, Vy}
            (0x8, _, _, 0x6) => {
                let vy = self.v[y as usize];
                self.v[x as usize] = vy >> 1;
                self.v[0xF] = vy & 0x1;
                self.duration_until_next_execute = Duration::from_micros(200);
            }
            // SUBN Vx, Vy
            (0x8, _, _, 0x7) => {
                let (diff, borrow) = self.v[y as usize].overflowing_sub(self.v[x as usize]);
                self.v[x as usize] = diff;
                self.v[0xF] = if !borrow { 0x1 } else { 0x0 };
                self.duration_until_next_execute = Duration::from_micros(200);
            }
            // SHL Vx {, Vy}
            (0x8, _, _, 0xE) => {
                let vy = self.v[y as usize];
                self.v[x as usize] = vy << 1;
                self.v[0xF] = (vy & 0x80) >> 7;
                self.duration_until_next_execute = Duration::from_micros(200);
            }
            // SNE Vx, Vy
            (0x9, _, _, 0x0) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
                self.duration_until_next_execute = Duration::from_micros(73);
            }
            // LD I, addr
            (0xA, _, _, _) => {
                self.i = nnn;
                self.duration_until_next_execute = Duration::from_micros(55);
            }
            // JP V0, addr
            (0xB, _, _, _) => {
                self.pc = nnn + self.v[0x0] as u16;
                self.duration_until_next_execute = Duration::from_micros(105);
            }
            // RND Vx, byte
            (0xC, _, _, _) => {
                self.v[x as usize] = self.rng.gen::<u8>() & nn;
                self.duration_until_next_execute = Duration::from_micros(164);
            }
            // DRW Vx, Vy, nibble
            (0xD, _, _, _) => {
                let mut y_coord = self.v[y as usize] as usize % DISPLAY_HEIGHT;
                self.v[0xF] = 0x0;
                for row in 0..n as usize {
                    if y_coord == DISPLAY_HEIGHT {
                        break;
                    }

                    let mut x_coord = self.v[x as usize] as usize % DISPLAY_WIDTH;
                    let row_data = self.ram[self.i as usize + row];

                    for pixel in 0..8usize {
                        if x_coord == DISPLAY_WIDTH {
                            break;
                        }

                        let sprite_pixel = ((row_data >> (7 - pixel)) & 0x1) == 0x1;
                        let display_pixel = self.vram[y_coord * DISPLAY_WIDTH + x_coord];

                        if sprite_pixel {
                            if display_pixel {
                                self.v[0xF] = 0x1;
                            }
                            self.vram[y_coord * DISPLAY_WIDTH + x_coord] = !display_pixel;
                            self.vram_changed = true;
                        }
                        x_coord += 1;
                    }
                    y_coord += 1;
                }
                self.duration_until_next_execute = Duration::from_micros(22734);
            }
            // SKP Vx
            (0xE, _, 0x9, 0xE) => {
                if self.keys[self.v[x as usize] as usize] {
                    self.pc += 2;
                }
                self.duration_until_next_execute = Duration::from_micros(73);
            }
            // SKNP Vx
            (0xE, _, 0xA, 0x1) => {
                if !self.keys[self.v[x as usize] as usize] {
                    self.pc += 2;
                }
                self.duration_until_next_execute = Duration::from_micros(73);
            }
            // LD Vx, DT
            (0xF, _, 0x0, 0x7) => {
                self.v[x as usize] = self.delay_timer;
                self.duration_until_next_execute = Duration::from_micros(45);
            }
            // LD Vx, K
            (0xF, _, 0x0, 0xA) => {
                if let Some(key) = self.keys.iter().position(|pressed| *pressed) {
                    self.v[x as usize] = key as u8;
                } else {
                    self.pc -= 2;
                }
                self.duration_until_next_execute = Duration::ZERO;
            }
            // LD DT, Vx
            (0xF, _, 0x1, 0x5) => {
                self.delay_timer = self.v[x as usize];
                self.duration_until_next_execute = Duration::from_micros(45);
            }
            // LD ST, Vx
            (0xF, _, 0x1, 0x8) => {
                self.sound_timer = self.v[x as usize];
                self.duration_until_next_execute = Duration::from_micros(45);
            }
            // ADD I, Vx
            (0xF, _, 0x1, 0xE) => {
                self.i += self.v[x as usize] as u16;
                if self.i >= 0x1000 {
                    self.v[0xF] = 0x1;
                    self.i &= 0x0FFF;
                } else {
                    self.v[0xF] = 0x0;
                }
                self.duration_until_next_execute = Duration::from_micros(86);
            }
            // LD F, Vx
            (0xF, _, 0x2, 0x9) => {
                self.i = (self.v[x as usize] & 0x0F) as u16 * 5;
                self.duration_until_next_execute = Duration::from_micros(91);
            }
            // LD B, Vx
            (0xF, _, 0x3, 0x3) => {
                let mut vx = self.v[x as usize];
                for digit in (0..=2usize).rev() {
                    self.ram[self.i as usize + digit] = vx % 10;
                    vx /= 10;
                }
                self.duration_until_next_execute = Duration::from_micros(927);
            }
            // LD [I], Vx
            // TODO: consider making this instruction configurable
            (0xF, _, 0x5, 0x5) => {
                for i in 0..=x as usize {
                    self.ram[self.i as usize] = self.v[i];
                    self.i += 1;
                }
                self.duration_until_next_execute = Duration::from_micros(605);
            }
            // LD Vx, [I]
            // TODO: consider making this instruction configurable
            (0xF, _, 0x6, 0x5) => {
                for i in 0..=x as usize {
                    self.v[i] = self.ram[self.i as usize];
                    self.i += 1;
                }
                self.duration_until_next_execute = Duration::from_micros(605);
            }

            (_, _, _, _) => panic!("Unexpected opcode"),
        }
    }
}
