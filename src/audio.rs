use crate::mmu::*;

pub struct AudioEngine{

}

enum SweepDirection{
    Addition,
    Subtraction
}

enum EnvelopeDirection{
    Increase,
    Decrease
}
struct SoundChannel{
    sweep_time: f32, // milliseconds
    sweep_dir: SweepDirection,
    sweep_num: u8, // 0-7
    wave_duty: f32, // percent 0.0 .. 1.0
    sound_length: u8, // 0-63 
    initial_volume: u8, // 0-0f
    envelope_direction: EnvelopeDirection,
    envelope_num: u8, // 0-7
    frequency: u8    
}

impl AudioEngine{
    pub fn Do_Audio(mem: &Memory){
        let mut chan_1 : SoundChannel;
        /* 
        loop {
            chan_1.frequency = 
            for i in 0xff10..0xff46 {
                    print!("{:X}-",mem[i]);
                
            }
            println!("");
        }
        */
    }
}