/* 
 * Core-Local Interruptor (CLINT)
 * The CLINT block holds memory-mapped control and status registers associated with software and timer interrupts
 */

use crate::emulator::bus::*;

/*
 * CLINT memory Layout
 * 
 * +-------------+-------------+---------------+
 * |     Base    |     Top     | Description   |
 * +-------------+-------------+---------------+
 * | 0x0200_0000 | 0x0200_0007 | MSIP          |
 * | 0x0200_0008 | 0x0200_3FFF | Reserved      |
 * | 0x0200_4000 | 0x0200_4007 | MTIMECMP      |
 * | 0x0200_4008 | 0x0200_BFF7 | Reserved      |
 * | 0x0200_BFF8 | 0x0200_BFFF | MTIME         |
 * | 0x0200_C000 | 0x0200_FFFF | Reserved      |
 * +-------------+-------------+---------------+
 * 
 */

pub const CLINT_SIZE: usize = CLINT_TOP - CLINT_BASE;

// msip for hart 0
pub const MSIP_BASE:        usize = 0x0000;
pub const MSIP_TOP:         usize = 0x0007;

// mtimecmp for hart 0
pub const MTIMECMP_BASE:    usize = 0x4000;
pub const MTIMECMP_TOP:     usize = 0x4008;

pub const MTIME_BASE:       usize = 0xBFF8;
pub const MTIME_TOP:        usize = 0xC000;

pub struct Clint {
    clint: Vec<u8>,
}

impl Clint {
    pub fn new() -> Self {
        Clint {
            clint: vec![0; CLINT_SIZE],
        }
    }

    pub fn write8(&mut self, addr: usize, data: u8) {
        match addr {
            MSIP_BASE ..= MSIP_TOP          => self.write_msip(addr, data),
            MTIMECMP_BASE ..= MTIMECMP_TOP  |
            MTIME_BASE ..= MTIME_TOP        => self.clint[addr] = data,
            _                               => unimplemented!(),
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

    pub fn read8(&self, addr: usize) -> u8 {
        match addr {
            MSIP_BASE ..= MSIP_TOP          => self.read_msip(addr),
            MTIMECMP_BASE ..= MTIMECMP_TOP  |
            MTIME_BASE ..= MTIME_TOP        => self.clint[addr],
            _                               => unimplemented!(),
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

    fn write_msip(&mut self, addr: usize, data: u8) {
        match addr {
            MSIP_BASE   => self.clint[addr] = data & 0x1,   // The least significant bit is reflected in the MSIP bit of the mip CSR
            _           => (),                              // Other bits in the msip register are hardwired to zero. 
        }
    }

    fn read_msip(&self, addr: usize) -> u8 {
        match addr {
            MSIP_BASE   => { return self.clint[addr] & 0x1; },  // The least significant bit is reflected in the MSIP bit of the mip CSR
            _           => 0,                                   // Other bits in the msip register are hardwired to zero. 
        }
    }
}