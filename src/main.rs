mod registers;
mod alu;
mod mmu;
mod cpu;
mod gpu;
mod audio;

use crate::audio::AudioEngine;
use crate::registers::*;
use crate::mmu::*;
use crate::cpu::*;
use crate::gpu::*;

use minifb::{ Window, WindowOptions };

use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::thread;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    println!("Hello, rustboy!");

    let mut reg = Registers::new();
    let mut mem: Memory = [0; 0xFFFF + 1];
    let mut gpu = Gpu::new();
    let mut window = Window::new("rustboy", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut buffer: Vec<u32> = vec![255; WIDTH * HEIGHT];
    
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
    
    // Start audio thread
    thread::spawn(move || AudioEngine::Do_Audio(&mem));

    let mut count: u32 = 0;
    loop {
        count += 1;

        // CPU
        let opcode = next_byte(&mut reg, &mut mem);
        let cycles = call_instruction(opcode, &mut reg, &mut mem);

        // GPU
        if gpu.tick(&mut mem, cycles * 4) {
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        }

        // Print
        println!("Last opcode: {:#04x}", opcode);
        println!("Call count: {}", count);
        println!("Line Y: {}", read_byte(0xFF44, &mem));
        reg.print();

        if reg.f << 4 > 0x00 {
            println!("{:b}", reg.f);
            panic!("Forbidden bit!");
        }

        if count > 160100 {
            //println!("{:?}", &mem[0x8000..0x97FF]);
            break;
        }
    }
}
