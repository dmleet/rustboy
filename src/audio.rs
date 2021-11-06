use crate::mmu::*;

pub struct Audio_Engine{

}

impl Audio_Engine{
    pub fn Do_Audio(mem: &Memory){
        loop {
            for i in 0xff10..0xff46 {
                if mem[i] != 0 {
                    println!("{} - {}",i, mem[i]);
                }
            }
        }
    }
}