use crate::emulator::mmu::Mmu;
use crate::emulator::memory::MEMORY_SIZE;
use crate::emulator::csr::{ Csr, PrivLevel};
use crate::emulator::exception::Exception;
use std::fs::read;

type Instruction    = u64;

const NREGISTERS:   usize = 32;
const INIT_PC:      usize = 0;

// General Registers (standardized names as part of the RISC-V application binary interface (ABI))
pub enum Registers {
    ZERO,   // x0:  hardwired to 0, ignores writes
    RA,     // x1:  return address for jumps
    SP,     // x2:  stack pointer
    GP,     // x3:  global pointer
    TP,     // x4:  thread pointer
    T0,     // x5:  temporary register 0
    T1,     // x6:  temporary register 1
    T2,     // x7:  temporary register 2
    FP,     // x8:  frame pointer
    S1,     // x9:  saved register 1
    A0,     // x10: return value or function argument 0
    A1,     // x11: return value or function argument 1
    A2,     // x12: function argument 2
    A3,     // x13: function argument 3
    A4,     // x14: function argument 4
    A5,     // x15: function argument 5
    A6,     // x16: function argument 6
    A7,     // x17: function argument 7
    S2,     // x18: saved regiser 2
    S3,     // x19: saved regiser 3
    S4,     // x20: saved regiser 4
    S5,     // x21: saved regiser 5
    S6,     // x22: saved regiser 6
    S7,     // x23: saved regiser 7
    S8,     // x24: saved regiser 8
    S9,     // x25: saved regiser 9
    S10,    // x26: saved regiser 10
    S11,    // x27: saved regiser 11
    T3,     // x28: temporary register 3
    T4,     // x29: temporary register 4
    T5,     // x30: temporary register 5
    T6,     // x31: temporary register 6
}

pub struct Cpu {
    pub register: [u64; NREGISTERS],        // General registers
    pub instruction: Instruction,           // Current instruction
    pub pc: usize,                          // Program counter
    pub mmu: Mmu,                           // MMU (Memory Management Unit)
    pub csr: Csr, 
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register: [0; NREGISTERS],
            instruction: 0,
            pc: INIT_PC,
            mmu: Mmu::new(),
            csr: Csr::new(),
        }
    }

    pub fn load(&mut self, filename: &String) -> usize {
        let binary = read(filename).unwrap();
        let len = binary.len();

        if len > MEMORY_SIZE {
            panic!("[ERROR] too large binary({}): {} Byte", filename, len);
        }
        
        for (i, byte) in binary.iter().enumerate() {
            self.mmu.write8(i, *byte);
        }

        len
    }

    pub fn run(&mut self) {
        loop {
            match self.fetch() {
                Ok(_)           => {},
                Err(exception)  => exception.take_trap(self),
            }
            
            match self.execute() {
                Ok(_)           => {},
                Err(exception)  => exception.take_trap(self),
            }

            self.pc += 4;
        }
    }

    pub fn fetch(&mut self) -> Result<(), Exception> {
        self.instruction = self.mmu.read64(self.csr, self.pc)?;
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), Exception> {
        // Decode instruction
        let imm:    u32 = ((self.instruction >> 12) & 0xF_FFFF) as u32;
        let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;
        let opcode: u8 = (self.instruction & 0x7F) as u8;
        
        match opcode {
            // LOAD
            0b000_0011  => self.decode_load()?,
            // STORE
            0b010_0011  => self.decode_store()?,
            // R-type
            0b011_0011  => self.decode_rtype()?,
            // I-type
            0b001_0011  => self.decode_itype()?,
            // LUI
            0b011_0111  => self.register[rd] = ((imm & 0xF_FFFF) << 12) as u64,
            // AUIPC
            0b001_0111  => self.register[rd] = ((imm & 0xF_FFFF) << 12) as u64 + self.pc as u64,
            // JAL
            0b110_1111  => {
                // signed offset in multiples of 2 bytes
                let mut offset: i32   = (((self.instruction & 0x8000_0000) >> 11) |     // imm[20]
                                         ((self.instruction & 0x7FE0_0000) >> 20) |     // imm[10:1]
                                         ((self.instruction & 0x100000)    >>  9) |     // imm[11]
                                          (self.instruction  & 0xFF000)) as i32;        // imm[19:12]
                offset = ((offset + (0b1000_0000_0000_0000)) & (0xFFFFF)) - 0b1000_0000_0000_0000;        // sign extention
                self.register[rd] = self.pc as u64 + 4;
                self.pc = (self.pc as i64 + offset as i64) as usize;
                if self.pc == 0 {
                    std::process::exit(0);
                }
                self.fetch()?;
                self.execute()?;
            },
            // B-type
            0b110_0011  => self.decode_btype()?,
            // JALR
            0b110_0111  => {
                // Decode instruction
                let mut imm:    i16 = ((self.instruction >> 20) & 0xFFF) as i16;
                imm = ((imm + (0b1000_0000_0000)) & (0xFFF)) - 0b1000_0000_0000;     // sign extention
                let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
                let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;

                self.register[rd] = ((self.pc as u64) + 4) & 0xFFFF_FFFF_FFFF_FFFE;
                self.pc = (self.register[rs1] as i64  + imm as i64) as usize;
                if self.pc == 0 {
                    std::process::exit(0);
                }
                self.fetch()?;
                self.execute()?;
            },
            // FENCE
            0b000_1111  => unimplemented!(),      // treat as nop
            // SYSTEM
            0b1110011   => self.decode_system()?,
            _           => unimplemented!(),
        }

        Ok(())
    }

    fn decode_rtype(&mut self) -> Result<(), Exception> {
        // Decode instruction
        let funct7: u8      = ((self.instruction >> 25) & 0x7F) as u8;
        let rs2:    usize   = ((self.instruction >> 20) & 0x1F) as usize;
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;
        let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;

        match funct7 {
            0b000_0000 => {    
                match funct3 {
                    // ADD
                    0b000   => self.register[rd] = self.register[rs1] + self.register[rs2],
                    // SLL
                    0b001   => self.register[rd] = self.register[rs1] << self.register[rs2],
                    // SLT
                    0b010   => {
                        if (self.register[rs1] as i64) < (self.register[rs2] as i64) {
                            self.register[rd] = 1;
                        }
                        else {
                            self.register[rd] = 0;
                        }
                    },
                    // SLTU
                    0b011   => {
                        if (self.register[rs1] as u64) < (self.register[rs2] as u64) {
                            self.register[rd] = 1;
                        }
                        else {
                            self.register[rd] = 0;
                        }
                    },
                    // XOR
                    0b100   => self.register[rd] = self.register[rs1] ^ self.register[rs2],
                    // SRL
                    0b101   => self.register[rd] = self.register[rs1] >> self.register[rs2],
                    // OR
                    0b110   => self.register[rd] = self.register[rs1] | self.register[rs2],
                    // AND
                    0b111   => self.register[rd] = self.register[rs1] & self.register[rs2],
                    _       => unimplemented!(),
                }
            },
            0b010_0000       => {
                match funct3 {
                    // SUB
                    0b000   => self.register[rd] = ((self.register[rs1] as i64) - (self.register[rs2] as i64)) as u64,
                    // SRA
                    0b101   => self.register[rd] = (self.register[rs1] as i64 >> self.register[rs2]) as u64,
                    _       => unimplemented!(),
                }
            },
            _               => unimplemented!(),
        }

        Ok(())
    }

    fn decode_itype(&mut self) -> Result<(), Exception> {
        // Decode instruction
        let mut imm:    i16 = ((self.instruction >> 20) & 0xFFF) as i16;
        imm = ((imm + (0b1000_0000_0000)) & (0xFFF)) - 0b1000_0000_0000;     // sign extention
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;
        let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;

        match funct3 {
            // ADDI
            0b000   => self.register[rd] = ((self.register[rs1] as i64) + (imm as i64)) as u64,
            // SLLI
            0b001   => self.register[rd] = ((self.register[rs1] as u64) << (imm as u64)) as u64,
            // SLTI
            0b010   => {
                if (self.register[rs1] as i64) < (imm as i64) {
                    self.register[rd] = 1;
                }
                else {
                    self.register[rd] = 0;
                }
            },
            // SLTIU
            0b011   => {
                if (self.register[rs1] as u64) < (imm as u64) {
                    self.register[rd] = 1;
                }
                else {
                    self.register[rd] = 0;
                }
            },
            // XORI
            0b100   => self.register[rd] = ((self.register[rs1] as i64) ^ (imm as i64)) as u64,
            0b101   => {
                match (imm >> 5) & 0x7F {
                    // SRLI
                    0b0000_0000 => self.register[rd] = ((self.register[rs1] as u64) >> (imm & 0x1F)) as u64,
                    // SRAI
                    0b0010_0000 => self.register[rd] = ((self.register[rs1] as i64) >> (imm & 0x1F)) as u64,
                    _           => unimplemented!(),
                }
            },
            // ORI
            0b110   => self.register[rd] = ((self.register[rs1] as i64) | (imm as i64)) as u64,
            // ANDI
            0b111   => self.register[rd] = ((self.register[rs1] as i64) & (imm as i64)) as u64,
            _       => unimplemented!(),
        }

        Ok(())
    }

    fn decode_btype(&mut self) -> Result<(), Exception> {
        // Decode instruction
        let mut imm: i16    = (((self.instruction & 0x8000_0000_0000_0000) >> 18) |
                               ((self.instruction & 0x80) << 4) |
                               ((self.instruction & 0x7E00_0000_0000_0000) >> 20) |
                               ((self.instruction & 0xF00) >> 7)) as i16;
        imm = ((imm + (0b1000_0000_0000)) & (0xFFF)) - 0b1000_0000_0000;     // sign extention
        let rs2:    usize   = ((self.instruction >> 20) & 0x1F) as usize;
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;

        match funct3 {
            // BEQ
            0b000   => {
                if self.register[rs1] == self.register[rs2] {
                    self.pc = (self.pc as i64 + imm as i64) as usize;
                    if self.pc == 0 {
                        std::process::exit(0);
                    }
                    self.fetch()?;
                    self.execute()?;
                }
            },
            // BNE
            0b001   => {
                if self.register[rs1] != self.register[rs2] {
                    self.pc = (self.pc as i64 + imm as i64) as usize;
                    if self.pc == 0 {
                        std::process::exit(0);
                    }
                    self.fetch()?;
                    self.execute()?;
                }
            },
            // BLT
            0b100   => {
                if (self.register[rs1] as i64) < (self.register[rs2] as i64){
                    self.pc = (self.pc as i64 + imm as i64) as usize;
                    if self.pc == 0 {
                        std::process::exit(0);
                    }
                    self.fetch()?;
                    self.execute()?;
                }
            },
            // BGE
            0b101   => {
                if (self.register[rs1] as i64) >= (self.register[rs2] as i64){
                    self.pc = (self.pc as i64 + imm as i64) as usize;
                    if self.pc == 0 {
                        std::process::exit(0);
                    }
                    self.fetch()?;
                    self.execute()?;
                }
            },
            // BLTU
            0b110   => {
                if self.register[rs1] < self.register[rs2] {
                    self.pc = (self.pc as i64 + imm as i64) as usize;
                    if self.pc == 0 {
                        std::process::exit(0);
                    }
                    self.fetch()?;
                    self.execute()?;
                }
            },
            // BGEU
            0b111   => {
                if (self.register[rs1]) >= (self.register[rs2]){
                    self.pc = (self.pc as i64 + imm as i64) as usize;
                    if self.pc == 0 {
                        std::process::exit(0);
                    }
                    self.fetch()?;
                    self.execute()?;
                }
            },
            _       => unimplemented!(),
        }

        Ok(())
    }

    fn decode_load(&mut  self) -> Result<(), Exception> {
        // Decode instruction
        let mut imm:    i16 = ((self.instruction >> 20) & 0xFFF) as i16;
        imm = ((imm + (0b1000_0000_0000)) & 0xFFF) - 0b1000_0000_0000;     // sign extention
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;
        let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;

        match funct3 {
            // LB
            0b000   => {
                let addr: usize     = (self.register[rs1] as i64 + imm as i64) as usize;
                let byte: u8        = self.mmu.read8(self.csr, addr)?;
                let data: i64       = ((byte as i64 + 0b1000_0000) & 0xFF) - 0b1000_0000;   // sign extention
                self.register[rd]   = data as u64;
            },
            // LH
            0b001   => {
                let addr: usize     = (self.register[rs1] as i64 + imm as i64) as usize;
                let hword: u16      = self.mmu.read16(self.csr, addr)?;
                let data: i64       = ((hword as i64 + 0b1000_0000_0000_0000) & 0xFFFF) - 0b1000_0000_0000_0000;   // sign extention
                self.register[rd]   = data as u64;
            },
            // LW
            0b010   => {
                let addr: usize     = (self.register[rs1] as i64 + imm as i64) as usize;
                let word: u32       = self.mmu.read32(self.csr, addr)?;
                let data: i64       = ((word as i64 + 0b1000_0000_0000_0000_0000_0000_0000_0000) & 0xFFFF_FFFF)
                                      - 0b1000_0000_0000_0000_0000_0000_0000_0000;   // sign extention
                self.register[rd]   = data as u64;
            },
            _       => unimplemented!(),
        }

        Ok(())
    }

    fn decode_store(&mut self) -> Result<(), Exception> {
        // Decode instruction
        let mut imm: i16    = (((self.instruction & 0xFE00_0000_0000_0000) >> 20) |
                               ((self.instruction & 0xF80) >> 7)) as i16;
        imm = ((imm + (0b1000_0000_0000)) & (0xFFF)) - 0b1000_0000_0000;     // sign extention
        let rs2:    usize   = ((self.instruction >> 20) & 0x1F) as usize;
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;

        match funct3{
            // SB
            0b000   => {
                let addr: usize = (self.register[rs1] as i64 + imm as i64) as usize;
                let byte: u8    = (self.register[rs2] & 0xFF) as u8;
                self.mmu.write8(addr, byte);
            },
            // SH
            0b001   => {
                let addr: usize = (self.register[rs1] as i64 + imm as i64) as usize;
                let hword: u16  = (self.register[rs2] & 0xFFFF) as u16;
                self.mmu.write16(addr, hword);
            },
            // SW
            0b010   => {
                let addr: usize = (self.register[rs1] as i64 + imm as i64) as usize;
                let word: u32   = (self.register[rs2] & 0xFFFF_FFFF) as u32;
                self.mmu.write32(addr, word);
            },
            _       => unimplemented!(),
        }

        Ok(())
    }

    fn decode_system(&mut self) -> Result<(), Exception> {
        // Decode instruction
        let funct12:    u16 = ((self.instruction >> 20) & 0xFFF) as u16;
        let funct3:     u8  = ((self.instruction >> 12) & 0x7) as u8;

        match funct3 {
            // Trap-Return Instruction
            0b000   => {
                match funct12 {
                    // ECALL
                    0b0000_0000_0000    => {
                        match self.csr.priv_level {
                            PrivLevel::USER         => return Err(Exception::EnvCallUmode),
                            PrivLevel::SUPERVISOR   => return Err(Exception::EnvCallSmode),
                            PrivLevel::MACHINE      => return Err(Exception::EnvCallMmode),
                            _                       => unimplemented!(),
                        }
                    },
                    // EBREAK
                    0b0000_0000_0001    => unimplemented!(),
                    // MRET
                    0b0000_0000_0010    => unimplemented!(),
                    // SRET
                    0b0001_0000_0010    => unimplemented!(),
                    // MRET
                    0b0011_0000_0010    => unimplemented!(),
                    _                   => unimplemented!(),
                }
            },
            // Zicsr
            _       => self.decode_zicsr()?,
        }

        Ok(())
    }

    fn decode_zicsr(&mut self) -> Result<(), Exception> {
        // Decode instruction
        let csr:    u16     = ((self.instruction >> 20) & 0xFFF) as u16;
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let uimm:   u8      = ((self.instruction >> 15) & 0x1F) as u8;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;
        let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;

        match funct3 {
            // CSRRW
            0b001   => {
                if rd != 0 {
                    let data: u64 = self.csr.read(csr) as u64;
                    self.register[rd] = data;
                }
                self.csr.write(csr, self.register[rs1]);
            },
            // CSRRS
            0b010   => {
                let data: u64 = self.csr.read(csr) as u64;
                self.register[rd] = data;
                self.csr.write(csr, data | self.register[rs1]);
            },
            // CSRRC
            0b011   => {
                let mut data: u64 = self.csr.read(csr);
                self.register[rd] = data as u64;
                data &= !(self.register[rs1]);
                self.csr.write(csr, data);
            },
            // CSRRWI
            0b101   => {
                if rd != 0 {
                    let data: u64 = self.csr.read(csr) as u64;
                    self.register[rd] = data;
                }
                self.csr.write(csr, uimm as u64);
            },
            // CSRRSI
            0b110   => {
                let data: u64 = self.csr.read(csr) as u64;
                self.register[rd] = data;
                self.csr.write(csr, data | (uimm as u64));
            },
            // CSRRCI
            0b111   => {
                let mut data: u64 = self.csr.read(csr);
                self.register[rd] = data as u64;
                data &= !(uimm) as u64;
                self.csr.write(csr, data);
            },
            _       => unimplemented!(),
        }

        Ok(())
    }
}