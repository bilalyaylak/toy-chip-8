use std::time::Duration;

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

pub struct Chip8 {
    ram: [u8; 4096],
    pc: u16,
    v: [u8; 16],
    i: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    pub vram: [bool; 64 * 32],
    vram_changed: bool,
    keys: [bool; 16],
    duration_until_next_execute: Duration,
}

pub struct Chip8TickResult {
    pub vram_changed: bool,
    pub beep: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            ram: [0; 4096],
            pc: 0x200,
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0_u8,
            delay_timer: 0,
            sound_timer: 0,
            vram: [false; 64 * 32],
            vram_changed: false,
            keys: [false; 16],
            //Helpers
            duration_until_next_execute: Duration::ZERO,
        };

        //Load font
        chip8.ram[0..FONT.len()].copy_from_slice(&FONT);

        chip8
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        if rom_data.len() > self.ram.len() - 0x200 {
            panic!("Memory size is not enough for the rom");
        }
        self.ram[0x200..0x200 + rom_data.len()].copy_from_slice(rom_data);
    }

    pub fn tick(&mut self, delta_time: Duration) -> Chip8TickResult {
        let mut vram_changed = false;
        let beep = false;

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
            // JP addr
            (0x1, _, _, _) => {
                self.pc = nnn;
                self.duration_until_next_execute = Duration::from_micros(109);
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
            // LD I, addr
            (0xA, _, _, _) => {
                self.i = nnn;
                self.duration_until_next_execute = Duration::from_micros(55);
            }
            // DRW Vx, Vy, nibble
            (0xD, _, _, _) => {
                let mut y_coord = self.v[y as usize] as usize % 32;
                self.v[0xF] = 0x0;
                for row in 0..n as usize {
                    if y_coord == 32 {
                        break;
                    }

                    let mut x_coord = self.v[x as usize] as usize % 64;
                    let row_data = self.ram[self.i as usize + row];

                    for pixel in 0..8usize {
                        if x_coord == 64 {
                            break;
                        }

                        let sprite_pixel = ((row_data >> (7 - pixel)) & 0x1) == 0x1;
                        let display_pixel = self.vram[y_coord * 64 + x_coord];

                        if sprite_pixel {
                            if display_pixel {
                                self.v[0xF] = 0x1;
                            }
                            self.vram[y_coord * 64 + x_coord] = !display_pixel;
                            self.vram_changed = true;
                        }
                        x_coord += 1;
                    }
                    y_coord += 1;
                }
                self.duration_until_next_execute = Duration::from_micros(22734);
            }
            (_, _, _, _) => panic!("Unexpected opcode"),
        }
    }
}
