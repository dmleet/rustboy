pub type Memory = [u8; 0xFFFF + 1];

pub fn init_memory(mem: &mut Memory) {
    mem[0x0000..0xFFFF + 1].iter_mut().for_each(|x| *x = 0x0000);

    let io: [(u16, u8); 40] = [
        (0xFF00, 0xCF), (0xFF02, 0x7E), (0xFF04, 0x18), (0xFF07, 0xF8),
        (0xFF0F, 0xE1), (0xFF10, 0x80), (0xFF11, 0xBF), (0xFF12, 0xF3),
        (0xFF13, 0xFF), (0xFF14, 0xBF), (0xFF16, 0x3F), (0xFF18, 0xFF),
        (0xFF19, 0xBF), (0xFF1A, 0x7F), (0xFF1B, 0xFF), (0xFF1C, 0x9F),
        (0xFF1D, 0xFF), (0xFF1E, 0xBF), (0xFF20, 0xFF), (0xFF23, 0xBF),
        (0xFF24, 0x77), (0xFF25, 0xF3), (0xFF26, 0xF1), (0xFF40, 0x91),
        (0xFF41, 0x85), (0xFF46, 0xFF), (0xFF47, 0xFC), (0xFF4D, 0xFF),
        (0xFF4F, 0xFF), (0xFF51, 0xFF), (0xFF52, 0xFF), (0xFF53, 0xFF),
        (0xFF54, 0xFF), (0xFF55, 0xFF), (0xFF56, 0xFF), (0xFF68, 0xFF), 
        (0xFF69, 0xFF), (0xFF6A, 0xFF), (0xFF6B, 0xFF), (0xFF70, 0xFF)
    ];

    for (adr, val) in io.iter() {
        write_byte(*adr, *val, mem);
    }
}

// Read byte from memory
pub fn read_byte(adr: u16, mem: &[u8]) -> u8 {
    print_debug("Read byte", adr);

    mem[adr as usize]
}

// Write word to memory
pub fn write_byte(adr: u16, val: u8, mem: &mut [u8]) {
    print_debug("Write byte", adr);

    mem[adr as usize] = val
}

// Read word from memory (lil' endian?)
pub fn read_word(adr: u16, mem: &[u8]) -> u16 {
    print_debug("Read word", adr);
    
    mem[adr as usize] as u16 | ((mem[(adr + 1) as usize] as u16) << 8)
}

// Write word to memory
pub fn write_word(adr: u16, val: u16, mem: &mut [u8]) {
    print_debug("Write word", adr);

    mem[adr as usize] = (val & 0x00FF) as u8;
    mem[(adr + 1) as usize] = (val >> 8) as u8;
}

pub fn read_bit(adr: u16, bit: u8, mem: &Memory) -> bool {
    print_debug("Read bit", adr);

    (mem[adr as usize] >> bit) & 1 == 1
}

fn print_debug(label: &str, adr: u16) {
    println!("{} ({:#04x}) - {}",
        label,
        adr,
        match adr {
            0x0000 ..= 0x7FFF => "ROM",
            0x8000 ..= 0x9FFF => "VRAM",
            0xA000 ..= 0xBFFF => "RAM",
            0xC000 ..= 0xDFFF => "WRAM",
            0xE000 ..= 0xFDFF => "byte written to forbidden memory",
            0xFE00 ..= 0xFE9F => "OAM",
            0xFEA0 ..= 0xFEFF => "byte written to forbidden memory",
            0xFF00 ..= 0xFF7F => "IO",
            0xFF80 ..= 0xFFFE => "HRAM",
            0xFFFF => "IE"
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_byte() {
        let mut mem: Memory = [0; 0xFFFF + 1];
        let adr = 0xFFFE;
        let val = 0xFF;
        write_byte(adr, val, &mut mem);
        assert_eq!(val, read_byte(adr, &mut mem));
    }

    #[test]
    fn init_memory_test() {
        let mut mem: Memory = [0; 0xFFFF + 1];
        init_memory(&mut mem);
        assert_eq!(0x00, read_byte(0x7FFF, &mut mem));
        assert_eq!(0xCF, read_byte(0xFF00, &mut mem));
        assert_eq!(0x7E, read_byte(0xFF02, &mut mem));
        assert_eq!(0x00, read_byte(0xFFFF, &mut mem));
    }
}