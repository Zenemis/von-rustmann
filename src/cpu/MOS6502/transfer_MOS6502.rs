use super::processor_status::ProcessorStatus;
use super::MOS6502;

fn on_ts_set_status(proc_status: &mut ProcessorStatus, value: u8) {
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

impl MOS6502 {
    // Transfer from A register
    pub fn tax(&mut self) -> u32 {
        self.regX = self.regA;
        on_ts_set_status(&mut self.proc_status, self.regA);
        1
    }

    pub fn tay(&mut self) -> u32 {
        self.regY = self.regA;
        on_ts_set_status(&mut self.proc_status, self.regA);
        1
    }

    // Transfer from stack pointer
    pub fn tsx(&mut self) -> u32 {
        self.regX = self.regSP;
        on_ts_set_status(&mut self.proc_status, self.regSP);
        1
    }

    // Transfer from X register
    pub fn txa(&mut self) -> u32 {
        self.regA = self.regX;
        on_ts_set_status(&mut self.proc_status, self.regX);
        1
    }

    pub fn txs(&mut self) -> u32 {
        self.regSP = self.regX;
        on_ts_set_status(&mut self.proc_status, self.regX);
        1
    }

    // Transfer from Y register
    pub fn tya(&mut self) -> u32 {
        self.regA = self.regY;
        on_ts_set_status(&mut self.proc_status, self.regY);        
        1
    }
}