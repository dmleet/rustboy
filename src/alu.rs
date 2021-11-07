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

// 8-bit Functions

// Stores the result of ADD n in A and sets flags
pub fn alu_add(reg: &mut Registers, n: u8) {
    let r = reg.a.carry_add(n);
    reg.set_flag(Flag::Z, r.0 == 0);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, r.1);
    reg.set_flag(Flag::C, r.2);
    reg.a = r.0;
}

// Returns the result of AND n and sets flags
pub fn alu_and(reg: &mut Registers, n: u8) {
    reg.a &= n;
    reg.set_flag(Flag::Z, reg.a == 0);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, true);
    reg.set_flag(Flag::C, false);
}

// BIT b, r
pub fn alu_bit(reg: &mut Registers, b: u8, r: u8) {
    reg.set_flag(Flag::Z, r & (1 << b) == 1);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, true);
}

// Sets flags from CP n
pub fn alu_cp(reg: &mut Registers, n: u8) {
    let r = reg.a.carry_sub(n);
    reg.set_flag(Flag::Z, r.0 == 0);
    reg.set_flag(Flag::N, true);
    reg.set_flag(Flag::H, r.1);
    reg.set_flag(Flag::C, r.2);
}

// Returns the result of DEC n and sets flags
pub fn alu_dec(reg: &mut Registers, n: u8) -> u8 {
    let r = n.carry_sub(1);
    reg.set_flag(Flag::Z, r.0 == 0);
    reg.set_flag(Flag::N, true);
    reg.set_flag(Flag::H, r.1);
    r.0
}

// Returns the result of INC n and sets flags
pub fn alu_inc(reg: &mut Registers, n: u8) -> u8 {
    let r = n.carry_add(1);
    reg.set_flag(Flag::Z, r.0 == 0);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, r.1);
    r.0
}

// Stores the reult of OR n in A and sets flags
pub fn alu_or(reg: &mut Registers, n: u8) {
    reg.a = reg.a | n;
    reg.set_flag(Flag::Z, reg.a == 0);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, false);
    reg.set_flag(Flag::C, false);
}

pub fn alu_sbc(reg: &mut Registers, n: u8) {
    let c = reg.get_flag(Flag::C) as u8;
    let r = reg.a.carry_sub(n + c);
    reg.set_flag(Flag::Z, r.0 == 0);
    reg.set_flag(Flag::N, true);
    reg.set_flag(Flag::H, r.1);
    reg.set_flag(Flag::C, r.2);
    reg.a = r.0;
}

// Stores the result of SUB n in A and sets flags
pub fn alu_sub(reg: &mut Registers, n: u8) {
    let r = reg.a.carry_sub(n);
    reg.set_flag(Flag::Z, r.0 == 0);
    reg.set_flag(Flag::N, true);
    reg.set_flag(Flag::H, r.1);
    reg.set_flag(Flag::C, r.2);
    reg.a = r.0;
}

// Stores the result of XOR n in A and sets flags
pub fn alu_xor(reg: &mut Registers, n: u8) {
    reg.a = reg.a ^ n;
    reg.set_flag(Flag::Z, reg.a == 0);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, false);
    reg.set_flag(Flag::C, false);
}

// 16-bit Functions

// Adds n to HL and sets flags
pub fn alu_add_hl(reg: &mut Registers, n: u16) {
    let r = reg.hl().wrapping_add(n);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, (reg.hl() & 0x07FF) + (n & 0x07FF) > 0x07FF);
    reg.set_flag(Flag::C, ((reg.hl() as u32) + (n as u32)) > 0xFFFF);
    reg.set_hl(r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alu_cp() {
        let mut reg = Registers::new();
        reg.a =		0b11010010;
        let n: u8 =	0b00101001;
        alu_cp(&mut reg, n);
		assert!(!reg.get_flag(Flag::Z));
		assert!(reg.get_flag(Flag::N));
		assert!(reg.get_flag(Flag::H));
		assert!(!reg.get_flag(Flag::C));
    }

	#[test]
	fn test_alu_cp_2() {
		let mut reg = Registers::new();
		reg.a =		0b01011001;
        let n: u8 =	0b10000100;
        alu_cp(&mut reg, n);
		assert!(!reg.get_flag(Flag::Z));
		assert!(reg.get_flag(Flag::N));
		assert!(!reg.get_flag(Flag::H));
		assert!(reg.get_flag(Flag::C));
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
	fn test_alu_add_hl_2() {
		let mut reg = Registers::new();
        reg.set_hl(0xF000);
        let n =	0xFFFF;
		alu_add_hl(&mut reg, n);
		assert!(!reg.get_flag(Flag::N));
		assert!(!reg.get_flag(Flag::H));
		assert!(reg.get_flag(Flag::C));
	}
}