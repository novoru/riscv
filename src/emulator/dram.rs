pub const DRAM_SIZE: usize = 1024 * 1024 * 128;     // 128MiB

pub struct Dram {
    dram: Vec<u8>,
}

impl Dram {
    pub fn new() -> Self {
        Dram {
            dram: vec![0; DRAM_SIZE],
        }
    }

    pub fn load(&mut self, binary: Vec<u8>) {
        if binary.len() > DRAM_SIZE {
            panic!("[ERROR] too large binary: {}[Byte] (limit: {}[Byte])", binary.len(), DRAM_SIZE);
        }

        for (i, byte) in binary.iter().enumerate() {
            self.dram[i] = *byte;
        }
    }

    pub fn read8(&self, paddr: usize) -> u8 {
        self.dram[paddr]
    }
    
    pub fn read16(&self, paddr: usize) -> u16 {
        self.read8(paddr) as u16 | (self.read8(paddr + 1)  as u16) << 8
    }

    pub fn read32(&self, paddr: usize) -> u32 {
        self.read16(paddr) as u32 | (self.read16(paddr + 2)  as u32) << 16
    }
    
    pub fn read64(&self, paddr: usize) -> u64 {
        self.read32(paddr) as u64 | (self.read32(paddr + 4) as u64) << 32
    }

    pub fn write8(&mut self, paddr: usize, data: u8) {
        self.dram[paddr] = data;
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
}