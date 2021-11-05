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

fn main() {
    println!("Hello, rustboy!");

    let mut reg = Registers::new();
    let mut mem: Memory = [0; 0xFFFF + 1];

    init_memory(&mut mem);
    
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
