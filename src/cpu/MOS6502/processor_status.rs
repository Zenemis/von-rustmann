#[derive(Copy, Clone)]
pub struct ProcessorStatus {
    status: u8, // 8-bit status register
}

impl ProcessorStatus {
    pub fn new() -> Self {
        ProcessorStatus { status: 0 }
    }

    // Carry Flag (bit 0)
    pub fn set_carry(&mut self) {
        self.status |= 1 << 0;
    }

    pub fn clear_carry(&mut self) {
        self.status &= !(1 << 0);
    }

    pub fn carry(&self) -> bool {
        self.status & (1 << 0) != 0
    }

    // Zero Flag (bit 1)
    pub fn set_zero(&mut self) {
        self.status |= 1 << 1;
    }

    pub fn clear_zero(&mut self) {
        self.status &= !(1 << 1);
    }

    pub fn zero(&self) -> bool {
        self.status & (1 << 1) != 0
    }

    // Interrupt Disable (bit 2)
    pub fn set_interrupt_disable(&mut self) {
        self.status |= 1 << 2;
    }

    pub fn clear_interrupt_disable(&mut self) {
        self.status &= !(1 << 2);
    }

    pub fn interrupt_disable(&self) -> bool {
        self.status & (1 << 2) != 0
    }

    // Decimal Mode (bit 3)
    pub fn set_decimal_mode(&mut self) {
        self.status |= 1 << 3;
    }

    pub fn clear_decimal_mode(&mut self) {
        self.status &= !(1 << 3);
    }

    pub fn decimal_mode(&self) -> bool {
        self.status & (1 << 3) != 0
    }

    // Break Command (bit 4)
    pub fn set_break_command(&mut self) {
        self.status |= 1 << 4;
    }

    pub fn clear_break_command(&mut self) {
        self.status &= !(1 << 4);
    }

    pub fn break_command(&self) -> bool {
        self.status & (1 << 4) != 0
    }

    // Overflow Flag (bit 6)
    pub fn set_overflow(&mut self) {
        self.status |= 1 << 6;
    }

    pub fn clear_overflow(&mut self) {
        self.status &= !(1 << 6);
    }

    pub fn overflow(&self) -> bool {
        self.status & (1 << 6) != 0
    }

    // Negative Flag (bit 7)
    pub fn set_negative(&mut self) {
        self.status |= 1 << 7;
    }

    pub fn clear_negative(&mut self) {
        self.status &= !(1 << 7);
    }

    pub fn negative(&self) -> bool {
        self.status & (1 << 7) != 0
    }
}

impl From<ProcessorStatus> for u8 {
    fn from(status: ProcessorStatus) -> Self {
        status.status
    }
}

impl From<&ProcessorStatus> for u8 {
    fn from(status: &ProcessorStatus) -> Self {
        status.status
    }
}

impl From<u8> for ProcessorStatus {
    fn from(value: u8) -> Self {
        ProcessorStatus { status: value }
    }
}