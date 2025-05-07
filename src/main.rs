mod cpu;

use cpu::cpu::CPU;
use cpu::MOS6502::MOS6502;
use cpu::memory::Memory;

fn main() {
    let mut my_cpu = MOS6502::new();
    let mut memory = Memory::new();
    memory[0xFFFC] = 0xA5;
    memory[0xFFFD] = 0x84;
    memory[0x0084] = 0x42;

    my_cpu.execute(3, &mut memory);
}