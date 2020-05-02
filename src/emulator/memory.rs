pub const MEMORY_SIZE: usize = 1024 * 1024 * 4;     // 4MiB

// Physical Memory
pub struct Memory {
    pub rom: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            rom: vec![0; MEMORY_SIZE],
        }
    }
}