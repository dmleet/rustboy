mod registers;
mod alu;
mod mmu;
mod cpu;

use crate::registers::*;
use crate::mmu::*;
use crate::cpu::*;

use std::io::Read;
use std::path::Path;
use std::fs::File;

fn initialize_memory(mem: &mut Memory){
    write_byte(0xFF00, 0xCF, mem);
    write_byte(0xFF01, 0x00, mem);
    write_byte(0xFF02, 0x7E, mem);
    write_byte(0xFF04, 0x18, mem);
    write_byte(0xFF05, 0x00, mem);
    write_byte(0xFF06, 0x00, mem);
    write_byte(0xFF07, 0xF8, mem);
    write_byte(0xFF0F, 0xE1, mem);
    write_byte(0xFF10, 0x80, mem);
    write_byte(0xFF11, 0xBF, mem);
    write_byte(0xFF12, 0xF3, mem);
    write_byte(0xFF13, 0xFF, mem);
    write_byte(0xFF14, 0xBF, mem);
    write_byte(0xFF16, 0x3F, mem);
    write_byte(0xFF17, 0x00, mem);
    write_byte(0xFF18, 0xFF, mem);
    write_byte(0xFF19, 0xBF, mem);
    write_byte(0xFF1A, 0x7F, mem);
    write_byte(0xFF1B, 0xFF, mem);
    write_byte(0xFF1C, 0x9F, mem);
    write_byte(0xFF1D, 0xFF, mem);
    write_byte(0xFF1E, 0xBF, mem);
    write_byte(0xFF20, 0xFF, mem);
    write_byte(0xFF21, 0x00, mem);
    write_byte(0xFF22, 0x00, mem);
    write_byte(0xFF23, 0xBF, mem);
    write_byte(0xFF24, 0x77, mem);
    write_byte(0xFF25, 0xF3, mem);
    write_byte(0xFF26, 0xF1, mem);
    write_byte(0xFF40, 0x91, mem);
    write_byte(0xFF41, 0x81, mem);
    write_byte(0xFF42, 0x00, mem);
    write_byte(0xFF43, 0x00, mem);
    write_byte(0xFF44, 0x91, mem);
    write_byte(0xFF45, 0x00, mem);
    write_byte(0xFF46, 0xFF, mem);
    write_byte(0xFF47, 0xFC, mem);
    write_byte(0xFF48, 0x00, mem);
    write_byte(0xFF49, 0x00, mem);
    write_byte(0xFF4A, 0x00, mem);
    write_byte(0xFF4B, 0x00, mem);
    write_byte(0xFF4D, 0xFF, mem);
    write_byte(0xFF4F, 0xFF, mem);
    write_byte(0xFF51, 0xFF, mem);
    write_byte(0xFF52, 0xFF, mem);
    write_byte(0xFF53, 0xFF, mem);
    write_byte(0xFF54, 0xFF, mem);
    write_byte(0xFF55, 0xFF, mem);
    write_byte(0xFF56, 0xFF, mem);
    write_byte(0xFF68, 0xFF, mem);
    write_byte(0xFF69, 0xFF, mem);
    write_byte(0xFF6A, 0xFF, mem);
    write_byte(0xFF6B, 0xFF, mem);
    write_byte(0xFF70, 0xFF, mem);
    write_byte(0xFFFF, 0x00, mem);

}
fn main() {
    println!("Hello, rustboy!");

    let mut reg = Registers::new();
    let mut mem: Memory = [0; 0xFFFF + 1];
    
    // Open the path in read-only mode
    let path = Path::new("tetris.gb");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    
    // Copy rom data to memory
    match file.read(&mut mem) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => { 
            print!("{} loaded!\n\n", display)
        }
    }

    assert_eq!(read_byte(0x0147, &mem), 0x00, "MBC not supported!");
    initialize_memory(&mut mem);

    let mut count = 0;
    loop {
        count += 1;

        let opcode = next_byte(&mut reg, &mut mem);

        // Call instruction
        call_instruction(opcode, &mut reg, &mut mem);

        // Print
        println!("Call count: {}", count);
        println!("Last opcode: {:#04x}", opcode);
        reg.print();
    }
}
