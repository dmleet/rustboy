use crate::alu::*;
use crate::registers::*;
use crate::mmu::*;

use log::debug;

trait SignedAdd {
    fn signed_add(self, rhs: i8) -> Self;
}

impl SignedAdd for u16 {
    fn signed_add(self, rhs: i8) -> u16 {
        ((self as i32) + (rhs as i32)) as u16
    }
}

pub struct Cpu {
    pub reg: Registers,
    ime_delay: u8,
    ime_set: Option<bool>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg: Registers::new(),
            ime_delay: 0,
            ime_set: None,
        }
    }

    // Returns tick length in m-cycles
    pub fn tick(&mut self, mem: &mut Memory) -> u16 {
        match self.interrupt(mem) {
            Some(cycles) => cycles,
            None => {
                self.call_instruction(mem)
            }
        }
    }

    fn interrupt(&mut self, mem: &mut Memory) -> Option<u16> {
        let if_flag = mem[0xFF0F];
        let ie_flag = mem[0xFFFF];

        self.ime_delay = match self.ime_delay {
            2 => 1,
            1 => {
                match self.ime_set {
                    Some(true) => {
                        self.reg.ime = true;
                        0
                    },
                    Some(false) => {
                        self.reg.ime = false;
                        0
                    },
                    _ => 0
                }
            },
            _ => 0,
        };

        if self.reg.ime && if_flag & ie_flag != 0 {
            self.reg.ime = false;
            match if_flag { // check if an interrupt is pending
                0x00 => {}, // no interrupt pending
                0x01 => { // VBlank interrupt
                    self.push_stack(self.reg.pc, mem);
                    self.reg.pc = 0x0040;
                    debug!("VBlank interrupt!");
                    return Some(4);
                },
                _ => {
                    panic!("Unhandled interrupt flag: 0x{:X}", mem[0xFF0F]);
                }
            }
        }

        return None;
    }

    // Get next byte from memory and increment program counter
    fn next_byte(&mut self, mem: &mut [u8]) -> u8 {
        let byte = read_byte(self.reg.pc, mem);
        self.reg.pc += 1;
        byte
    }
    
    // Get next word from memory and increment program counter
    fn next_word(&mut self, mem: &[u8]) -> u16 {
        let word = read_word(self.reg.pc, mem);
        self.reg.pc += 2;
        word
    }
    
    fn push_stack(&mut self, val: u16, mem: &mut Memory) {
        self.reg.sp -= 2;
        write_word(self.reg.sp, val, mem);
        debug!("PUSH: {:#04x}", val);
    }
    
    fn pop_stack(&mut self, mem: &mut Memory) -> u16 {
        let val = read_word(self.reg.sp, mem);
        self.reg.sp += 2;
        debug!("POP: {:#04x}", val);
        val
    }

    fn set_ime(&mut self, val: bool) {
        self.ime_set = Some(val);
        self.ime_delay = 2;
    }

    // Cpu instruction set
    // Returns m-cycle length of instruction
    fn call_instruction(&mut self, mem: &mut Memory) -> u16 {
        let opcode = self.next_byte(mem);
        debug!("Last opcode: {:02X}", opcode);
    
        match opcode {
    
            // NOP
            0x00 => { 1 },
    
            // LD BC, d16
            0x01 => {
                let val = self.next_word(mem);
                self.reg.set_bc(val);
                3
            },
    
            // LD (BC), A
            0x02 => {
                write_byte(self.reg.bc(), self.reg.a, mem);
                2
            },
    
            // INC BC
            0x03 => {
                self.reg.set_bc(self.reg.bc().wrapping_add(1));
                2
            },
    
            // INC B
            0x04 => {
                let n = self.reg.b;
                self.reg.b = alu_inc(&mut self.reg, n);
                1
            },
    
            // DEC B
            0x05 => {
                let n = self.reg.b;
                self.reg.b = alu_dec(&mut self.reg, n);
                1
            },
    
            // LD B, d8
            0x06 => {
                self.reg.b = self.next_byte(mem);
                2
            },
    
            // RLCA
            0x07 => {
                let c = self.reg.a >> 7;
                self.reg.a = (self.reg.a << 1) | c;
                self.reg.set_flag(Flag::Z, false);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c == 1);
                1
            },
    
            // LD (a16), SP
            0x08 => {
                let adr = self.next_word(mem);
                write_word(adr, self.reg.sp, mem);
                5
            },

            // ADD HL, BC
            0x09 => {
                let n = self.reg.bc();
                alu_add_hl(&mut self.reg, n);
                2
            }
    
            // LD A, (BC)
            0x0A => {
                self.reg.a = read_byte(self.reg.bc(), mem);
                2
            },
    
            // DEC BC
            0x0B => {
                self.reg.set_bc(self.reg.bc().wrapping_sub(1));
                2
            },
    
            // DEC C
            0x0D => {
                let n = self.reg.c;
                self.reg.c = alu_dec(&mut self.reg, n);
                1
            },
    
            // LD C, d8
            0x0E => {
                self.reg.c = self.next_byte(mem);
                2
            },
    
            // STOP
            0x10 => { 1 },
    
            // LD DE, d16
            0x11 => {
                let val = self.next_word(mem);
                self.reg.set_de(val);
                3
            },

            // LD (DE), A
            0x12 => {
                let adr = self.reg.de();
                let val = self.reg.a;
                write_byte(adr, val, mem);
                2
            },
    
            // INC D
            0x14 => {
                let n = self.reg.d;
                self.reg.d = alu_inc(&mut self.reg, n);
                1
            }
    
            // DEC D
            0x15 => {
                let n = self.reg.d;
                self.reg.d = alu_dec(&mut self.reg, n);
                1
            },
    
            // LD D, d8
            0x16 => {
                self.reg.d = self.next_byte(mem);
                2
            },
    
            // JR r8
            0x18 => {
                let n = self.next_byte(mem) as i8;
                self.reg.pc = ((self.reg.pc as i32) + (n as i32)) as u16;
                3
            },
    
            // INC C
            0x0C => {
                let n = self.reg.c;
                self.reg.c = alu_inc(&mut self.reg, n);
                1
            },

            // INC DE
            0x13 => {
                self.reg.set_de(self.reg.de().wrapping_add(1));
                2
            },
    
            // ADD HL, DE
            0x19 => {
                let n = self.reg.de();
                alu_add_hl(&mut self.reg, n);
                2
            },

            // LD A, (DE)
            0x1A => {
                let adr = self.reg.de();
                self.reg.a = read_byte(adr, mem);
                2
            },
    
            // DEC E
            0x1D => {
                let n = self.reg.e;
                self.reg.e = alu_dec(&mut self.reg, n);
                1
            },
    
            // LD E, d8
            0x1E => {
                self.reg.e = self.next_byte(mem);
                2
            },
    
            // RRA
            0x1F => {
                let c = self.reg.get_flag(Flag::C) as u8;
                self.reg.set_flag(Flag::Z, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::C, (self.reg.a & 0x01) == 0x01);
                self.reg.a = (self.reg.a >> 1) | (c << 7);
                1
            },
    
            // JR NZ, r8
            0x20 => {
                if !self.reg.get_flag(Flag::Z) {
                    let n = self.next_byte(mem) as i8;
                    self.reg.pc = self.reg.pc.signed_add(n);
                    return 3;
                } else {
                    self.reg.pc += 1;
                    2
                }
            },
    
            // LD HL, d16
            0x21 => {
                let word = self.next_word(mem);
                self.reg.set_hl(word);
                3
            },

            // LD (HL+), A
            0x22 => {
                let adr = self.reg.hl();
                write_byte(adr, self.reg.a, mem);
                self.reg.set_hl(self.reg.hl().wrapping_add(1));
                2
            },
    
            // INC HL
            0x23 => {
                self.reg.set_hl(self.reg.hl().wrapping_add(1));
                2
            }
    
            // DEC H
            0x25 => {
                let n = self.reg.h;
                self.reg.h = alu_dec(&mut self.reg, n);
                1
            }

            // JR Z, r8
            0x28 => {
                if self.reg.get_flag(Flag::Z) {
                    let n = self.next_byte(mem) as i8;
                    self.reg.pc = self.reg.pc.signed_add(n);
                    return 3;
                } else {
                    self.reg.pc += 1;
                    2
                }
            },
    
            // LD A, (HL+)
            0x2A => {
                self.reg.a = read_byte(self.reg.hl(), mem);
                self.reg.set_hl(self.reg.hl().wrapping_add(1));
                2
            },
    
            // INC L
            0x2C => {
                let n = self.reg.l;
                self.reg.l = alu_inc(&mut self.reg, n);
                1
            },
    
            // ADD HL, HL
            0x29 => {
                let n = self.reg.hl();
                alu_add_hl(&mut self.reg, n);
                2
            },
    
            // CPL
            0x2F => {
                self.reg.a = !self.reg.a;
                self.reg.set_flag(Flag::N, true);
                self.reg.set_flag(Flag::H, true);
                1
            },
    
            // LD SP, d16
            0x31 => {
                self.reg.sp = self.next_word(mem);
                3
            },
    
            // LD (HL-), A
            0x32 => { 
                write_byte(self.reg.hl(), self.reg.a, mem);
                self.reg.set_hl(self.reg.hl().wrapping_sub(1));
                2
            },

            // INC (HL)
            0x34 => {
                let adr = self.reg.hl();
                let val = read_byte(adr, mem);
                write_byte(adr + 0x0001, val, mem);
                3
            }
    
            //LD (HL), d8
            0x36 => {
                let n = self.next_byte(mem);
                alu_inc(&mut self.reg, n);
                2
            },
    
            // JR C, r8
            0x38 => {
                if self.reg.get_flag(Flag::C) {
                    let n = self.next_byte(mem) as i8;
                    self.reg.pc = ((self.reg.pc as i32) + (n as i32)) as u16;
                }
                2
            },

            // INC A
            0x3C => {
                let n = self.reg.a;
                self.reg.a = alu_inc(&mut self.reg, n);
                1
            },

            // DEC A
            0x3D => {
                let n = self.reg.a;
                self.reg.a = alu_dec(&mut self.reg, n);
                1
            },
    
            // LD A, d8
            0x3E => {
                self.reg.a = self.next_byte(mem);
                2
            },
    
            // LD B, C
            0x41 => {
                self.reg.b = self.reg.c;
                1
            },
    
            // LD B, A
            0x47 => {
                self.reg.b = self.reg.a;
                1
            },
    
            // LD C, A
            0x4F => {
                self.reg.c = self.reg.a;
                1
            },

            // LD D, (HL)
            0x56 => {
                self.reg.d = read_byte(self.reg.hl(), mem);
                2
            },

            // LD E, (HL)
            0x5E => { 
                self.reg.e = read_byte(self.reg.hl(), mem);
                2
            },

            // LD E, A
            0x5F => {
                self.reg.e = self.reg.a;
                1
            },
    
            // LD H, A
            0x67 => {
                self.reg.h = self.reg.a;
                1
            },

            // LD L, A
            0x6F => {
                self.reg.l = self.reg.a;
                1
            },
    
            // LD (HL), A
            0x77 => {
                write_byte(self.reg.hl(), self.reg.a, mem);
                2
            },
    
            // LD A, B
            0x78 => {
                self.reg.a = self.reg.b;
                1
            },
    
            // LD A, C
            0x79 => {
                self.reg.a = self.reg.c;
                1
            },
    
            // LD A, E
            0x7B => {
                self.reg.a = self.reg.e;
                1
            },
    
            // LD A, H
            0x7C => {
                self.reg.a = self.reg.h;
                1
            },

            // LD A, L
            0x7D => {
                self.reg.a = self.reg.l;
                1
            },

            // LD A, (HL)
            0x7E => {
                let adr = self.reg.hl();
                self.reg.a = read_byte(adr, mem);
                2
            },
    
            // LD A, A
            0x7F => {
                self.reg.a = self.reg.a;
                1
            },

            // ADD A, B
            0x80 => {
                let n = self.reg.b;
                alu_add(&mut self.reg, n);
                1
            },

            // ADD A, L
            0x85 => {
                let n = self.reg.l;
                alu_add(&mut self.reg, n);
                1
            },

            // AND A, A
            0x87 => {
                let n = self.reg.a;
                alu_and(&mut self.reg, n);
                1
            },

            // ADC A, C
            0x89 => {
                let n = self.reg.c;
                alu_adc(&mut self.reg, n);
                1
            }
    
            // ADD A, D
            0x8A => {
                let n = self.reg.d;
                alu_add(&mut self.reg, n);
                1
            },

            // SUB B
            0x90 => {
                let n = self.reg.b;
                alu_sub(&mut self.reg, n);
                1
            },
    
            // SUB E
            0x93 => {
                let n = self.reg.e;
                alu_sub(&mut self.reg, n);
                1
            },

            // SBC A, C
            0x99 => {
                let n = self.reg.c;
                alu_sbc(&mut self.reg, n);
                1
            },
    
            // AND C
            0xA1 => {
                let n = self.reg.c;
                alu_and(&mut self.reg, n);
                1
            },

            // AND A
            0xA7 => {
                let n = self.reg.a;
                alu_and(&mut self.reg, n);
                1
            },
    
            // XOR C
            0xA9 => {
                let n = self.reg.c;
                alu_xor(&mut self.reg, n);
                1
            },
    
            // XOR A
            0xAF => {
                let n = self.reg.a;
                alu_xor(&mut self.reg, n);
                1
            },
    
            // OR B
            0xB0 => {
                let n = self.reg.b;
                alu_or(&mut self.reg, n);
                1
            },
    
            // OR C
            0xB1 => {
                let n = self.reg.c;
                alu_or(&mut self.reg, n);
                1
            },
    
            // CP A
            0xBF => {
                let n = self.reg.a;
                alu_cp(&mut self.reg, n);
                1
            },

            // RET NZ
            0xC0 => {
                if !self.reg.get_flag(Flag::Z) {
                    self.reg.pc = self.pop_stack(mem);
                    return 5;
                } else {
                    self.reg.pc += 1;
                    2
                }
            },

            // POP BC
            0xC1 => {
                let val = self.pop_stack(mem);
                self.reg.set_bc(val);
                3
            },
    
            // JP a16
            0xC3 => {
                self.reg.pc = self.next_word(mem);
                4
            },

            // PUSH BC
            0xC5 => {
                self.push_stack(self.reg.bc(), mem);
                4
            },

            // ADD A, d8
            0xC6 => {
                let n = self.next_byte(mem);
                alu_add(&mut self.reg, n);
                2
            },

            // RET Z
            0xC8 => {
                if self.reg.get_flag(Flag::Z) {
                    self.reg.pc = self.pop_stack(mem);
                    return 5;
                } else {
                    self.reg.pc += 1;
                    2
                }
            },
    
            // RET
            0xC9 => {
                let adr = self.pop_stack(mem);
                self.reg.pc = adr;
                4
            }
    
            // CALL CB
            0xCB => {
                let opcode = self.next_byte(mem);
                self.cb_prefix(opcode, mem)
            }
    
            // CALL a16
            0xCD => {
                self.push_stack(self.reg.pc + 2, mem);
                self.reg.pc = self.next_word(mem);
                6
            },

            // POP DE
            0xD1 => {
                let val = self.pop_stack(mem);
                self.reg.set_de(val);
                3
            },
    
            // JP NC, a16
            0xD2 => {
                if self.reg.get_flag(Flag::C) {
                    self.reg.pc = self.next_word(mem);
                    return 4
                }
                3
            },

            // PUSH DE
            0xD5 => {
                self.push_stack(self.reg.de(), mem);
                4
            },

            // SBC A, d8
            0xDE => {
                let n = self.next_byte(mem);
                alu_sbc(&mut self.reg, n);
                2
            },

            // RETI
            0xD9 => {
                self.reg.pc = self.pop_stack(mem);
                self.set_ime(true);
                4
            },
    
            // LDH (a8), A
            0xE0 => {
                let adr = self.next_byte(mem) as u16;
                write_byte(0xFF00 + adr, self.reg.a, mem);
                3
            },

            // POP HL
            0xE1 => {
                let val = self.pop_stack(mem);
                self.reg.set_hl(val);
                3
            },
    
            // LD (C), A
            0xE2 => {
                let adr = 0xFF00 + self.reg.c as u16;
                write_byte(adr, self.reg.a, mem);
                2
            },

            // PUSH HL
            0xE5 => {
                self.push_stack(self.reg.hl(), mem);
                4
            },
    
            // AND d8
            0xE6 => {
                let n = self.next_byte(mem);
                alu_and(&mut self.reg, n);
                2
            },

            // JP HL
            0xE9 => {
                self.reg.pc = self.reg.hl();
                1
            },
    
            // LD (a16), A
            0xEA => {
                let adr = self.next_word(mem);
                write_byte(adr, self.reg.a, mem);
                4
            },

            // RST 28H
            0xEF => {
                self.push_stack(self.reg.pc, mem);
                self.reg.pc = 0x0028;
                4
            },
    
            // LDH A, (a8)
            0xF0 => {
                let adr = 0xFF00 + self.next_byte(mem) as u16;
                self.reg.a = read_byte(adr, mem);
                3
            },

            // POP AF
            0xF1 => {
                let val = self.pop_stack(mem);
                self.reg.set_af(val);
                3
            },
    
            // DI
            0xF3 => {
                self.set_ime(false);
                1
            },

            // PUSH AF
            0xF5 => {
                self.push_stack(self.reg.af(), mem);
                4
            },

            // LD A, (a16)
            0xFA => {
                let adr = self.next_word(mem);
                self.reg.a = read_byte(adr, mem);
                4
            },
    
            // EI
            0xFB => {
                self.set_ime(true);
                1
            },
    
            // CP d8
            0xFE => {
                let n = self.next_byte(mem);
                alu_cp(&mut self.reg, n);
                2
            },
    
            // Instruction not implemented
            _ => {
                panic!("unsupported instruction: {:#04x}", opcode);
            }
        }
    }
}


