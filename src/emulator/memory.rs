pub const MEMORY_SIZE: usize = 1024 * 1024 * 4;     // 4MiB

// Physical Memory
pub struct Memory {
    rom: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            rom: vec![0; MEMORY_SIZE],
        }
    }

    pub fn read8(&mut self, paddr: usize) -> u8 {
        self.rom[paddr]
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

    pub fn write8(&mut self, paddr: usize, data: u8) {
        self.rom[paddr] = data;
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
        eprintln!("  [INFO] data = 0x{:16x}", data);
        self.write32(paddr, (data & 0xFFFF_FFFF) as u32);
        self.write32(paddr + 4, ((data >> 32) & 0xFFFF_FFFF) as u32);
    }
}