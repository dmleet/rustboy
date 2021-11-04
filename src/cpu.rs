use crate::alu::*;
use crate::registers::*;
use crate::mmu::*;

trait SignedAdd {
    fn signed_add(self, rhs: i8) -> Self;
}

impl SignedAdd for u16 {
    fn signed_add(self, rhs: i8) -> u16 {
        ((self as i32) + (rhs as i32)) as u16
    }
}

    // Get next byte from memory and increment program counter
pub fn next_byte(reg: &mut Registers, mem: &mut [u8]) -> u8 {
    let byte = read_byte(reg.pc, mem);
    reg.pc += 1;
    byte
}

// Get next word from memory and increment program counter
pub fn next_word(reg: &mut Registers, mem: &[u8]) -> u16 {
    let word = read_word(reg.pc, mem);
    reg.pc += 2;
    word
}

// Cpu instruction set
// Returns m-cycle length of instruction
pub fn call_instruction(opcode: u8, reg: &mut Registers, mem: &mut [u8]) -> u32 {
    match opcode {

        // NOP
        0x00 => { 1 },

        // LD BC, d16
        0x01 => {
            let word = next_word(reg, mem);
            reg.set_bc(word);
            3
        },

        // LD (BC), A
        0x02 => {
            write_byte(reg.bc(), reg.a, mem);
            2
        },

        // INC BC
        0x03 => {
            reg.set_bc(reg.bc().wrapping_add(1));
            2
        },

        // INC B
        0x04 => {
            reg.b = alu_inc(reg, reg.b);
            1
        },

        // DEC B
        0x05 => {
            reg.b = alu_dec(reg, reg.b);
            1
        },

        // LD B, d8
        0x06 => {
            reg.b = next_byte(reg, mem);
            2
        },

        // RLCA
        0x07 => {
            let c = reg.a >> 7;
            reg.a = (reg.a << 1) | c;
            reg.set_flag(Flag::Z, false);
            reg.set_flag(Flag::N, false);
            reg.set_flag(Flag::H, false);
            reg.set_flag(Flag::C, c == 1);
            1
        },

        // LD (a16), SP
        0x08 => {
            let adr = next_word(reg, mem);
            write_word(adr, reg.sp, mem);
            5
        },

        // LD A, (BC)
        0x0A => {
            reg.a = read_byte(reg.bc(), mem);
            2
        },

        // DEC D
        0x0D => {
            reg.d = alu_dec(reg, reg.d);
            1
        },

        // LD C, d8
        0x0E => {
            reg.c = next_byte(reg, mem);
            2
        },

        // STOP
        0x10 => { 1 },

        // LD DE, d16
        0x11 => {
            let w = next_word(reg, mem);
            reg.set_de(w);
            3
        },

        // INC D
        0x14 => {
            reg.d = alu_inc(reg, reg.d);
            1
        }

        // DEC D
        0x15 => {
            reg.d = alu_dec(reg, reg.d);
            1
        },

        // LD D, d8
        0x16 => {
            reg.d = next_byte(reg, mem);
            2
        },

        // JR r8
        0x18 => {
            let n = next_byte(reg, mem) as i8;
            reg.pc = ((reg.pc as i32) + (n as i32)) as u16;
            3
        },

        // INC C
        0x0C => {
            reg.c = alu_inc(reg, reg.c);
            1
        },

        // ADD HL, DE
        0x19 => {
            alu_add_hl(reg, reg.de());
            2
        },

        // DEC E
        0x1D => {
            reg.e = alu_dec(reg, reg.e);
            1
        },

        // LD E, d8
        0x1E => {
            reg.e = next_byte(reg, mem);
            2
        },

        // RRA
        0x1F => {
            let c = reg.get_flag(Flag::C) as u8;
            reg.set_flag(Flag::Z, false);
            reg.set_flag(Flag::H, false);
            reg.set_flag(Flag::N, false);
            reg.set_flag(Flag::C, (reg.a & 0x01) == 0x01);
            reg.a = (reg.a >> 1) | (c << 7);
            1
        },

        // JR NZ, r8
        0x20 => {
            if !reg.get_flag(Flag::Z) {
                let n = next_byte(reg, mem) as i8;
                println!("r8: {}", n);
                reg.pc = reg.pc.signed_add(n);
                return 3;
            } else {
                reg.pc += 1;
                2
            }
        },

        // LD HL, d16
        0x21 => {
            let word = next_word(reg, mem);
            reg.set_hl(word);
            3
        },

        // INC HL
        0x23 => {
            reg.set_hl(reg.hl().wrapping_add(1));
            2
        }

        // DEC H
        0x25 => {
            reg.h = alu_dec(reg, reg.h);
            1
        }

        // INC L
        0x2C => {
            reg.l = alu_inc(reg, reg.l);
            1
        },

        // ADD HL, HL
        0x29 => {
            alu_add_hl(reg, reg.hl());
            2
        },

        // LD SP, d16
        0x31 => {
            reg.sp = next_word(reg, mem);
            3
        },

        // LD (HL-), A
        0x32 => { 
            reg.set_hl(reg.hl() - 1);
            write_byte(reg.hl(), reg.a, mem);
            2
        },

        // JR C, r8
        0x38 => {
            if reg.get_flag(Flag::C) {
                let n = next_byte(reg, mem) as i8;
                reg.pc = ((reg.pc as i32) + (n as i32)) as u16;
            }
            2
        },

        // LD B, C
        0x41 => {
            reg.b = reg.c;
            1
        },

        // LD H, A
        0x67 => {
            reg.h = reg.a;
            1
        },

        // LD (HL), A
        0x77 => {
            write_byte(reg.hl(), reg.a, mem);
            2
        },

        // LD A, B
        0x78 => {
            reg.a = reg.b;
            1
        },

        // LD A, C
        0x79 => {
            reg.a = reg.c;
            1
        },

        // LD A, E
        0x7B => {
            reg.a = reg.e;
            1
        },

        // LD A, H
        0x7C => {
            reg.a = reg.h;
            1
        },

        // LD A, A
        0x7F => {
            reg.a = reg.a;
            1
        },

        // ADD A, D
        0x8A => {
            alu_add(reg, reg.d);
            1
        },

        // SUB E
        0x93 => {
            alu_sub(reg, reg.e);
            1
        },

        // XOR A
        0xAF => {
            reg.a ^= reg.a;
            reg.set_flag(Flag::Z, true);
            reg.set_flag(Flag::N, false);
            reg.set_flag(Flag::H, false);
            reg.set_flag(Flag::C, false);
            1
        },

        // OR B
        0xB0 => {
            alu_or(reg, reg.b);
            1
        },

        // CP A
        0xBF => {
            alu_cp(reg, reg.a);
            1
        },

        // JP a16
        0xC3 => {
            reg.pc = next_word(reg, mem);
            4
        },

        // JP NC, a16
        0xD2 => {
            if reg.get_flag(Flag::C) {
                reg.pc = next_word(reg, mem);
                return 4
            }
            3
        },

        // Instruction not implemented
        _ => {
            panic!("unsupported instruction: {:#04x}", opcode);
        }
    }
}
