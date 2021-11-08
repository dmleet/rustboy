use crate::alu::*;
use crate::mmu::*;
use crate::cpu::*;

impl Cpu {
    pub fn cb_prefix(&mut self, opcode: u8, mem: &mut Memory) -> u16 {

        match opcode {

            // RLC (HL)
            0x06 => {
                let adr = self.reg.hl();
                let val = alu_rlc(&mut self.reg, read_byte(adr, mem));
                write_byte(adr, val, mem);
                4
            }

            // SLA A
            0x27 => {
                let n = self.reg.a;
                let val = alu_sla(&mut self.reg, n);
                self.reg.a = val;
                2
            }
    
            // SWAP A
            0x37 => {
                let n = self.reg.a;
                self.reg.a = alu_swap(&mut self.reg, n);
                2
            },

            // BIT 0, C
            0x41 => {
                let r = self.reg.c;
                alu_bit(&mut self.reg, 0, r);
                2
            },

            // BIT 5, A
            0x6F => {
                let b = self.reg.a;
                alu_bit(&mut self.reg, b, 5);
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

            // SET 7. (HL)
            0xFE => {
                let adr = self.reg.hl();
                let val = read_byte(adr, mem) | 0x80;
                write_byte(adr, val, mem);
                4
            },
    
            _ => {
                panic!("unsupported instruction: CB {:#04x}", opcode);
            }
        }
    }
}