use crate::registers::{ Registers, Flag };

trait Carry {
    fn carry_add(&self, rhs: u8) -> (u8, bool, bool);
    fn carry_sub(&self, rhs: u8) -> (u8, bool, bool);
}

impl Carry for u8 {
    fn carry_add(&self, rhs: u8) -> (u8, bool, bool) {
        let r = self.wrapping_add(rhs);
        // https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/
        let h = ((*self & 0x0F) + (rhs & 0x0F)) & 0x10 == 0x10;
        let c = (*self as u16) + (rhs as u16) > 0xFF;
        (r, h, c)
    }

    fn carry_sub(&self, rhs: u8) -> (u8, bool, bool) {
        let r = self.wrapping_sub(rhs);
        let h = (*self & 0x0F) < (rhs & 0x0F);
        let c = *self < rhs;
        (r, h, c)
    }
}

/// 8-bit Functions

// Add A, n
pub fn alu_add(reg: &mut Registers, n: u8) {
    let (r, h, c) = reg.a.carry_add(n);
    reg.set_flags(r == 0, false, h, c);
    reg.a = r;
}

// ADC A, n
pub fn alu_adc(reg: &mut Registers, n: u8) {
    let c: u8 = reg.get_flag_bit(Flag::C);
    alu_add(reg, n + c);
}

// SUB A, n
pub fn alu_sub(reg: &mut Registers, n: u8) {
    let (r, h, c) = reg.a.carry_sub(n);
    reg.set_flags(r == 0, true, h, c);
    reg.a = r;
}

// SBC A, n
pub fn alu_sbc(reg: &mut Registers, n: u8) {
    let c: u8 = reg.get_flag_bit(Flag::C);
    alu_sub(reg, n + c);
}

// AND n
pub fn alu_and(reg: &mut Registers, n: u8) {
    reg.a &= n;
    reg.set_flags(reg.a == 0, false, true, false);
}

// OR n
pub fn alu_or(reg: &mut Registers, n: u8) {
    reg.a |= n;
    reg.set_flags(reg.a == 0, false, false, false);
}

// XOR n
pub fn alu_xor(reg: &mut Registers, n: u8) {
    reg.a ^= n;
    reg.set_flags(reg.a == 0, false, false, false);
}

// Sets flags from CP n
pub fn alu_cp(reg: &mut Registers, n: u8) {
    let (r, h, c) = reg.a.carry_sub(n);
    reg.set_flags(r == 0, true, h, c);
}

// Returns the result of INC n and sets flags
pub fn alu_inc(reg: &mut Registers, n: u8) -> u8 {
    let (r, h, _) = n.carry_add(1);
    reg.set_flag(Flag::Z, r == 0);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, h);
    r
}

// Returns the result of DEC n and sets flags
pub fn alu_dec(reg: &mut Registers, n: u8) -> u8 {
    let (r, h, _) = n.carry_sub(1);
    reg.set_flag(Flag::Z, r == 0);
    reg.set_flag(Flag::N, true);
    reg.set_flag(Flag::H, h);
    r
}

/// 16-bit Functions

// Adds n to HL and sets flags
pub fn alu_add_hl(reg: &mut Registers, n: u16) {
    let r = reg.hl().wrapping_add(n);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, (reg.hl() & 0x07FF) + (n & 0x07FF) > 0x07FF);
    reg.set_flag(Flag::C, ((reg.hl() as u32) + (n as u32)) > 0xFFFF);
    reg.set_hl(r);
}

/// Misc

// SWAP n
pub fn alu_swap(reg: &mut Registers, n: u8) -> u8 {
    let r = (n & 0xF0) >> 4 | (n & 0x0F) << 4;
    reg.set_flags(r == 0, false, false, false);
    r
}

/// Rotates

// RLC n
pub fn alu_rlc(reg: &mut Registers, n: u8) -> u8 {
    let r = n << 1 | n >> 7;
    reg.set_flags(r == 0, false, false, n & 0x80 == 0x80);
    r
}

// SLA n
pub fn alu_sla(reg: &mut Registers, n: u8) -> u8 {
    let r = (n << 1) & 0xFE;
    reg.set_flags(r == 0, false, false, n & 0x80 == 0x80);
    r
}

/// Bit Opcodes

// BIT b, r
pub fn alu_bit(reg: &mut Registers, b: u8, r: u8) {
    reg.set_flag(Flag::Z, r & (1 << b) == 0);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, true);
}

// SET b, r
pub fn alu_set(r: u8, b: u8) -> u8 {
    r | (1 << b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carry_add() {
        let mut reg = Registers::new();
        reg.a = 0xFF;
        let (r, h, c) = reg.a.carry_add(0x01);
        assert_eq!(r, 0x00);
        assert_eq!(h, true);
        assert_eq!(c, true);
    }

    #[test]
    fn test_carry_sub() {
        let mut reg = Registers::new();
        reg.a = 0x00;
        let (r, h, c) = reg.a.carry_sub(0xFF);
        assert_eq!(r, 0x01);
        assert_eq!(h, true);
        assert_eq!(c, true);
    }

    #[test]
    fn test_alu_add() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        reg.a = 0x01;
        alu_add(&mut reg, 0x02);
        assert_eq!(reg.a, 0x03);
        assert_eq!(reg.f, 0);
    }

    #[test]
    fn test_alu_adc() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        reg.a = 0x01;
        alu_adc(&mut reg, 0x02);
        assert_eq!(reg.a, 0x04);
        assert_eq!(reg.f, 0);
    }

    #[test]
    fn test_alu_sub() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        reg.a = 0x01;
        alu_sub(&mut reg, 0x02);
        assert_eq!(reg.a, 0xFF);
        assert_eq!(reg.f, 0b01110000);
    }

    #[test]
    fn test_alu_sbc() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        reg.a = 0x01;
        alu_sbc(&mut reg, 0x02);
        assert_eq!(reg.a, 0xFE);
        assert_eq!(reg.f, 0b01110000);
    }

    #[test]
    fn test_alu_and() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        reg.a = 0x01;
        alu_and(&mut reg, 0x02);
        assert_eq!(reg.a, 0x00);
        assert_eq!(reg.f, 0b10100000);
    }

    #[test]
    fn test_alu_or() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        reg.a = 0x01;
        alu_or(&mut reg, 0x03);
        assert_eq!(reg.a, 0x03);
        assert_eq!(reg.f, 0);
    }

    #[test]
    fn test_alu_xor() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        reg.a = 0x01;
        alu_xor(&mut reg, 0x03);
        assert_eq!(reg.a, 0x02);
        assert_eq!(reg.f, 0);
    }

    #[test]
    fn test_alu_cp() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        reg.a = 0x01;
        alu_cp(&mut reg, 0x02);
        assert_eq!(reg.a, 0x01);
        assert_eq!(reg.f, 0b01110000);
    }

    #[test]
    fn test_alu_inc() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        reg.a = 0x01;
        let n = reg.a;
        reg.a = alu_inc(&mut reg, n);
        assert_eq!(reg.a, 0x02);
        assert_eq!(reg.f, 0b00010000);
    }

    #[test]
    fn test_alu_dec() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        reg.a = 0x01;
        let n = reg.a;
        reg.a = alu_dec(&mut reg, n);
        assert_eq!(reg.a, 0x00);
        assert_eq!(reg.f, 0b11010000);
    }

	#[test]
	fn test_alu_add_hl() {
		let mut reg = Registers::new();
        reg.set_hl(0x0FFF);
        let n =	0x0FFE;
		alu_add_hl(&mut reg, n);
		assert!(!reg.get_flag(Flag::N));
		assert!(reg.get_flag(Flag::H));
		assert!(!reg.get_flag(Flag::C));
	}

    #[test]
    fn test_alu_swap() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        let n = alu_swap(&mut reg, 0x01);
        assert_eq!(n, 0b00010000);
        assert_eq!(reg.f, 0b00000000);
    }

    #[test]
    fn test_alu_rlc() {
        let mut reg = Registers::new();
        reg.f = 0b10000000;
        let n = alu_rlc(&mut reg, 0x81);
        assert_eq!(n, 0b00000011);
        assert_eq!(reg.f, 0b00010000);
    }

    #[test]
    fn test_alu_sla() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        let n = alu_sla(&mut reg, 0x81);
        assert_eq!(n, 0b00000010);
        assert_eq!(reg.f, 0b00010000);
    }

    #[test]
    fn test_alu_bit() {
        let mut reg = Registers::new();
        reg.f = 0b11110000;
        alu_bit(&mut reg, 0, 0x01);
        assert_eq!(reg.f, 0b00110000);
    }

    #[test]
    fn test_alu_set() {
        let mut r = 0x01;
        r = alu_set(r, 1);
        assert_eq!(r, 0x03);
    }
}