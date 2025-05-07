use crate::cpu::memory::Memory; 

use super::MOS6502;

impl MOS6502 {
    // Push operations
    pub fn pha(&mut self, memory : &mut Memory) -> u32 {
        memory[0x0100 | self.regSP as usize] = self.regA;
        2
    }

    pub fn php(&mut self, memory : &mut Memory) -> u32 {
        memory[0x0100 | self.regSP as usize] = self.proc_status.into();
        2
    }

    // Pull operations
    pub fn pla(&mut self, memory : &mut Memory) -> u32 {
        self.regA = memory[0x0100 | self.regPC as usize];
        if self.regA == 0 {
            self.proc_status.set_zero();
        } else {
            self.proc_status.clear_zero();
        }
        if self.regA & 0b1000_0000 != 0 {
            self.proc_status.set_negative();
        } else {
            self.proc_status.clear_negative();
        }
        3
    }

    pub fn plp(&mut self, memory : &mut Memory) -> u32 {
        self.proc_status = memory[0x0100 | self.regSP as usize].into();
        3
    }
}