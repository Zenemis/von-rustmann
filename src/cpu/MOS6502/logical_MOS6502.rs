use crate::cpu::memory::Memory; 

use crate::cpu::cpu::CPU;

use super::processor_status::ProcessorStatus;
use super::MOS6502;

fn on_logic_set_status(proc_status: &mut ProcessorStatus, value: u8) {
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


macro_rules! logic_im {
    ($self:ident, $memory:ident, $op:tt) => {{
        let byte: u8 = $self.fetch($memory);
        $self.regA $op byte;
        on_logic_set_status(&mut $self.proc_status, $self.regA);
    }};
}

macro_rules! logic_zp {
    ($self:ident, $memory:ident, $op:tt) => {{
        let zero_page_address: u16 = $self.fetch($memory) as u16;
        $self.regA $op $self.read(zero_page_address, $memory);
        on_logic_set_status(&mut $self.proc_status, $self.regA);
        2
    }};
}

macro_rules! logic_zpx {
    ($self:ident, $memory:ident, $op:tt) => {{
        let base_address: u8 = $self.fetch($memory);
        let effective_address: u8 = base_address.wrapping_add($self.regX);
        let value: u8 = $memory[effective_address as usize];
        $self.regA $op value;
        on_logic_set_status(&mut $self.proc_status, $self.regA);
        3
    }};
}

macro_rules! logic_abs {
    ($self:ident, $memory:ident, $op:tt) => {{
        let low_byte: u8 = $self.fetch($memory);
        let high_byte: u8 = $self.fetch($memory);
        let address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let value: u8 = $self.read(address, $memory);
        $self.regA $op value;
        on_logic_set_status(&mut $self.proc_status, $self.regA);
        3
    }};
}

macro_rules! logic_absx {
    ($self:ident, $memory:ident, $op:tt) => {{
        let low_byte: u8 = $self.fetch($memory);
        let high_byte: u8 = $self.fetch($memory);
        let base_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let effective_address: u16 = base_address.wrapping_add($self.regX as u16);
        let value: u8 = $self.read(effective_address, $memory);
        $self.regA $op value;
        on_logic_set_status(&mut $self.proc_status, $self.regA);

        if (base_address & 0xFF00) != (effective_address & 0xFF00) {
            4
        } else {
            3
        }
    }};
}

macro_rules! logic_absy {
    ($self:ident, $memory:ident, $op:tt) => {{
        let low_byte: u8 = $self.fetch($memory);
        let high_byte: u8 = $self.fetch($memory);
        let base_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let effective_address: u16 = base_address.wrapping_add($self.regY as u16);
        let value: u8 = $self.read(effective_address, $memory);
        $self.regA $op value;
        on_logic_set_status(&mut $self.proc_status, $self.regA);

        if (base_address & 0xFF00) != (effective_address & 0xFF00) {
            4
        } else {
            3
        }
    }};
}

macro_rules! logic_indx {
    ($self:ident, $memory:ident, $op:tt) => {{
        let base_address: u8 = $self.fetch($memory);
        let indirect_address: u8 = base_address.wrapping_add($self.regX);
        let low_byte: u8 = $memory[indirect_address as usize];
        let high_byte: u8 = $memory[indirect_address.wrapping_add(1) as usize];
        let final_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let value: u8 = $self.read(final_address, $memory);
        $self.regA $op value;
        on_logic_set_status(&mut $self.proc_status, $self.regA);
        5
    }};
}

macro_rules! logic_indy {
    ($self:ident, $memory:ident, $op:tt) => {{
        let base_address: u8 = $self.fetch($memory);
        let low_byte: u8 = $memory[base_address as usize];
        let high_byte: u8 = $memory[base_address.wrapping_add(1) as usize];
        let base_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let effective_address: u16 = base_address.wrapping_add($self.regY as u16);
        let value: u8 = $self.read(effective_address, $memory);
        $self.regA $op value;
        on_logic_set_status(&mut $self.proc_status, $self.regA);

        if (base_address & 0xFF00) != (effective_address & 0xFF00) {
            5
        } else {
            4
        }
    }};
}

impl MOS6502 {
    // And 
    pub fn and_im(&mut self, memory : &Memory) -> u32 {
        logic_im!(self, memory, &=);
        1
    }

    pub fn and_zp(&mut self, memory : &Memory) -> u32 {
        logic_zp!(self, memory, &=)
    }

    pub fn and_zpx(&mut self, memory : &Memory) -> u32 {
        logic_zpx!(self, memory, &=)
    }

    pub fn and_abs(&mut self, memory : &Memory) -> u32 {
        logic_abs!(self, memory, &=)
    }

    pub fn and_absx(&mut self, memory : &Memory) -> u32 {
        logic_absx!(self, memory, &=)
    }

    pub fn and_absy(&mut self, memory : &Memory) -> u32 {
        logic_absy!(self, memory, &=)
    }

    pub fn and_indx(&mut self, memory: &Memory) -> u32 {
        logic_indx!(self, memory, &=)
    }

    pub fn and_indy(&mut self, memory: &Memory) -> u32 {
        logic_indy!(self, memory, &=)
    }

    // Exclusive OR
    pub fn eor_im(&mut self, memory: &Memory) -> u32 {
        logic_im!(self, memory, ^=);
        1
    }

    pub fn eor_zp(&mut self, memory: &Memory) -> u32 {
        logic_zp!(self, memory, ^=)
    }

    pub fn eor_zpx(&mut self, memory: &Memory) -> u32 {
        logic_zpx!(self, memory, ^=)
    }

    pub fn eor_abs(&mut self, memory: &Memory) -> u32 {
        logic_abs!(self, memory, ^=)
    }

    pub fn eor_absx(&mut self, memory: &Memory) -> u32 {
        logic_absx!(self, memory, ^=)
    }

    pub fn eor_absy(&mut self, memory: &Memory) -> u32 {
        logic_absy!(self, memory, ^=)
    }

    pub fn eor_indx(&mut self, memory: &Memory) -> u32 {
        logic_indx!(self, memory, ^=)
    }

    pub fn eor_indy(&mut self, memory: &Memory) -> u32 {
        logic_indy!(self, memory, ^=)
    }

    // Inclusive OR
    pub fn ora_im(&mut self, memory: &Memory) -> u32 {
        logic_im!(self, memory, |=);
        1
    }

    pub fn ora_zp(&mut self, memory: &Memory) -> u32 {
        logic_zp!(self, memory, |=)
    }

    pub fn ora_zpx(&mut self, memory: &Memory) -> u32 {
        logic_zpx!(self, memory, |=)
    }

    pub fn ora_abs(&mut self, memory: &Memory) -> u32 {
        logic_abs!(self, memory, |=)
    }

    pub fn ora_absx(&mut self, memory: &Memory) -> u32 {
        logic_absx!(self, memory, |=)
    }

    pub fn ora_absy(&mut self, memory: &Memory) -> u32 {
        logic_absy!(self, memory, |=)
    }

    pub fn ora_indx(&mut self, memory: &Memory) -> u32 {
        logic_indx!(self, memory, |=)
    }

    pub fn ora_indy(&mut self, memory: &Memory) -> u32 {
        logic_indy!(self, memory, |=)
    }

    // BIT - Bit Test
    pub fn bit_zp(&mut self, memory: &Memory) -> u32 {
        let zero_page_address: u16 = self.fetch(memory) as u16;
        let value: u8 = self.read(zero_page_address, memory);
        if (self.regA & value) == 0 { self.proc_status.set_zero(); } 
        if value & 0b1000_0000 != 0 { self.proc_status.set_negative(); }
        if value & 0b0100_0000 != 0 { self.proc_status.set_overflow(); } 
        2
    }

    pub fn bit_abs(&mut self, memory: &Memory) -> u32 {
        let low_byte: u8 = self.fetch(memory);
        let high_byte: u8 = self.fetch(memory);
        let address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let value: u8 = self.read(address, memory);
        if (self.regA & value) == 0 { self.proc_status.set_zero(); } 
        if value & 0b1000_0000 != 0 { self.proc_status.set_negative(); }
        if value & 0b0100_0000 != 0 { self.proc_status.set_overflow(); } 
        3
    }
}