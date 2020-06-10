/*
 * PLIC: Platform-Level Interrupt Controller
 * Reference:   RISC-V Platform-Level Interrupt Controller Specification
 *              https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc
 */

use crate::emulator::bus::*;
use crate::emulator::interrupt::IrqNumber;
use crate::emulator::csr::MIP_SEIP;

pub const PLIC_SIZE: usize = PLIC_TOP - PLIC_BASE;

pub const PRIORITY_BASE:    usize = 0x0000_0004;
pub const PRIORITY_TOP:     usize = 0x0000_0FFF;

pub const PENDING_ARRAY_BASE:   usize = 0x0000_1000;
pub const PENDING_ARRAY_TOP:    usize = 0x0000_107F;

pub const ENABLE_BASE:  usize = 0x0000_2000;
pub const ENABLE_TOP:   usize = 0x001F_2002;

pub const CONTEXT_BASE: usize = 0x0020_0000;
pub const CONTEXT_TOP:  usize = 0x03FF_F007;

pub struct Plic {
    plic:   Vec<u8>,
    clock:  u64,
    irq:    Option<IrqNumber>,
}

impl Plic {
    pub fn new() -> Self {
        Plic {
            plic:   vec![0; PLIC_SIZE],
            clock:  0,
            irq:    None,
        }
    }

    pub fn get_irqno(&self) -> Option<IrqNumber> {
        self.irq
    }

    pub fn tick(&mut self, virtio_is_interrupting: bool, uart_is_interrupting: bool, mip: &mut u64) {
        self.clock = self.clock.wrapping_add(1);

        let virtio_priority = self.read32(PRIORITY_BASE + (IrqNumber::VIRTIO as usize * 4) as usize);
        let uart_priority   = self.read32(PRIORITY_BASE + (IrqNumber::UART as usize * 4) as usize);

        let virtio_enabled  = ((self.read32(ENABLE_BASE) >> IrqNumber::VIRTIO as u32) & 0x1) == 1;
        let uart_enabled    = ((self.read32(ENABLE_BASE) >> IrqNumber::UART as u32) & 0x1) == 1;

        let threshold       = self.read32(CONTEXT_BASE);
        let interruptings   = [virtio_is_interrupting, uart_is_interrupting];
        let enables         = [virtio_enabled, uart_enabled];
        let priorities      = [virtio_priority, uart_priority];
        let irqs            = [IrqNumber::VIRTIO, IrqNumber::UART];

        let mut irq         = None;
        let mut priority    = 0;

        for i in 0..2 {
            if  interruptings[i] && enables[i] &&
                priorities[i] > threshold &&
                priorities[i] > priority {
                    irq         = Some(irqs[i]);
                    priority    = priorities[i];
                }
        }

        if irq != None {
            self.irq = irq;
            *mip |= MIP_SEIP;
        }
    }

    pub fn write8(&mut self, addr: usize, data: u8) {
        match addr {
            PRIORITY_BASE ..= PRIORITY_TOP              |
            PENDING_ARRAY_BASE ..= PENDING_ARRAY_TOP    |
            ENABLE_BASE ..= ENABLE_TOP                  |
            CONTEXT_BASE ..= CONTEXT_TOP                => self.plic[addr] = data,
            _                                           => unimplemented!(),
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
            PRIORITY_BASE ..= PRIORITY_TOP              |
            PENDING_ARRAY_BASE ..= PENDING_ARRAY_TOP    |
            ENABLE_BASE ..= ENABLE_TOP                  |
            CONTEXT_BASE ..= CONTEXT_TOP                => self.plic[addr],
            _                                           => unimplemented!(),
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