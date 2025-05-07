use crate::cpu::memory::Memory; 

use crate::cpu::cpu::CPU;

use super::processor_status::ProcessorStatus;
use super::MOS6502;

fn on_incdec_set_status(proc_status: &mut ProcessorStatus, value: u8) {
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



macro_rules! arit_im {
    ($self:ident, $memory:ident, $call:tt) => {{
        let byte: u8 = $self.fetch($memory);
        $self.$call(byte);
        1
    }};
}

macro_rules! arit_zp {
    ($self:ident, $memory:ident, $call:ident) => {{
        let zero_page_address: u16 = $self.fetch($memory) as u16;
        let value = $self.read(zero_page_address, $memory);
        $self.$call(value);
        2
    }};
}

macro_rules! arit_zpx {
    ($self:ident, $memory:ident, $call:ident) => {{
        let base_address: u8 = $self.fetch($memory);
        let effective_address: u8 = base_address.wrapping_add($self.regX);
        let value: u8 = $memory[effective_address as usize];
        $self.$call(value);
        3
    }};
}

macro_rules! arit_abs {
    ($self:ident, $memory:ident, $call:ident) => {{
        let low_byte: u8 = $self.fetch($memory);
        let high_byte: u8 = $self.fetch($memory);
        let address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let value: u8 = $self.read(address, $memory);
        $self.$call(value);
        3
    }};
}

macro_rules! arit_absx {
    ($self:ident, $memory:ident, $call:ident) => {{
        let low_byte: u8 = $self.fetch($memory);
        let high_byte: u8 = $self.fetch($memory);
        let base_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let effective_address: u16 = base_address.wrapping_add($self.regX as u16);
        let value: u8 = $self.read(effective_address, $memory);
        $self.$call(value);

        if (base_address & 0xFF00) != (effective_address & 0xFF00) {
            4
        } else {
            3
        }
    }};
}

macro_rules! arit_absy {
    ($self:ident, $memory:ident, $call:ident) => {{
        let low_byte: u8 = $self.fetch($memory);
        let high_byte: u8 = $self.fetch($memory);
        let base_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let effective_address: u16 = base_address.wrapping_add($self.regY as u16);
        let value: u8 = $self.read(effective_address, $memory);
        $self.$call(value);

        if (base_address & 0xFF00) != (effective_address & 0xFF00) {
            4
        } else {
            3
        }
    }};
}

macro_rules! arit_indx {
    ($self:ident, $memory:ident, $call:ident) => {{
        let base_address: u8 = $self.fetch($memory);
        let indirect_address: u8 = base_address.wrapping_add($self.regX);
        let low_byte: u8 = $memory[indirect_address as usize];
        let high_byte: u8 = $memory[indirect_address.wrapping_add(1) as usize];
        let final_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let value: u8 = $self.read(final_address, $memory);
        $self.$call(value);
        5
    }};
}

macro_rules! arit_indy {
    ($self:ident, $memory:ident, $call:ident) => {{
        let base_address: u8 = $self.fetch($memory);
        let low_byte: u8 = $memory[base_address as usize];
        let high_byte: u8 = $memory[base_address.wrapping_add(1) as usize];
        let base_address: u16 = ((high_byte as u16) << 8) | (low_byte as u16);
        let effective_address: u16 = base_address.wrapping_add($self.regY as u16);
        let value: u8 = $self.read(effective_address, $memory);
        $self.$call(value);

        if (base_address & 0xFF00) != (effective_address & 0xFF00) {
            5
        } else {
            4
        }
    }};
}

impl MOS6502 {
    // Increment in memory
    pub fn inc_zp(&mut self, memory : &mut Memory) -> u8 {
        let zero_page_address: u16 = self.fetch(memory) as u16;
        let value = self.read(zero_page_address, memory);
        memory[zero_page_address] = value+1;
        on_incdec_set_status(self.proc_status, value+1);
        4
    }

    pub fn inc_zpx(&mut self, memory : &mut Memory) -> u8 {
        let base_address: u8 = self.fetch(memory);
        let effective_address: u8 = base_address.wrapping_add(self.regX);
        let value = self.read(zero_page_address, memory);
        memory[zero_page_address] = value+1;
        on_incdec_set_status(self.proc_status, value+1);
        5
    }

    ...
    
    // Increment registes
    pub fn inx(&mut self) -> u8 {
        self.regX += 1;
        on_incdec_set_status(self.proc_status, self.regX);
        1
    }

    pub fn iny(&mut self) -> u8 {
        self.regy += 1;
        on_incdec_set_status(self.proc_status, self.regY);
        1
    }

    // Decrement in memory

    // Decrement registes
    pub fn dex(&mut self) -> u8 {
        self.regX -= 1;
        on_incdec_set_status(self.proc_status, self.regX);
        1
    }

    pub fn dey(&mut self) -> u8 {
        self.regy -= 1;
        on_incdec_set_status(self.proc_status, self.regY);
        1
    }

}