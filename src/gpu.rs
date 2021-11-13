use crate::mmu::*;

#[derive(Debug, Clone, Copy)]
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

    pub fn tick(&mut self, mem: &mut Memory) {
        self.lcd_enabled = read_bit(0xFF40, 7, mem) == 1;
        if !self.lcd_enabled {
            mem[0xFF44] = 0;
            return;
        }

        self.clocks += 1;
        if self.clocks % 114 == 0 {
            let line_adr = 0xFF44 as usize;
            //mem[line_adr] = (mem[line_adr] + 1) % 154;
            mem[line_adr] += 1;
            if mem[line_adr] > 153 {
                mem[line_adr] = 0;
                mem[0xFF0F] |= 0x01;
                self.clocks -= 456;
            }

            // VBlank Interrupt
            if mem[line_adr] == 0 {
                //mem[0xFF0F] |= 0x01;
            }
        }
    }
}