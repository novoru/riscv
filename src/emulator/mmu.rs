use crate::emulator::memory::Memory;

// Memory Management Unit
pub struct Mmu {
    pub memory: Memory,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            memory: Memory::new()
        }
    }

    pub fn read8(& self, vaddr: usize) -> u8 {
        self.memory.rom[vaddr]
    }
    
    pub fn read16(& self, vaddr: usize) -> u16 {
        self.read8(vaddr) as u16 | (self.read8(vaddr + 1)  as u16) << 8
    }

    pub fn read32(& self, vaddr: usize) -> u32 {
        self.read16(vaddr) as u32 | (self.read16(vaddr + 2)  as u32) << 16
    }
    
    pub fn read64(& self, vaddr: usize) -> u64 {
        self.read32(vaddr) as u64 | (self.read32(vaddr + 4) as u64) << 32
    }

    pub fn write8(& mut self, vaddr: usize, data: u8) {
        self.memory.rom[vaddr] = data;
    }

    pub fn write16(&mut self, vaddr: usize, data: u16) {
        self.write8(vaddr, (data & 0xFF) as u8);
        self.write8(vaddr + 1, (data >> 8 & 0xFF) as u8);
    }

    pub fn write32(&mut self, vaddr: usize, data: u32) {
        self.write16(vaddr, (data & 0xFFFF) as u16);
        self.write16(vaddr + 2, (data >> 16 & 0xFFFF) as u16);
    }

    pub fn write64(&mut self, vaddr: usize, data: u64) {
        self.write32(vaddr, (data & 0xFFFF_FFFF) as u32);
        self.write32(vaddr + 4, (data >> 32 & 0xFFFF_FFFF) as u32);
    }

}