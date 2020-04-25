const MEMORY_SIZE: usize = 1024 * 4;     // 4KiB

// Physical Memory
pub struct Memory {
    pub rom: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            rom: [0; MEMORY_SIZE],
        }
    }
}