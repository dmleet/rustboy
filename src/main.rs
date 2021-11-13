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

use std::io::stdin;
use std::io::{ Read, Write };
use std::ops::Mul;
use std::path::Path;
use std::fs::File;
use std::time::Duration;
use log::debug;
use std::thread;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const LIMIT_CYCLES: bool = true;

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
    
    const M_CYCLES_PER_FRAME: u32 = 16384;
    //const M_CYCLE_DUR: Duration = Duration::from_secs_f64(4.0 / 4194304.0 as f64); // unstable
    const M_CYCLE_DUR: Duration = Duration::from_nanos(954); // 953.67431640625
    let mut call_count = 0;
    let mut frame_cur_m_cycles = 0;
    loop {
        call_count += 1;
        debug!("IME: {}", cpu.reg.ime);

        // Run CPU m-cycle
        let now = std::time::Instant::now();
        let op_cycles = cpu.tick(&mut mem);
        for _cycle in 0..op_cycles {
            gpu.tick(&mut mem);
            frame_cur_m_cycles += 1;
            if frame_cur_m_cycles == M_CYCLES_PER_FRAME {
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap(); 
                frame_cur_m_cycles = 0;
            }
        }
        let elapsed = now.elapsed();
        let op_duration = M_CYCLE_DUR.mul(op_cycles as u32);
        if elapsed < op_duration && LIMIT_CYCLES {
            thread::sleep(op_duration - elapsed);
        }

        // Debug
        debug!("Call count: {}", call_count);
        debug!("Line Y: {}", read_byte(0xFF44, &mem));
        cpu.reg.debug();
        debug!("\n");

        if call_count > 80000 {
            break;
        }
    }

    println!("{:?}", &mem[0x8000..0x8800]);

    let mut tile_row = 0;
    let tile_data = &mem[0x8000..0x8800];
    tile_data.chunks_exact(16).enumerate().for_each(|(i, tile)| {
        if i % 20 == 0 && i != 0 { tile_row += 1 }
        tile.chunks_exact(2).enumerate().for_each(|(j, row)| {
            for col in 0..7 {
                let pixel = (row[0] >> (7 - col) & 0x1) + ((row[1] >> (7 - col) & 0x1));
                buffer[(tile_row * WIDTH * 8) + ((j * WIDTH) + (i * 8 + col)) as usize] = match pixel {
                    0 => 0xFF0000FF,
                    1 => 0xFF00FF00,
                    2 => 0xFFFF0000,
                    3 => 0xFFFFFFFF,
                    _ => 0xFF000000,
                };
            }
        });
    });

    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    
    // Press enter to exit
    println!("\nPress enter to exit...");
    stdin().read(&mut [0]).unwrap();

}
