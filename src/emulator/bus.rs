use crate::emulator::dram::*;
use crate::emulator::clint::*;
use crate::emulator::plic::*;
use crate::emulator::uart::*;
use crate::emulator::virtio::*;
use crate::emulator::interrupt::IrqNumber;

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
    clock:  u64,
    dram:   Dram,
    clint:  Clint,
    pub plic:   Plic,
    uart0:  Uart,
    virtio: Virtio,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            clock:  0,
            dram:   Dram::new(),
            clint:  Clint::new(),
            plic:   Plic::new(),
            uart0:  Uart::new(),
            virtio: Virtio::new(DeviceID::BlockDevice),
        }
    }

    pub fn tick(&mut self, mip: &mut u64) {

        self.clint.tick(mip);
        //self.virtio.tick(&mut self.dram);
        self.uart0.tick();
        self.plic.tick(false, self.uart0.is_interrupting(), mip);
        self.clock = self.clock.wrapping_add(1);
    }

    pub fn get_irqno(&self) -> Option<IrqNumber> {
        self.plic.get_irqno()
    }
    
    pub fn write8(&mut self, paddr: usize, data: u8) {
        match paddr {
            // boot ROM
            BOOT_ROM_BASE ..= BOOT_ROM_TOP  => unimplemented!(),
            // CLINT
            CLINT_BASE ..= CLINT_TOP        => self.clint.write8(paddr - CLINT_BASE, data),
            // PLIC
            PLIC_BASE ..= PLIC_TOP          => self.plic.write8(paddr - PLIC_BASE, data),
            // UART0
            UART0_BASE ..= UART0_TOP        => self.uart0.write8(paddr - UART0_BASE, data),
            // VIRTIO
            VIRTIO_BASE ..= VIRTIO_TOP      => self.virtio.write8(paddr - VIRTIO_BASE, data),
            // DRAM
            DRAM_BASE ..= DRAM_TOP          => self.dram.write8(paddr - DRAM_BASE, data),
            _                               => unimplemented!(),
        }
    }

    pub fn write16(&mut self, paddr: usize, data: u16) {
        match paddr {
            // boot ROM
            BOOT_ROM_BASE ..= BOOT_ROM_TOP  => unimplemented!(),
            // CLINT
            CLINT_BASE ..= CLINT_TOP        => self.clint.write16(paddr - CLINT_BASE, data),
            // PLIC
            PLIC_BASE ..= PLIC_TOP          => self.plic.write16(paddr - PLIC_BASE, data),
            // UART0
            UART0_BASE ..= UART0_TOP        => self.uart0.write16(paddr - UART0_BASE, data),
            // VIRTIO
            VIRTIO_BASE ..= VIRTIO_TOP      => self.virtio.write16(paddr - VIRTIO_BASE, data),
            // DRAM
            DRAM_BASE ..= DRAM_TOP          => self.dram.write16(paddr - DRAM_BASE, data),
            _                               => unimplemented!(),
        }
    }
    
    pub fn write32(&mut self, paddr: usize, data: u32) {
        match paddr {
            // boot ROM
            BOOT_ROM_BASE ..= BOOT_ROM_TOP  => unimplemented!(),
            // CLINT
            CLINT_BASE ..= CLINT_TOP        => self.clint.write32(paddr - CLINT_BASE, data),
            // PLIC
            PLIC_BASE ..= PLIC_TOP          => self.plic.write32(paddr - PLIC_BASE, data),
            // UART0
            UART0_BASE ..= UART0_TOP        => self.uart0.write32(paddr - UART0_BASE, data),
            // VIRTIO
            VIRTIO_BASE ..= VIRTIO_TOP      => self.virtio.write32(paddr - VIRTIO_BASE, data),
            // DRAM
            DRAM_BASE ..= DRAM_TOP          => self.dram.write32(paddr - DRAM_BASE, data),
            _                               => unimplemented!(),
        }
    }
    
    pub fn write64(&mut self, paddr: usize, data: u64) {
        match paddr {
            // boot ROM
            BOOT_ROM_BASE ..= BOOT_ROM_TOP  => unimplemented!(),
            // CLINT
            CLINT_BASE ..= CLINT_TOP        => self.clint.write64(paddr - CLINT_BASE, data),
            // PLIC
            PLIC_BASE ..= PLIC_TOP          => self.plic.write64(paddr - PLIC_BASE, data),
            // UART0
            UART0_BASE ..= UART0_TOP        => self.uart0.write64(paddr - UART0_BASE, data),
            // VIRTIO
            VIRTIO_BASE ..= VIRTIO_TOP      => self.virtio.write64(paddr - VIRTIO_BASE, data),
            // DRAM
            DRAM_BASE ..= DRAM_TOP          => self.dram.write64(paddr - DRAM_BASE, data),
            _                               => unimplemented!(),
        }
    }

    pub fn read8(&mut self, paddr: usize) -> u8 {
        match paddr {
            // boot ROM
            BOOT_ROM_BASE ..= BOOT_ROM_TOP  => unimplemented!(),
            // CLINT
            CLINT_BASE ..= CLINT_TOP        => self.clint.read8(paddr - CLINT_BASE),
            // PLIC
            PLIC_BASE ..= PLIC_TOP          => self.plic.read8(paddr - PLIC_BASE),
            // UART0
            UART0_BASE ..= UART0_TOP        => self.uart0.read8(paddr - UART0_BASE),
            // VIRTIO
            VIRTIO_BASE ..= VIRTIO_TOP      => self.virtio.read8(paddr - VIRTIO_BASE),
            // DRAM
            DRAM_BASE ..= DRAM_TOP          => self.dram.read8(paddr - DRAM_BASE),
            _                               => panic!("invalid paddr: 0x{:016x}", paddr),
        }
    }
    
    pub fn read16(&mut self, paddr: usize) -> u16 {
        match paddr {
            // boot ROM
            BOOT_ROM_BASE ..= BOOT_ROM_TOP  => unimplemented!(),
            // CLINT
            CLINT_BASE ..= CLINT_TOP        => self.clint.read16(paddr - CLINT_BASE),
            // PLIC
            PLIC_BASE ..= PLIC_TOP          => self.plic.read16(paddr - PLIC_BASE),
            // UART0
            UART0_BASE ..= UART0_TOP        => self.uart0.read16(paddr - UART0_BASE),
            // VIRTIO
            VIRTIO_BASE ..= VIRTIO_TOP      => self.virtio.read16(paddr - VIRTIO_BASE),
            // DRAM
            DRAM_BASE ..= DRAM_TOP          => self.dram.read16(paddr - DRAM_BASE),
            _                               => panic!("invalid paddr: 0x{:016x}", paddr),
        }
    }

    pub fn read32(&mut self, paddr: usize) -> u32 {
        match paddr {
            // boot ROM
            BOOT_ROM_BASE ..= BOOT_ROM_TOP  => unimplemented!(),
            // CLINT
            CLINT_BASE ..= CLINT_TOP        => self.clint.read32(paddr - CLINT_BASE),
            // PLIC
            PLIC_BASE ..= PLIC_TOP          => self.plic.read32(paddr - PLIC_BASE),
            // UART0
            UART0_BASE ..= UART0_TOP        => self.uart0.read32(paddr - UART0_BASE),
            // VIRTIO
            VIRTIO_BASE ..= VIRTIO_TOP      => self.virtio.read32(paddr - VIRTIO_BASE),
            // DRAM
            DRAM_BASE ..= DRAM_TOP          => self.dram.read32(paddr - DRAM_BASE),
            _                               => panic!("invalid paddr: 0x{:016x}", paddr),
        }
    }
    
    pub fn read64(&mut self, paddr: usize) -> u64 {
        match paddr {
            // boot ROM
            BOOT_ROM_BASE ..= BOOT_ROM_TOP  => unimplemented!(),
            // CLINT
            CLINT_BASE ..= CLINT_TOP        => self.clint.read64(paddr - CLINT_BASE),
            // PLIC
            PLIC_BASE ..= PLIC_TOP          => self.plic.read64(paddr - PLIC_BASE),
            // UART0
            UART0_BASE ..= UART0_TOP        => self.uart0.read64(paddr - UART0_BASE),
            // VIRTIO
            VIRTIO_BASE ..= VIRTIO_TOP      => self.virtio.read64(paddr - VIRTIO_BASE),
            // DRAM
            DRAM_BASE ..= DRAM_TOP          => self.dram.read64(paddr - DRAM_BASE),
            _                               => panic!("invalid paddr: 0x{:016x}", paddr),
        }
    }

}