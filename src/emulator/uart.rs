use crate::emulator::bus::*;

pub const UART_SIZE:    usize   = UART0_TOP - UART0_BASE;
pub const RXFIFO_SIZE:  usize   = 8;

// Write mode
pub const THR: usize    = 0b000;    // Transmit Holding Register
pub const IER: usize    = 0b001;    // Interrupt Enable Register
pub const FCR: usize    = 0b010;    // FIFO Control Register
pub const LCR: usize    = 0b011;    // Line Control Register
pub const MCR: usize    = 0b100;    // Modem Control Register

// Read mode
pub const RHR: usize    = 0b000;    // Receive Holding Register
pub const ISR: usize    = 0b010;    // Interrupt Status Register
pub const LSR: usize    = 0b101;    // Line Status Register
pub const MSR: usize    = 0b110;    // Modem Status Register

// Read/Write
pub const SPR: usize    = 0b111;    // Scatchpad Register

pub struct Uart {
    uart: Vec<u8>,
    rxfifo: Vec<u8>,
}

impl Uart {
    pub fn new() -> Self {
        let mut uart = vec![0; UART_SIZE];
        uart[LSR]   |= 1 << 5;

        Uart {
            uart:   uart,
            rxfifo: vec![0; RXFIFO_SIZE],
        }
    }

    pub fn write16(&mut self, addr: usize, data: u16) {
        self.write8(addr, (data & 0xFF) as u8);
        self.write8(addr + 1, ((data >> 8) & 0xFF) as u8);
    }

    pub fn write32(&mut self, addr: usize, data: u32) {
        self.write16(addr, (data & 0xFFFF) as u16);
        self.write16(addr + 2, ((data >> 16) & 0xFFFF) as u16);
    }
    
    pub fn write64(&mut self, addr: usize, data: u64) {
        self.write32(addr, (data & 0xFFFF_FFFF) as u32);
        self.write32(addr + 4, ((data >> 32) & 0xFFFF_FFFF) as u32);
    }

    pub fn write8(&mut self, addr: usize, data: u8) {
        match addr {
            THR => {
                print!("{}", data as char);
                // ToDo: update irq
            },
            IER |
            FCR |
            LCR |
            MCR |
            SPR => self.uart[addr] = data,
            _   => unimplemented!(),
        }
    }

    pub fn read8(&mut self, addr: usize) -> u8 {
        match addr {
            RHR => {
                let r = self.rxfifo.remove(0);
                self.rxfifo.push(0);
                // DoDo: update irq

                return r;
            },
            ISR |
            LSR |
            MSR |
            SPR => self.uart[addr],
            _   => unimplemented!(),
        }
    }
    
    pub fn read16(&mut self, addr: usize) -> u16 {
        self.read8(addr) as u16 | (self.read8(addr + 1)  as u16) << 8
    }

    pub fn read32(&mut self, addr: usize) -> u32 {
        self.read16(addr) as u32 | (self.read16(addr + 2)  as u32) << 16
    }
    
    pub fn read64(&mut self, addr: usize) -> u64 {
        self.read32(addr) as u64 | (self.read32(addr + 4) as u64) << 32
    }
}