use crate::mmu::*;

pub struct Gpu {
    pub clocks: u16,
    lcd_enabled: bool
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            clocks: 0,
            lcd_enabled: false
        }
    }

    pub fn tick(&mut self, mem: &mut Memory, cycles: u16) -> bool {
        self.lcd_enabled = read_bit(0xFF40, 7, mem);
        if !self.lcd_enabled {
            mem[0xFF44] = 0;
            return false;
        }

        self.clocks += cycles;
        if self.clocks >= 456 {
            let line_adr = 0xFF44 as usize;
            mem[line_adr] = (mem[line_adr] + 1) % 154;
            self.clocks -= 456;

            // VBlank Interrupt
            if mem[line_adr] == 0 {
                mem[0xFF0F] |= 0x01;
            }
            return true;
        }

        false
    }
}