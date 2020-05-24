use crate::emulator::dram::*;
use crate::emulator::clint::*;
use crate::emulator::plic::*;
use crate::emulator::uart::*;

/*
 * Physical Address Layout
 * 
 * +-------------+-------------+---------------+
 * |     Base    |     Top     | Description   |
 * +-------------+-------------+---------------+
 * | 0x0000_1000 | 0x0000_10FF | boot ROM      |
 * | 0x0000_1100 | 0x01FF_FFFF | Reserved      |
 * | 0x0200_0000 | 0x0200_FFFF | CLINT         |
 * | 0x0201_0000 | 0x0BFF_FFFF | Reserved      |
 * | 0x0c00_0000 | 0x0FFF_FFFF | PLIC          |
 * | 0x1000_0000 | 0x1000_00FF | UART0         |
 * | 0x1000_1000 | 0x1000_1FFF | VIRTIO        |
 * | 0x1000_2000 | 0x7FFF_FFFF | Reserved      |
 * | 0x8000_0000 | 0x87FF_FFFF | DRAM (128MiB) |
 * | 0x8800_0000 | 0xFFFF_FFFF | Reserved      |
 * +-------------+-------------+---------------+
 * 
 */

pub const BOOT_ROM_BASE:    usize = 0x000_1000;
pub const BOOT_ROM_TOP:     usize = 0x000_10FF;

pub const CLINT_BASE:    usize = 0x0200_0000;
pub const CLINT_TOP:     usize = 0x0200_FFFF;

pub const PLIC_BASE:    usize = 0x0C00_0000;
pub const PLIC_TOP:     usize = 0x0FFF_FFFF;

pub const UART0_BASE:    usize = 0x1000_0000;
pub const UART0_TOP:     usize = 0x1000_00FF;

pub const VIRTIO_BASE:    usize = 0x1000_1000;
pub const VIRTIO_TOP:     usize = 0x1000_1FFF;

pub const DRAM_BASE:    usize = 0x8000_0000;
pub const DRAM_TOP:     usize = 0x87FF_FFFF;


pub struct Bus {
    dram:   Dram,
    clint:  Clint,
    plic:   Plic,
    uart0:  Uart,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            dram:   Dram::new(),
            clint:  Clint::new(),
            plic:   Plic::new(),
            uart0:  Uart::new(),
        }
    }

    pub fn write8(&mut self, paddr: usize, data: u8) {
        match paddr {
            // boot ROM
            BOOT_ROM_BASE ... BOOT_ROM_TOP  => unimplemented!(),
            // CLINT
            CLINT_BASE ... CLINT_TOP        => self.clint.write8(paddr - CLINT_BASE, data),
            // PLIC
            PLIC_BASE ... PLIC_TOP          => self.plic.write8(paddr - PLIC_BASE, data),
            // UART0
            UART0_BASE ... UART0_TOP        => self.uart0.write8(paddr - UART0_BASE, data),
            // VIRTIO
            VIRTIO_BASE ... VIRTIO_TOP      => unimplemented!(),
            // DRAM
            DRAM_BASE ... DRAM_TOP          => self.dram.write8(paddr - DRAM_BASE, data),
            _                               => unimplemented!(),
        }
    }

    pub fn write16(&mut self, paddr: usize, data: u16) {
        self.write8(paddr, (data & 0xFF) as u8);
        self.write8(paddr + 1, ((data >> 8) & 0xFF) as u8);
    }
    
    pub fn write32(&mut self, paddr: usize, data: u32) {
        self.write16(paddr, (data & 0xFFFF) as u16);
        self.write16(paddr + 2, ((data >> 16) & 0xFFFF) as u16);
    }
    
    pub fn write64(&mut self, paddr: usize, data: u64) {
        self.write32(paddr, (data & 0xFFFF_FFFF) as u32);
        self.write32(paddr + 4, ((data >> 32) & 0xFFFF_FFFF) as u32);
    }

    pub fn read8(&mut self, paddr: usize) -> u8 {
        match paddr {
            // boot ROM
            BOOT_ROM_BASE ... BOOT_ROM_TOP  => unimplemented!(),
            // CLINT
            CLINT_BASE ... CLINT_TOP        => self.clint.read8(paddr - CLINT_BASE),
            // PLIC
            PLIC_BASE ... PLIC_TOP          => self.plic.read8(paddr - PLIC_BASE),
            // UART0
            UART0_BASE ... UART0_TOP        => self.uart0.read8(paddr - UART0_BASE),
            // VIRTIO
            VIRTIO_BASE ... VIRTIO_TOP      => unimplemented!(),
            // DRAM
            DRAM_BASE ... DRAM_TOP          => self.dram.read8(paddr - DRAM_BASE),
            _                               => panic!("invalid paddr: 0x{:016x}", paddr),
        }
    }
    
    pub fn read16(&mut self, paddr: usize) -> u16 {
        self.read8(paddr) as u16 | (self.read8(paddr + 1)  as u16) << 8
    }

    pub fn read32(&mut self, paddr: usize) -> u32 {
        self.read16(paddr) as u32 | (self.read16(paddr + 2)  as u32) << 16
    }
    
    pub fn read64(&mut self, paddr: usize) -> u64 {
        self.read32(paddr) as u64 | (self.read32(paddr + 4) as u64) << 32
    }
}