use crate::alu::alu_bit;
use crate::registers::*;
use crate::mmu::*;
use crate::cpu::*;

impl Cpu {
    pub fn cb_prefix(&mut self, opcode: u8, mem: &mut Memory) -> u16 {

        match opcode {

            // RLC (HL)
            0x06 => {
                let adr = self.reg.hl();
                let byte = read_byte(adr, mem);
                let res = byte << 1;
                self.reg.set_flag(Flag::Z, res == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, (byte & 0x80) == 0x80);
                write_byte(adr, res, mem);
                4
            }
    
            // SWAP A
            0x37 => {
                self.reg.a = (self.reg.a >> 4) | (self.reg.a << 4);
                self.reg.set_flag(Flag::Z, false);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, false);
                2
            },

            // BIT 6, A
            0x77 => {
                let r = self.reg.a;
                alu_bit(&mut self.reg, 6, r);
                2
            }

            // RES 0, A
            0x87 => {
                self.reg.a &= 0xFE;
                2
            },
    
            _ => {
                panic!("unsupported instruction: CB {:#04x}", opcode);
            }
        }
    }
}