pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub ime: bool,
}

#[derive(Clone, Copy)]
pub enum Flag {
    Z = 0b10000000,
    N = 0b01000000,
    H = 0b00100000,
    C = 0b00010000
}

impl Registers {
    pub fn new() -> Registers {
        // Initial values set by bootstrap ROM
        // https://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM#Contents_of_the_ROM
        Registers {
            a: 0x01,
            f: 0x80,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100, // entry point
            ime: false,
        }
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0x00FF) as u8;
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0x00FF) as u8;
    }

    pub fn set_flag(&mut self, flag: Flag, state: bool) {
        match state {
            true => self.f |= flag as u8,
            false => self.f &= !(flag as u8)
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => { ((self.f >> 7) & 0x01) > 0 },
            Flag::N => { ((self.f >> 6) & 0x01) > 0 },
            Flag::H => { ((self.f >> 5) & 0x01) > 0 },
            Flag::C => { ((self.f >> 4) & 0x01) > 0 }
        }
    }

    pub fn print(&self) {
        print!("af: {:#04x} {:#04x}\nbc: {:#04x} {:#04x}\nde: {:#04x} {:#04x}\nhl: {:#04x} {:#04x}\nsp: {:#06x}\npc: {:#06x}\n",
            self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc);
        print!("Z N H C\n{} {} {} {}\n\n", 
            self.get_flag(Flag::Z) as u8, self.get_flag(Flag::N) as u8, self.get_flag(Flag::H) as u8, self.get_flag(Flag::C) as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]	
    fn test_registers()
    {
        let mut reg = Registers::new();

        reg.b = 0x34;
        reg.c = 0x45;
        reg.d = 0x56;
        reg.e = 0x67;
        reg.h = 0x78;
        reg.l = 0x89;
        assert_eq!(reg.bc(), 0x3445);
        assert_eq!(reg.de(), 0x5667);
        assert_eq!(reg.hl(), 0x7889);

        reg.set_bc(0x1111);
        reg.set_de(0x1111);
        reg.set_hl(0x1111);
        assert_eq!(reg.bc(), 0x1111);
        assert_eq!(reg.de(), 0x1111);
        assert_eq!(reg.hl(), 0x1111);
    }

    #[test]
    fn test_flags()
    {
        let mut reg = Registers::new();

        for flag in [Flag::Z, Flag::N, Flag::H, Flag::C].iter() {
            reg.set_flag(*flag, true);
            assert_eq!(reg.get_flag(*flag), true);
            reg.set_flag(*flag, false);
            assert_eq!(reg.get_flag(*flag), false);
        }
    }
}