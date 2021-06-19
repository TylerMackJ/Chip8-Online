mod utils;

use wasm_bindgen::prelude::*;
use rand::Rng;
use std::fmt;

const DISPLAY_WIDTH: u8 = 64;
const DISPLAY_HEIGHT: u8 = 32;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Chip8 {
    memory: [u8; 0x1000],
    v: [u8; 0x10],
    i: u16,
    dt: u8,
    st: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 0x10], 
    display: Vec<bool>,
    keypad: [bool; 0x10],
}

#[wasm_bindgen]
impl Chip8 {
    pub fn new() -> Chip8 {
        utils::set_panic_hook();

        let display = (0..DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize).map(|_| {
            false
        }).collect();

        // Sprites
        let mut memory = [0u8; 0x1000];
        // 0
        memory[0] = 0xF0;
        memory[1] = 0x90;
        memory[2] = 0x90;
        memory[3] = 0x90;
        memory[4] = 0xF0;

        // 1
        memory[5] = 0x20;
        memory[6] = 0x60;
        memory[7] = 0x20;
        memory[8] = 0x20;
        memory[9] = 0x70;

        // 2
        memory[10] = 0xF0;
        memory[11] = 0x10;
        memory[12] = 0xF0;
        memory[13] = 0x80;
        memory[14] = 0xF0;

        // 3
        memory[15] = 0xF0;
        memory[16] = 0x10;
        memory[17] = 0xF0;
        memory[18] = 0x10;
        memory[19] = 0xF0;

        // 4
        memory[20] = 0x90;
        memory[21] = 0x90;
        memory[22] = 0xF0;
        memory[23] = 0x10;
        memory[24] = 0x10;

        // 5
        memory[25] = 0xF0;
        memory[26] = 0x80;
        memory[27] = 0xF0;
        memory[28] = 0x10;
        memory[29] = 0xF0;

        // 6
        memory[30] = 0xF0;
        memory[31] = 0x80;
        memory[32] = 0xF0;
        memory[33] = 0x90;
        memory[34] = 0xF0;

        // 7
        memory[35] = 0xF0;
        memory[36] = 0x10;
        memory[37] = 0x20;
        memory[38] = 0x40;
        memory[39] = 0x40;

        // 8
        memory[40] = 0xF0;
        memory[41] = 0x90;
        memory[42] = 0xF0;
        memory[43] = 0x90;
        memory[44] = 0xF0;

        // 9
        memory[45] = 0xF0;
        memory[46] = 0x90;
        memory[47] = 0xF0;
        memory[48] = 0x10;
        memory[49] = 0xF0;

        // A
        memory[50] = 0xF0;
        memory[51] = 0x90;
        memory[52] = 0xF0;
        memory[53] = 0x90;
        memory[54] = 0x90;

        // B
        memory[55] = 0xE0;
        memory[56] = 0x90;
        memory[57] = 0xE0;
        memory[58] = 0x90;
        memory[59] = 0xE0;

        // C
        memory[60] = 0xF0;
        memory[61] = 0x80;
        memory[62] = 0x80;
        memory[63] = 0x80;
        memory[64] = 0xF0;

        // D
        memory[65] = 0xE0;
        memory[66] = 0x90;
        memory[67] = 0x90;
        memory[68] = 0x90;
        memory[69] = 0xE0;

        // E
        memory[70] = 0xF0;
        memory[71] = 0x80;
        memory[72] = 0xF0;
        memory[73] = 0x80;
        memory[74] = 0xF0;

        // F
        memory[75] = 0xF0;
        memory[76] = 0x80;
        memory[77] = 0xF0;
        memory[78] = 0x80;
        memory[79] = 0x80;

        Chip8 {
            memory: memory,
            v: [0u8; 0x10],
            i: 0,
            dt: 0,
            st: 0,
            pc: 0x200,
            sp: 0,
            stack: [0u16; 0x10],
            display: display,
            keypad: [false; 0x10],
        }
    }

    pub fn key_pressed(&mut self, key: u8, pressed: bool) {
        self.keypad[key as usize] = pressed;
    }

    pub fn width(&self) -> u8 {
        DISPLAY_WIDTH
    }

    pub fn height(&self) -> u8 {
        DISPLAY_HEIGHT
    }

    pub fn display(&self) -> *const bool {
        self.display.as_ptr()
    }

    pub fn handle_timers(&mut self) -> bool {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
        self.st > 0
    }

    pub fn load_rom(&mut self, rom: String) {
        *self = Chip8::new();
        rom.chars().enumerate().for_each(|(i, c)| {
            self.memory[0x200 + i] = c as u8;
        });
    }

    fn get_instruction(&mut self) -> u16 {
        self.pc += 2;
        ((self.memory[self.pc as usize - 2] as u16) << 8) | self.memory[self.pc as usize - 1] as u16
    }

    fn get_display_index(&self, x: u8, y: u8) -> usize {
        (x as usize % DISPLAY_WIDTH as usize) + ((y as usize % DISPLAY_HEIGHT as usize) * DISPLAY_WIDTH as usize)
    }

    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();
        let instruction: u16 = self.get_instruction();
        let x: u8    = ((instruction & 0x0F00) >> 8) as u8;
        let y: u8    = ((instruction & 0x00F0) >> 4) as u8;
        let n: u8    = ( instruction & 0x000F      ) as u8;
        let kk: u8   = ( instruction & 0x00FF      ) as u8;
        let nnn: u16 =  instruction & 0x0FFF;

        match instruction & 0xF000 {
            0x0000 => {
                match instruction & 0x00FF {
                    0x00E0 => {
                        let display = (0..DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize).map(|_| {
                            false
                        }).collect();
                        self.display = display;
                    },
                    0x00EE => {
                        self.pc = self.stack[self.sp as usize];
                        self.sp -= 1;
                    },
                    _ => panic!("Instruction unknown {}", instruction),
                }
            },
            0x1000 => self.pc = nnn,
            0x2000 => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = nnn;
            },
            0x3000 => {
                if self.v[x as usize] == kk {
                    self.pc += 2;
                }
            },
            0x4000 => {
                if self.v[x as usize] != kk {
                    self.pc += 2;
                }
            },
            0x5000 => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            },
            0x6000 => self.v[x as usize] = kk,
            0x7000 => self.v[x as usize] += kk,
            0x8000 => {
                match instruction & 0x000F {
                    0x0000 => self.v[x as usize]  = self.v[y as usize],
                    0x0001 => self.v[x as usize] |= self.v[y as usize],
                    0x0002 => self.v[x as usize] &= self.v[y as usize],
                    0x0003 => self.v[x as usize] ^= self.v[y as usize],
                    0x0004 => {
                        let result = self.v[x as usize].overflowing_add(self.v[y as usize]);
                        self.v[x as usize] = result.0;
                        self.v[0xF] = if result.1 { 1 } else { 0 };
                    },
                    0x0005 => {
                        let result = self.v[x as usize].overflowing_sub(self.v[y as usize]);
                        self.v[x as usize] = result.0;
                        self.v[0xF] = if result.1 { 1 } else { 0 };
                    },
                    0x0006 => {
                        self.v[0xF] = if self.v[x as usize] & 0x01 == 0x01 { 1 } else { 0 };
                        self.v[x as usize] *= 2;
                    },
                    0x0007 => {
                        let result = self.v[y as usize].overflowing_sub(self.v[x as usize]);
                        self.v[x as usize] = result.0;
                        self.v[0xF] = if result.1 { 1 } else { 0 };
                    },
                    0x000E => {
                        self.v[0xF] = if self.v[x as usize] & 0x80 == 0x80 { 1 } else { 0 };
                        self.v[x as usize] *= 2;
                    },
                    _ => panic!("Instruction unknown {}", instruction),
                }
            },
            0x9000 => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            },
            0xA000 => self.i = nnn,
            0xB000 => self.pc = self.v[0] as u16 + nnn,
            0xC000 => self.v[x as usize] = rng.gen::<u8>() & kk,
            0xD000 => {
                let mut collision: bool = false;
                for row in 0..n {
                    let row_byte: u8 = self.memory[self.i as usize + row as usize];
                    for col in 0..8 {
                        let col_bit: bool = (row_byte >> (7 - col)) & 0x01 == 0x01;

                        if col_bit {
                            let display_index: usize = self.get_display_index(self.v[x as usize] + col, self.v[y as usize] + row);
                            if self.display[display_index] == true {
                                collision = true;
                            }
                            self.display[display_index] = !self.display[display_index];
                        }
                    }
                }
                self.v[0xF] = if collision { 1 } else { 0 };
            },
            0xE000 => {
                match instruction & 0x00FF {
                    // Keyboard
                    0x009E => {
                        if self.keypad[self.v[x as usize] as usize] {
                            self.pc += 2;
                        }
                    },
                    0x00A1 => {
                        if !self.keypad[self.v[x as usize] as usize] {
                            self.pc += 2;
                        }
                    },
                    _ => panic!("Instruction unknown {}", instruction),
                }
            },
            0xF000 => {
                match instruction & 0x00FF {
                    0x0007 => self.v[x as usize] = self.dt,
                    0x000A => {
                        self.pc -= 2;
                        for (i, &key) in self.keypad.iter().enumerate() {
                            if key {
                                self.v[x as usize] = i as u8;
                                self.pc += 2;
                                break;
                            }
                        }
                    },
                    0x0015 => self.dt = self.v[x as usize],
                    0x0018 => self.st = self.v[x as usize],
                    0x001E => self.i += self.v[x as usize] as u16,
                    0x0029 => self.i  = self.v[x as usize] as u16 * 5,
                    0x0033 => {
                        self.memory[self.i as usize    ] = (self.v[x as usize]      ) / 100;
                        self.memory[self.i as usize + 1] = (self.v[x as usize] % 100) / 10;
                        self.memory[self.i as usize + 2] =  self.v[x as usize] % 10 ;
                    },
                    0x0055 => {
                        for offset in 0..0x10 {
                            self.memory[self.i as usize + offset] = self.v[offset];
                        }
                    },
                    0x0065 => {
                        for offset in 0..0x10 {
                            self.v[offset] = self.memory[self.i as usize + offset];
                        }
                    },
                    _ => panic!("Instruction unknown {}", instruction),
                }
            },
            _ => panic!("Instruction unknown {}", instruction),
        }
    }
}

impl fmt::Display for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.display.as_slice().chunks(DISPLAY_WIDTH as usize) {
            for &pixel in line {
                let symbol = if pixel { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}