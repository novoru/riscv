use crate::emulator::mmu::Mmu;

type Instruction    = u64;

const NREGISTERS: usize = 32;
const INIT_PC: usize = 0;

pub struct Cpu {
    pub register: [u64; NREGISTERS],        // X Registers
    pub instruction: Instruction,           // Current istruciton
    pub pc: usize,                          // Program counter
    pub mmu: Mmu,                           // MMU (Memory Management Unit)
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register: [0; NREGISTERS],
            instruction: 0,
            pc: INIT_PC,
            mmu: Mmu::new()
        }
    }

    pub fn fetch(&mut self) {
        self.instruction = self.mmu.read64(self.pc);
        self.pc += 4;
    }

    pub fn execute(&mut self) {
        let opcode: u8 = (self.instruction & 0x7F) as u8;
        
        match opcode {
            // R-type
            0b0110011   => self.decode_rtype(),
            // I-type
            0b0010011   => self.decode_itype(),
            _           => unimplemented!(),
        }
    }

    fn decode_rtype(&mut self) {
        // Decode instruction
        let funct7: u8      = ((self.instruction >> 25) & 0x7F) as u8;
        let rs2:    usize   = ((self.instruction >> 20) & 0x1F) as usize;
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;
        let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;

        match funct7 {
            0b0000000 => {    
                match funct3 {
                    // ADD
                    0b000   => self.register[rd] = self.register[rs1] + self.register[rs2],
                    _       => unimplemented!(),
                }
            },
            _               => unimplemented!(),
        }
    }

    fn decode_itype(&mut self) {
        // Decode instruction
        let imm:    i16     = ((self.instruction >> 20) & 0xFFF) as i16;
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;
        let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;

        match funct3 {
            // ADDI
            0b000   => self.register[rd] = ((self.register[rs1] as i64) + (imm as i64)) as u64,
            _       => unimplemented!(),
        }
    }
        
}