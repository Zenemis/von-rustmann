mod processor_status;

mod load_MOS6502;
mod store_MOS6502;
mod transfer_MOS6502;
mod stack_MOS6502;
mod logical_MOS6502;
mod arithmetic_MOS6502;

use processor_status::ProcessorStatus;

use super::memory::Memory; 
use super::cpu::CPU;

use num_enum::TryFromPrimitive;
use num_enum::IntoPrimitive;
use std::convert::TryFrom;

#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)] // Ensure the enum is represented as a `u8`
enum Instr {
    LDA_IM = 0xA9,
    LDA_ZP = 0xA5,
    LDA_ZPX = 0xB5,
    LDA_ABS = 0xAD,
    LDA_ABSX = 0xBD,
    LDA_ABSY = 0xB9,
    LDA_INDX = 0xA1,
    LDA_INDY = 0xB1,
    LDX_IM = 0xA2,
    LDX_ZP = 0xA6,
    LDX_ZPY = 0xB6,
    LDX_ABS = 0xAE,
    LDX_ABSY = 0xBE,
    LDY_IM = 0xA0,
    LDY_ZP = 0xA4,
    LDY_ZPX = 0xB4,
    LDY_ABS = 0xAC,
    LDY_ABSX = 0xBC,
    STA_ZP = 0x85,
    STA_ZPX = 0x95,
    STA_ABS = 0x8D,
    STA_ABSX = 0x9D,
    STA_ABSY = 0x99,
    STA_INDX = 0x81,
    STA_INDY = 0x91,
    STX_ZP = 0x86,
    STX_ZPY = 0x96,
    STX_ABS = 0x8E,
    STY_ZP = 0x84,
    STY_ZPX = 0x94,
    STY_ABS = 0x8C,
    TAX = 0xAA,
    TAY = 0xA8,
    TSX = 0xBA,
    TXA = 0x8A,
    TXS = 0x9A,
    TYA = 0x98,
    PHA = 0x48,
    PHP = 0x08,
    PLA = 0x68,
    PLP = 0x28,
    AND_IM = 0x29,
    AND_ZP = 0x25,
    AND_ZPX = 0x35,
    AND_ABS = 0x2D,
    AND_ABSX = 0x3D,
    AND_ABSY = 0x39,
    AND_INDX = 0x21,
    AND_INDY = 0x31,
    EOR_IM = 0x49,
    EOR_ZP = 0x45,
    EOR_ZPX = 0x55,
    EOR_ABS = 0x4D,
    EOR_ABSX = 0x5D,
    EOR_ABSY = 0x59,
    EOR_INDX = 0x41,
    EOR_INDY = 0x51,
    ORA_IM = 0x09,
    ORA_ZP = 0x05,
    ORA_ZPX = 0x15,
    ORA_ABS = 0x0D,
    ORA_ABSX = 0x1D,
    ORA_ABSY = 0x19,
    ORA_INDX = 0x01,
    ORA_INDY = 0x11,
    BIT_ZP = 0x24,
    BIT_ABS = 0x2C,
    ADC_IM = 0x69,
    ADC_ZP = 0x65,
    ADC_ZPX = 0x75,
    ADC_ABS = 0x6D,
    ADC_ABSX = 0x7D,
    ADC_ABSY = 0x79,
    ADC_INDX = 0x61,
    ADC_INDY = 0x71,
    SBC_IM = 0xE9,
    SBC_ZP = 0xE5,
    SBC_ZPX = 0xF5,
    SBC_ABS = 0xED,
    SBC_ABSX = 0xFD,
    SBC_ABSY = 0xF9,
    SBC_INDX = 0xE1,
    SBC_INDY = 0xF1,
    CMP_IM = 0xC9,
    CMP_ZP = 0xC5,
    CMP_ZPX = 0xD5,
    CMP_ABS = 0xCD,
    CMP_ABSX = 0xDD,
    CMP_ABSY = 0xD9,
    CMP_INDX = 0xC1,
    CMP_INDY = 0xD1,
    CPX_IM = 0xE0,
    CPX_ZP = 0xE4,
    CPX_ABS = 0xEC,
    CPY_IM = 0xC0,
    CPY_ZP = 0xC4,
    CPY_ABS = 0xCC,
}

pub struct MOS6502 {
    regPC : u16,
    regSP : u8,

    regA : u8,
    regX : u8,
    regY : u8,

    proc_status: ProcessorStatus,
}

impl MOS6502 {
    pub fn new() -> Self {
        MOS6502 {
            regPC : 0xFFFC,
            regSP : 0xFF,
            regA : 0,
            regX : 0,
            regY : 0,
            proc_status : ProcessorStatus::new(),
        }
    }
}

impl CPU for MOS6502 {
    fn fetch(&mut self, memory : &Memory) -> u8 {
        let res = self.read(self.regPC, memory);
        self.regPC += 1;
        res
    }

    fn read(&self, address: u16, memory : &Memory) -> u8 {
       memory[address as usize]
    }

    fn execute(&mut self, mut cycles : u32, memory : &mut Memory) {
        while cycles > 0 {
            let instruction : u8 = self.fetch(memory);
            cycles -= 1;
            match Instr::try_from(instruction) {
                Ok(Instr::LDA_IM) => {
                    cycles -= self.lda_im(memory);
                }
                Ok(Instr::LDA_ZP) => {
                    cycles -= self.lda_zp(memory);
                }
                Ok(Instr::LDA_ZPX) => {
                    cycles -= self.lda_zpx(memory);
                }
                Ok(Instr::LDA_ABS) => {
                    cycles -= self.lda_abs(memory);
                }
                Ok(Instr::LDA_ABSX) => {
                    cycles -= self.lda_absx(memory);
                }
                Ok(Instr::LDA_ABSY) => {
                    cycles -= self.lda_absy(memory);
                }
                Ok(Instr::LDA_INDX) => {
                    cycles -= self.lda_indx(memory);
                }
                Ok(Instr::LDA_INDY) => {
                    cycles -= self.lda_indy(memory);
                }
                Ok(Instr::LDX_IM) => {
                    cycles -= self.ldx_im(memory);
                }
                Ok(Instr::LDX_ZP) => {
                    cycles -= self.ldx_zp(memory);
                }
                Ok(Instr::LDX_ZPY) => {
                    cycles -= self.ldx_zpy(memory);
                }
                Ok(Instr::LDX_ABS) => {
                    cycles -= self.ldx_abs(memory);
                }
                Ok(Instr::LDX_ABSY) => {
                    cycles -= self.ldx_absy(memory);
                }
                Ok(Instr::LDY_IM) => {
                    cycles -= self.ldy_im(memory);
                }
                Ok(Instr::LDY_ZP) => {
                    cycles -= self.ldy_zp(memory);
                }
                Ok(Instr::LDY_ZPX) => {
                    cycles -= self.ldy_zpx(memory);
                }
                Ok(Instr::LDY_ABS) => {
                    cycles -= self.ldy_abs(memory);
                }
                Ok(Instr::LDY_ABSX) => {
                    cycles -= self.ldy_absx(memory);
                }
                Ok(Instr::STA_ZP) => {
                    cycles -= self.sta_zp(memory);
                }
                Ok(Instr::STA_ZPX) => {
                    cycles -= self.sta_zpx(memory);
                }
                Ok(Instr::STA_ABS) => {
                    cycles -= self.sta_abs(memory);
                }
                Ok(Instr::STA_ABSX) => {
                    cycles -= self.sta_absx(memory);
                }
                Ok(Instr::STA_ABSY) => {
                    cycles -= self.sta_absy(memory);
                }
                Ok(Instr::STA_INDX) => {
                    cycles -= self.sta_indx(memory);
                }
                Ok(Instr::STA_INDY) => {
                    cycles -= self.sta_indy(memory);
                }
                Ok(Instr::STX_ZP) => {
                    cycles -= self.stx_zp(memory);
                }
                Ok(Instr::STX_ZPY) => {
                    cycles -= self.stx_zpy(memory);
                }
                Ok(Instr::STX_ABS) => {
                    cycles -= self.stx_abs(memory);
                }
                Ok(Instr::STY_ZP) => {
                    cycles -= self.sty_zp(memory);
                }
                Ok(Instr::STY_ZPX) => {
                    cycles -= self.sty_zpx(memory);
                }
                Ok(Instr::STY_ABS) => {
                    cycles -= self.sty_abs(memory);
                }
                Ok(Instr::TAX) => {
                    cycles -= self.tax();
                }
                Ok(Instr::TAY) => {
                    cycles -= self.tay();
                }
                Ok(Instr::TSX) => {
                    cycles -= self.tsx();
                }
                Ok(Instr::TXA) => {
                    cycles -= self.txa();
                }
                Ok(Instr::TXS) => {
                    cycles -= self.txs();
                }
                Ok(Instr::TYA) => {
                    cycles -= self.tya();
                }
                Ok(Instr::PHA) => {
                    cycles -= self.pha(memory);
                }
                Ok(Instr::PHP) => {
                    cycles -= self.php(memory);
                }
                Ok(Instr::PLA) => {
                    cycles -= self.pla(memory);
                }
                Ok(Instr::PLP) => {
                    cycles -= self.plp(memory);
                }
                Ok(Instr::AND_IM) => {
                    cycles -= self.and_im(memory);
                }
                Ok(Instr::AND_ZP) => {
                    cycles -= self.and_zp(memory);
                }
                Ok(Instr::AND_ZPX) => {
                    cycles -= self.and_zpx(memory);
                }
                Ok(Instr::AND_ABS) => {
                    cycles -= self.and_abs(memory);
                }
                Ok(Instr::AND_ABSX) => {
                    cycles -= self.and_absx(memory);
                }
                Ok(Instr::AND_ABSY) => {
                    cycles -= self.and_absy(memory);
                }
                Ok(Instr::AND_INDX) => {
                    cycles -= self.and_indx(memory);
                }
                Ok(Instr::AND_INDY) => {
                    cycles -= self.and_indy(memory);
                }
                Ok(Instr::EOR_IM) => {
                    cycles -= self.eor_im(memory);
                }
                Ok(Instr::EOR_ZP) => {
                    cycles -= self.eor_zp(memory);
                }
                Ok(Instr::EOR_ZPX) => {
                    cycles -= self.eor_zpx(memory);
                }
                Ok(Instr::EOR_ABS) => {
                    cycles -= self.eor_abs(memory);
                }
                Ok(Instr::EOR_ABSX) => {
                    cycles -= self.eor_absx(memory);
                }
                Ok(Instr::EOR_ABSY) => {
                    cycles -= self.eor_absy(memory);
                }
                Ok(Instr::EOR_INDX) => {
                    cycles -= self.eor_indx(memory);
                }
                Ok(Instr::EOR_INDY) => {
                    cycles -= self.eor_indy(memory);
                }
                Ok(Instr::ORA_IM) => {
                    cycles -= self.ora_im(memory);
                }
                Ok(Instr::ORA_ZP) => {
                    cycles -= self.ora_zp(memory);
                }
                Ok(Instr::ORA_ZPX) => {
                    cycles -= self.ora_zpx(memory);
                }
                Ok(Instr::ORA_ABS) => {
                    cycles -= self.ora_abs(memory);
                }
                Ok(Instr::ORA_ABSX) => {
                    cycles -= self.ora_absx(memory);
                }
                Ok(Instr::ORA_ABSY) => {
                    cycles -= self.ora_absy(memory);
                }
                Ok(Instr::ORA_INDX) => {
                    cycles -= self.ora_indx(memory);
                }
                Ok(Instr::ORA_INDY) => {
                    cycles -= self.ora_indy(memory);
                }
                Ok(Instr::BIT_ZP) => {
                    cycles -= self.bit_zp(memory);
                }
                Ok(Instr::BIT_ABS) => {
                    cycles -= self.bit_abs(memory);
                }
                Ok(Instr::ADC_IM) => {
                    cycles -= self.adc_im(memory);
                }
                Ok(Instr::ADC_ZP) => {
                    cycles -= self.adc_zp(memory);
                }
                Ok(Instr::ADC_ZPX) => {
                    cycles -= self.adc_zpx(memory);
                }
                Ok(Instr::ADC_ABS) => {
                    cycles -= self.adc_abs(memory);
                }
                Ok(Instr::ADC_ABSX) => {
                    cycles -= self.adc_absx(memory);
                }
                Ok(Instr::ADC_ABSY) => {
                    cycles -= self.adc_absy(memory);
                }
                Ok(Instr::ADC_INDX) => {
                    cycles -= self.adc_indx(memory);
                }
                Ok(Instr::ADC_INDY) => {
                    cycles -= self.adc_indy(memory);
                }
                Ok(Instr::SBC_IM) => {
                    cycles -= self.sbc_im(memory);
                }
                Ok(Instr::SBC_ZP) => {
                    cycles -= self.sbc_zp(memory);
                }
                Ok(Instr::SBC_ZPX) => {
                    cycles -= self.sbc_zpx(memory);
                }
                Ok(Instr::SBC_ABS) => {
                    cycles -= self.sbc_abs(memory);
                }
                Ok(Instr::SBC_ABSX) => {
                    cycles -= self.sbc_absx(memory);
                }
                Ok(Instr::SBC_ABSY) => {
                    cycles -= self.sbc_absy(memory);
                }
                Ok(Instr::SBC_INDX) => {
                    cycles -= self.sbc_indx(memory);
                }
                Ok(Instr::SBC_INDY) => {
                    cycles -= self.sbc_indy(memory);
                }
                Ok(Instr::CMP_IM) => {
                    cycles -= self.cmp_im(memory);
                }
                Ok(Instr::CMP_ZP) => {
                    cycles -= self.cmp_zp(memory);
                }
                Ok(Instr::CMP_ZPX) => {
                    cycles -= self.cmp_zpx(memory);
                }
                Ok(Instr::CMP_ABS) => {
                    cycles -= self.cmp_abs(memory);
                }
                Ok(Instr::CMP_ABSX) => {
                    cycles -= self.cmp_absx(memory);
                }
                Ok(Instr::CMP_ABSY) => {
                    cycles -= self.cmp_absy(memory);
                }
                Ok(Instr::CMP_INDX) => {
                    cycles -= self.cmp_indx(memory);
                }
                Ok(Instr::CMP_INDY) => {
                    cycles -= self.cmp_indy(memory);
                }
                Ok(Instr::CPX_IM) => {
                    cycles -= self.cpx_im(memory);
                }
                Ok(Instr::CPX_ZP) => {
                    cycles -= self.cpx_zp(memory);
                }
                Ok(Instr::CPX_ABS) => {
                    cycles -= self.cpx_abs(memory);
                }
                Ok(Instr::CPY_IM) => {
                    cycles -= self.cpy_im(memory);
                }
                Ok(Instr::CPY_ZP) => {
                    cycles -= self.cpy_zp(memory);
                }
                Ok(Instr::CPY_ABS) => {
                    cycles -= self.cpy_abs(memory);
                }
                Err(_) => {
                    println!("Unknown instruction: {:#X}", instruction);
                }
            }
        }
    }
}