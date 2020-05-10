use crate::emulator::bus::*;

pub const UART_SIZE:    usize  = UART0_TOP - UART0_BASE;

pub const TXFIFO_BASE:  usize   = 0x0000;
pub const TXFIFO_TOP:   usize   = 0x0003;
pub const TXFIFO_SIZE:  usize   = 0x0010;

pub const RXFIFO_BASE:  usize   = 0x0004;
pub const RXFIFO_TOP:   usize   = 0x0007;
pub const RXFIFO_SIZE:  usize   = 0x0010;

pub const TXCTRL_BASE:  usize   = 0x0008;
pub const TXCTRL_TOP:   usize   = 0x0009;

pub const TXMARK_BASE:  usize   = 0x000A;
pub const TXMARK_TOP:   usize   = 0x000B;

pub const RXCTRL_BASE:  usize   = 0x000C;
pub const RXCTRL_TOP:   usize   = 0x000D;

pub const RXMARK_BASE:  usize   = 0x000E;
pub const RXMARK_TOP:   usize   = 0x000F;

pub const IE_BASE:      usize   = 0x0010;
pub const IE_TOP:       usize   = 0x0013;

pub const IP_BASE:      usize   = 0x0014;
pub const IP_TOP:       usize   = 0x0017;

pub const DIV_BASE:     usize   = 0x0018;
pub const DIV_TOP:      usize   = 0x0019;

pub const MAX_BASE:     usize   = 0x0020;
pub const MAX_TOP:      usize   = 0x0021;


pub struct Uart {
    uart: Vec<u8>,
    rxfifo: Vec<u8>,
}

impl Uart {
    pub fn new() -> Self {
        Uart {
            uart:   vec![0; UART_SIZE],
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
            TXFIFO_BASE ... TXFIFO_TOP  => {
                print!("{}", data as char);
                // ToDo: update irq
            },
            IE_BASE ... IE_TOP          => {
                self.uart[addr] = data;
                // ToDo: update irq
            },
            TXCTRL_BASE ... TXCTRL_TOP  |
            TXMARK_BASE ... TXMARK_TOP  |
            RXCTRL_BASE ... RXCTRL_TOP  |
            RXMARK_BASE ... RXMARK_TOP  |
            IP_BASE ... IP_TOP          |
            DIV_BASE ... DIV_TOP        |
            MAX_BASE ... MAX_TOP        => self.uart[addr] = data,
            _                           => unimplemented!(),
        }
    }

    pub fn read8(&mut self, addr: usize) -> u8 {
        match addr {
            TXFIFO_BASE ... TXFIFO_TOP  => 0,
            RXFIFO_BASE ... RXFIFO_TOP  => {
                let r = self.rxfifo.remove(0);
                self.rxfifo.push(0);
                // DoDo: update irq

                return r;
            },
            IE_BASE ... IE_TOP          => {
                // ToDo: update irq
                return self.uart[addr];
            },
            TXCTRL_BASE ... TXCTRL_TOP  |
            TXMARK_BASE ... TXMARK_TOP  |
            RXCTRL_BASE ... RXCTRL_TOP  |
            RXMARK_BASE ... RXMARK_TOP  |
            IP_BASE ... IP_TOP          |
            DIV_BASE ... DIV_TOP        |
            MAX_BASE ... MAX_TOP        => self.uart[addr],
            _                           => unimplemented!(),
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