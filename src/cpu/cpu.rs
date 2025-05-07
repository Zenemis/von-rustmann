use super::memory::Memory;

pub trait CPU {
    fn fetch(&mut self, memory: &Memory) -> u8;
    fn read(&self, address: u16, memory: &Memory) -> u8;
    fn execute(&mut self, cycles : u32, memory : &mut Memory);
}
