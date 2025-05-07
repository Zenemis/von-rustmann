use crate::cpu::memory::Memory; 

use crate::cpu::cpu::CPU;

use super::processor_status::ProcessorStatus;
use super::MOS6502;

fn on_arit_set_status(proc_status: &mut ProcessorStatus, value: u8) {
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
    // Add with carry
    fn adc(&mut self, value: u8) {
        let carry = if self.proc_status.carry() { 1 } else { 0 };
        let result = self.regA as u16 + value as u16 + carry as u16;

        // Update carry flag
        if result > 0xFF {
            self.proc_status.set_carry();
        } else {
            self.proc_status.clear_carry();
        }

        // Update overflow flag
        let a_sign = (self.regA & 0x80) != 0;
        let value_sign = (value & 0x80) != 0;
        let result_sign = ((result as u8) & 0x80) != 0;
        if (a_sign == value_sign) && (a_sign != result_sign) {
            self.proc_status.set_overflow();
        } else {
            self.proc_status.clear_overflow();
        }

        // Update accumulator
        self.regA = result as u8;

        // Update zero and negative flags
        on_arit_set_status(&mut self.proc_status, self.regA);
    }
    
    pub fn adc_im(&mut self, memory : &Memory) -> u32 {
        arit_im!(self, memory, adc)
    }

    pub fn adc_zp(&mut self, memory : &Memory) -> u32 {
        arit_zp!(self, memory, adc)
    }

    pub fn adc_zpx(&mut self, memory : &Memory) -> u32 {
        arit_zpx!(self, memory, adc)
    }

    pub fn adc_abs(&mut self, memory : &Memory) -> u32 {
        arit_abs!(self, memory, adc)
    }

    pub fn adc_absx(&mut self, memory : &Memory) -> u32 {
        arit_absx!(self, memory, adc)
    }

    pub fn adc_absy(&mut self, memory : &Memory) -> u32 {
        arit_absy!(self, memory, adc)
    }

    pub fn adc_indx(&mut self, memory: &Memory) -> u32 {
        arit_indx!(self, memory, adc)
    }

    pub fn adc_indy(&mut self, memory: &Memory) -> u32 {
        arit_indy!(self, memory, adc)
    }

    // Subtract with Carry
    fn sbc(&mut self, value: u8) {
        let carry = if self.proc_status.carry() { 1 } else { 0 };
        let result = self.regA as i16 - value as i16 - (1 - carry as i16);

        // Update carry flag
        if result >= 0 {
            self.proc_status.set_carry();
        } else {
            self.proc_status.clear_carry();
        }

        // Update overflow flag
        let a_sign = (self.regA & 0x80) != 0;
        let value_sign = (value & 0x80) != 0;
        let result_sign = ((result as u8) & 0x80) != 0;
        if (a_sign != value_sign) && (a_sign != result_sign) {
            self.proc_status.set_overflow();
        } else {
            self.proc_status.clear_overflow();
        }

        // Update accumulator
        self.regA = result as u8;

        // Update zero and negative flags
        on_arit_set_status(&mut self.proc_status, self.regA);
    }

    pub fn sbc_im(&mut self, memory: &Memory) -> u32 {
        arit_im!(self, memory, sbc)
    }

    pub fn sbc_zp(&mut self, memory: &Memory) -> u32 {
        arit_zp!(self, memory, sbc)
    }

    pub fn sbc_zpx(&mut self, memory: &Memory) -> u32 {
        arit_zpx!(self, memory, sbc)
    }

    pub fn sbc_abs(&mut self, memory: &Memory) -> u32 {
        arit_abs!(self, memory, sbc)
    }

    pub fn sbc_absx(&mut self, memory: &Memory) -> u32 {
        arit_absx!(self, memory, sbc)
    }

    pub fn sbc_absy(&mut self, memory: &Memory) -> u32 {
        arit_absy!(self, memory, sbc)
    }

    pub fn sbc_indx(&mut self, memory: &Memory) -> u32 {
        arit_indx!(self, memory, sbc)
    }

    pub fn sbc_indy(&mut self, memory: &Memory) -> u32 {
        arit_indy!(self, memory, sbc)
    }

    // Compare
    fn cmp(&mut self, reg: u8, result: u8, value: u8) {

        // Update carry flag
        if reg >= value {
            self.proc_status.set_carry();
        } else {
            self.proc_status.clear_carry();
        }

        // Update zero flag
        if reg == value {
            self.proc_status.set_zero();
        } else {
            self.proc_status.clear_zero();
        }

        // Update negative flag
        if result & 0x80 != 0 {
            self.proc_status.set_negative();
        } else {
            self.proc_status.clear_negative();
        }
    }

    fn cmpa2cmp(&mut self, value: u8) {
        let result = self.regA.wrapping_sub(value);
        self.cmp(self.regA, result, value)
    }

    fn cmpx2cmp(&mut self, value: u8) {
        let result = self.regX.wrapping_sub(value);
        self.cmp(self.regX, result, value)
    }

    fn cmpy2cmp(&mut self, value: u8) {
        let result = self.regY.wrapping_sub(value);
        self.cmp(self.regY, result, value)
    }

    pub fn cmp_im(&mut self, memory: &Memory) -> u32 {
        arit_im!(self, memory, cmpa2cmp)
    }

    pub fn cmp_zp(&mut self, memory: &Memory) -> u32 {
        arit_zp!(self, memory, cmpa2cmp)
    }

    pub fn cmp_zpx(&mut self, memory: &Memory) -> u32 {
        arit_zpx!(self, memory, cmpa2cmp)
    }

    pub fn cmp_abs(&mut self, memory: &Memory) -> u32 {
        arit_abs!(self, memory, cmpa2cmp)
    }

    pub fn cmp_absx(&mut self, memory: &Memory) -> u32 {
        arit_absx!(self, memory, cmpa2cmp)
    }

    pub fn cmp_absy(&mut self, memory: &Memory) -> u32 {
        arit_absy!(self, memory, cmpa2cmp)
    }

    pub fn cmp_indx(&mut self, memory: &Memory) -> u32 {
        arit_indx!(self, memory, cmpa2cmp)
    }

    pub fn cmp_indy(&mut self, memory: &Memory) -> u32 {
        arit_indy!(self, memory, cmpa2cmp)
    }

    pub fn cpx_im(&mut self, memory: &Memory) -> u32 {
        arit_im!(self, memory, cmpx2cmp)
    }

    pub fn cpx_zp(&mut self, memory: &Memory) -> u32 {
        arit_zp!(self, memory, cmpx2cmp)
    }

    pub fn cpx_abs(&mut self, memory: &Memory) -> u32 {
        arit_abs!(self, memory, cmpx2cmp)
    }

    pub fn cpy_im(&mut self, memory: &Memory) -> u32 {
        arit_im!(self, memory, cmpy2cmp)
    }

    pub fn cpy_zp(&mut self, memory: &Memory) -> u32 {
        arit_zp!(self, memory, cmpy2cmp)
    }

    pub fn cpy_abs(&mut self, memory: &Memory) -> u32 {
        arit_abs!(self, memory, cmpy2cmp)
    }
}