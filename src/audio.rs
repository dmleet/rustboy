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

impl SoundChannel{
    fn new() -> SoundChannel{
        return SoundChannel { sweep_time: 0.0, sweep_dir: SweepDirection::Addition, sweep_num: 0,
             wave_duty: 0.0, sound_length: 0, initial_volume: 0,
              envelope_direction: EnvelopeDirection::Increase, envelope_num: 0, frequency: 1 };
    }
}

impl AudioEngine{
    pub fn do_audio(mem: &Memory){
        let mut chan_1 = SoundChannel::new();
        loop {

            chan_1.sweep_dir = match read_bit(0xFF10, 3, mem){
                true => SweepDirection::Subtraction,
                false => SweepDirection::Addition
            };

            


        }
        
    }
}