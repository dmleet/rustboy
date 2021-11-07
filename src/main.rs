mod registers;
mod alu;
mod mmu;
mod cpu;
mod gpu;
mod cb;

use crate::mmu::*;
use crate::cpu::*;
use crate::gpu::*;

use minifb::{ Window, WindowOptions };

use std::io::{ Read, Write };
use std::path::Path;
use std::fs::File;
use log::debug;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    env_logger::builder()
        .format(|buf, record| {
            match record.level() {
                log::Level::Debug => writeln!(buf, "{}", record.args()),
                _ => writeln!(buf, "{}: {}", record.level(), record.args()),
                
            }
        }).init();

    println!("Hello, rustboy!");

    let mut mem: Memory = [0; 0xFFFF + 1];
    let mut cpu: Cpu = Cpu::new();
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
    
    let mut count: u32 = 0;
    loop {
        count += 1;

        debug!("IME: {}", cpu.reg.ime);

        // CPU
        let cycles = cpu.tick(&mut mem);

        // GPU
        if gpu.tick(&mut mem, cycles * 4) {
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        }

        // Print
        debug!("Call count: {}", count);
        debug!("Line Y: {}", read_byte(0xFF44, &mem));
        cpu.reg.debug();
        debug!("\n");
    }
}
