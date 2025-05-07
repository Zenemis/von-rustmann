use crate::cpu::memory::Memory; 

use crate::cpu::cpu::CPU;

use super::processor_status::ProcessorStatus;
use super::MOS6502;

fn on_ld_set_status(proc_status: &mut ProcessorStatus, value: u8) {
    if value == 0 {
        proc_status.set_zero();
    } else {
        proc_status.clear_zero();
    }
    if value & 0b1000_0000 != 0 {
        proc_status.set_negative();
    } else {
        proc_status.clear_negative();
    }
}

macro_rules! ld_im {
    ($self:ident, $reg:ident, $memory:ident) => {{
        let byte: u8 = $self.fetch($memory);
        $self.$reg = byte;
        on_ld_set_status(&mut $self.proc_status, $self.$reg);
        1
    }};
}

macro_rules! ld_zp {
    ($self:ident, $reg:ident, $memory:ident) => {{
        let zero_page_address: u16 = $self.fetch($memory) as u16;
        $self.$reg = $self.read(zero_page_address, $memory);
        on_ld_set_status(&mut $self.proc_status, $self.$reg);
        2
    }};
}

macro_rules! ld_zp_with_offset {
    ($self:ident, $reg:ident, $memory:ident, $offset:expr) => {{
        let base_address: u8 = $self.fetch($memory);
        let effective_address: u8 = base_address.wrapping_add($offset);
        let value: u8 = $memory[effective_address as usize];
        $self.$reg = value;
        on_ld_set_status(&mut $self.proc_status, $self.$reg);
        3
    }};
}

macro_rules! ld_abs {
    ($self:ident, $reg:ident, $memory:ident) => {{
        let low_byte: u8 = $self.fetch($memory);
        let high_byte: u8 = $self.fetch($memory);
        let address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let value: u8 = $self.read(address, $memory);
        $self.$reg = value;
        on_ld_set_status(&mut $self.proc_status, $self.$reg);
        3
    }};
}

macro_rules! ld_absolute_with_offset {
    ($self:ident, $reg:ident, $memory:ident, $offset:expr) => {{
        let low_byte: u8 = $self.fetch($memory);
        let high_byte: u8 = $self.fetch($memory);
        let base_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let effective_address: u16 = base_address.wrapping_add($offset as u16);
        let value: u8 = $self.read(effective_address, $memory);
        $self.$reg = value;
        on_ld_set_status(&mut $self.proc_status, $self.$reg);

        // Check for page boundary crossing and return the appropriate cycle count
        if (base_address & 0xFF00) != (effective_address & 0xFF00) {
            4
        } else {
            3
        }
    }};
}

impl MOS6502 {
    // Load Accumulator register
    pub fn lda_im(&mut self, memory: &Memory) -> u32 {
        ld_im!(self, regA, memory)
    }

    pub fn lda_zp(&mut self, memory: &Memory) -> u32 {
        ld_zp!(self, regA, memory)
    }

    pub fn lda_zpx(&mut self, memory: &Memory) -> u32 {
        ld_zp_with_offset!(self, regA, memory, self.regX)
    }

    pub fn lda_abs(&mut self, memory: &Memory) -> u32 {
        ld_abs!(self, regA, memory)
    }

    pub fn lda_absx(&mut self, memory: &Memory) -> u32 {
        ld_absolute_with_offset!(self, regA, memory, self.regX)
    }

    pub fn lda_absy(&mut self, memory: &Memory) -> u32 {
        ld_absolute_with_offset!(self, regA, memory, self.regY)
    }

    pub fn lda_indx(&mut self, memory: &Memory) -> u32 {
        let base_address: u8 = self.fetch(memory);
        let indirect_address: u8 = base_address.wrapping_add(self.regX);
        let low_byte: u8 = memory[indirect_address as usize];
        let high_byte: u8 = memory[indirect_address.wrapping_add(1) as usize];
        let final_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let value: u8 = self.read(final_address, memory);
        self.regA = value;
        on_ld_set_status(&mut self.proc_status, self.regA);
        5
    }

    pub fn lda_indy(&mut self, memory: &Memory) -> u32 {
        let base_address: u8 = self.fetch(memory);
        let low_byte: u8 = memory[base_address as usize];
        let high_byte: u8 = memory[base_address.wrapping_add(1) as usize];
        let base_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let effective_address: u16 = base_address.wrapping_add(self.regY as u16);
        let value: u8 = self.read(effective_address, memory);
        self.regA = value;
        on_ld_set_status(&mut self.proc_status, self.regA);
        
        // Check for page boundary crossing and return the appropriate cycle count
        if (base_address & 0xFF00) != (effective_address & 0xFF00) {
            5 
        } else {
            4
        }
    }

    // Load X register
    pub fn ldx_im(&mut self, memory: &Memory) -> u32 {
        ld_im!(self, regX, memory)
    }

    pub fn ldx_zp(&mut self, memory: &Memory) -> u32 {
        ld_zp!(self, regX, memory)
    }

    pub fn ldx_zpy(&mut self, memory: &Memory) -> u32 {
        ld_zp_with_offset!(self, regX, memory, self.regY)
    }

    pub fn ldx_abs(&mut self, memory: &Memory) -> u32 {
        ld_abs!(self, regX, memory)
    }

    pub fn ldx_absy(&mut self, memory: &Memory) -> u32 {
        ld_absolute_with_offset!(self, regX, memory, self.regY)
    }

    // Load Y register
    pub fn ldy_im(&mut self, memory: &Memory) -> u32 {
        ld_im!(self, regY, memory)
    }

    pub fn ldy_zp(&mut self, memory: &Memory) -> u32 {
        ld_zp!(self, regY, memory)
    }

    pub fn ldy_zpx(&mut self, memory: &Memory) -> u32 {
        ld_zp_with_offset!(self, regY, memory, self.regX)
    }

    pub fn ldy_abs(&mut self, memory: &Memory) -> u32 {
        ld_abs!(self, regY, memory)
    }

    pub fn ldy_absx(&mut self, memory: &Memory) -> u32 {
        ld_absolute_with_offset!(self, regY, memory, self.regX)
    }
}