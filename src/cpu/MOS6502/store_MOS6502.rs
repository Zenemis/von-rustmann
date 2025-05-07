use crate::cpu::memory::Memory; 

use crate::cpu::cpu::CPU;

use super::MOS6502;

macro_rules! st_zp {
    ($self:ident, $reg:ident, $memory:ident) => {{
        let zero_page_address: u16 = $self.fetch($memory) as u16;
        $memory[zero_page_address as usize] = $self.$reg;
        2
    }};
}

macro_rules! st_zp_with_offset {
    ($self:ident, $reg:ident, $memory:ident, $offset:expr) => {{
        let base_address: u8 = $self.fetch($memory);
        let effective_address: u8 = base_address.wrapping_add($offset);
        $memory[effective_address as usize] = $self.$reg;
        3
    }};
}

macro_rules! st_abs {
    ($self:ident, $reg:ident, $memory:ident) => {{
        let low_byte: u8 = $self.fetch($memory);
        let high_byte: u8 = $self.fetch($memory);
        let address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        $memory[address as usize] = $self.$reg;
        3
    }};
}

macro_rules! st_abs_with_offset {
    ($self:ident, $reg:ident, $memory:ident, $offset:expr) => {{
        let low_byte: u8 = $self.fetch($memory);
        let high_byte: u8 = $self.fetch($memory);
        let base_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let effective_address: u16 = base_address.wrapping_add($offset as u16);
        $memory[effective_address as usize] = $self.$reg;
        4
    }};
}

impl MOS6502 {
    // Store Accumulator register
    pub fn sta_zp(&mut self, memory: &mut Memory) -> u32 {
        st_zp!(self, regA, memory)
    }

    pub fn sta_zpx(&mut self, memory: &mut Memory) -> u32 {
        st_zp_with_offset!(self, regA, memory, self.regX)
    }

    pub fn sta_abs(&mut self, memory: &mut Memory) -> u32 {
        st_abs!(self, regA, memory)
    }

    pub fn sta_absx(&mut self, memory: &mut Memory) -> u32 {
        st_abs_with_offset!(self, regA, memory, self.regX)
    }

    pub fn sta_absy(&mut self, memory: &mut Memory) -> u32 {
        st_abs_with_offset!(self, regA, memory, self.regY)
    }

    pub fn sta_indx(&mut self, memory: &mut Memory) -> u32 {
        let base_address: u8 = self.fetch(memory);
        let indirect_address: u8 = base_address.wrapping_add(self.regX);
        let low_byte: u8 = memory[indirect_address as usize];
        let high_byte: u8 = memory[indirect_address.wrapping_add(1) as usize];
        let final_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        memory[final_address as usize] = self.regA;
        5
    }

    pub fn sta_indy(&mut self, memory: &mut Memory) -> u32 {
        let base_address: u8 = self.fetch(memory);
        let low_byte: u8 = memory[base_address as usize];
        let high_byte: u8 = memory[base_address.wrapping_add(1) as usize];
        let base_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let effective_address: u16 = base_address.wrapping_add(self.regY as u16);
        memory[effective_address as usize] = self.regA;
        5
    }

    // Store X register
    pub fn stx_zp(&mut self, memory: &mut Memory) -> u32 {
        st_zp!(self, regX, memory)
    }

    pub fn stx_zpy(&mut self, memory: &mut Memory) -> u32 {
        st_zp_with_offset!(self, regX, memory, self.regY)
    }

    pub fn stx_abs(&mut self, memory: &mut Memory) -> u32 {
        st_abs!(self, regX, memory)
    }

    // Store Y register
    pub fn sty_zp(&mut self, memory: &mut Memory) -> u32 {
        st_zp!(self, regY, memory)
    }

    pub fn sty_zpx(&mut self, memory: &mut Memory) -> u32 {
        st_zp_with_offset!(self, regY, memory, self.regX)
    }

    pub fn sty_abs(&mut self, memory: &mut Memory) -> u32 {
        st_abs!(self, regY, memory)
    }
}