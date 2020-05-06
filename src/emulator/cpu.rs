use crate::emulator::mmu::Mmu;
use crate::emulator::memory::MEMORY_SIZE;
use crate::emulator::csr::*;
use crate::emulator::exception::Exception;
use std::fs::read;
use std::fmt;

type Instruction    = u32;

const NREGISTERS:   usize = 32;
const INIT_PC:      usize = 0;

// General Registers (standardized names as part of the RISC-V application binary interface (ABI))
#[derive(Copy, Clone)]
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

pub fn reg_name(index: u8) -> String {
    match index {
        0   => "ZERO".to_string(),
        1   => "RA".to_string(),
        2   => "SP".to_string(),
        3   => "GP".to_string(),
        4   => "TP".to_string(),
        5   => "T0".to_string(),
        6   => "T1".to_string(),
        7   => "T2".to_string(),
        8   => "FP".to_string(),
        9   => "S1".to_string(),
        10  => "A0".to_string(),
        11  => "A1".to_string(),
        12  => "A2".to_string(),
        13  => "A3".to_string(),
        14  => "A4".to_string(),
        15  => "A5".to_string(),
        16  => "A6".to_string(),
        17  => "A7".to_string(),
        18  => "S2".to_string(),
        19  => "S3".to_string(),
        20  => "S4".to_string(),
        21  => "S5".to_string(),
        22  => "S6".to_string(),
        23  => "S7".to_string(),
        24  => "S8".to_string(),
        25  => "S9".to_string(),
        26  => "S10".to_string(),
        27  => "S11".to_string(),
        28  => "T3".to_string(),
        29  => "T4".to_string(),
        30  => "T5".to_string(),
        31  => "T6".to_string(),
        _   => "unknown".to_string(),
    }
}

#[derive(Debug)]
pub struct XRegisters {
    register: [u64; NREGISTERS],        // General registers
}

impl fmt::Display for XRegisters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, reg) in self.register.into_iter().enumerate() {
            write!(f, "  x{:2}({})\t= 0x{:016x}\n", i, reg_name(i as u8), reg)?;
        }
        write!(f, "")
    }
}

impl XRegisters {
    fn new() -> Self {
        XRegisters {
            register: [0; NREGISTERS],
        }
    }

    pub fn write(&mut self, index: usize, data: u64) {
        if index == 0 {
            return;
        }
        self.register[index] = data;
    }

    pub fn read(&self, index: usize) -> u64 {
        self.register[index]
    }
}

pub struct Cpu {
    pub register: XRegisters,       // General registers
    pub instruction: Instruction,   // Current instruction
    pub pc: usize,                  // Program counter
    pub mmu: Mmu,                   // MMU (Memory Management Unit)
    pub csr: Csr,                   // CSRs (Control/Status Registers)
    pub debug: bool,                // Debug flag
    pub step: bool,                 // Step execution mode flag
    watchpoint: (Registers, u64),
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register: XRegisters::new(),
            instruction: 0,
            pc: INIT_PC,
            mmu: Mmu::new(),
            csr: Csr::new(),
            debug: false,
            step: false,
            watchpoint: (Registers::ZERO, 1),
        }
    }

    pub fn load(&mut self, filename: &String) -> usize {
        let binary = read(filename).unwrap();
        let len = binary.len();

        if len > MEMORY_SIZE {
            panic!("[ERROR] too large binary({}): {} Byte", filename, len);
        }
        
        for (i, byte) in binary.iter().enumerate() {
            self.mmu.write8(self.csr, i, *byte).unwrap();
        }

        len
    }

    pub fn run(&mut self) {
        use std::io::stdin;

        let mut input = String::new();

        loop {
            match self.fetch() {
                Ok(_)           => {},
                Err(exception)  => exception.take_trap(self),
            }
            
            if self.debug { println!("[INFO] pc: 0x{:08x}", self.pc); }
            if self.debug { println!("{}", inspect_instruciton(self.instruction)); }
            if self.debug { println!("[INFO] ==Register==\n{}", self.register); }
            if self.step { stdin().read_line(&mut input); }
            if self.register.read(self.watchpoint.0 as usize) == self.watchpoint.1 { return; }

            match self.execute() {
                Ok(_)           => {},
                Err(exception)  => exception.take_trap(self),
            }

            self.pc += 4;

        }
    }

    pub fn fetch(&mut self) -> Result<(), Exception> {
        self.instruction = self.mmu.read32(self.csr, self.pc)?;
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
            0b011_0111  => self.register.write(rd as usize, ((imm & 0xF_FFFF) << 12) as i32 as i64 as u64),
            // AUIPC
            0b001_0111  => self.register.write(rd as usize, (((imm as i32) << 12) as i64 + self.pc as i64) as u64),
            // JAL
            0b110_1111  => {
                // signed offset in multiples of 2 bytes
                let mut offset: i32   = (((self.instruction & 0x8000_0000) >> 11) |     // imm[20]
                                         ((self.instruction & 0x7FE0_0000) >> 20) |     // imm[10:1]
                                         ((self.instruction & 0x100000)    >>  9) |     // imm[11]
                                          (self.instruction  & 0xFF000)) as i32;        // imm[19:12]
                offset = ((offset + (0b1000_0000_0000_0000)) & (0xFFFFF)) - 0b1000_0000_0000_0000;        // sign extension
                if rd != 0 {
                    self.register.write(rd as usize, self.pc as u64 + 4);
                };
                self.pc = (self.pc as i64 + offset as i64) as usize;
                if self.pc == 0 {
                    std::process::exit(0);
                }
                self.pc -= 4;
            },
            // B-type
            0b110_0011  => self.decode_btype()?,
            // JALR
            0b110_0111  => {
                // Decode instruction
                let mut imm:    i16 = ((self.instruction >> 20) & 0xFFF) as i16;
                imm = ((imm + (0b1000_0000_0000)) & (0xFFF)) - 0b1000_0000_0000;     // sign extension
                let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
                let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;

                let addr = self.register.read(rs1);
                if rd != 0 {
                    self.register.write(rd as usize, ((self.pc as u64) + 4) & 0xFFFF_FFFF_FFFF_FFFE);
                }
                self.pc = (addr as i32  + imm as i32) as i64 as usize;
                if self.pc == 0 {
                    std::process::exit(0);
                }
                self.pc -= 4;
            },
            // FENCE
            0b000_1111  => return Ok(()),      // treat as nop
            // SYSTEM
            0b111_0011  => self.decode_system()?,
            // RV64I Integer Register-Immediate Instructions
            0b001_1011  => self.decode_rv64i_itype()?,
            // RV64I Integer Register-Register Operations
            0b011_1011  => self.decode_rv64i_rtype()?,
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
                    0b000   => self.register.write(rd as usize, self.register.read(rs1 as usize).wrapping_add(self.register.read(rs2 as usize))),
                    // SLL
                    0b001   => self.register.write(rd as usize, self.register.read(rs1 as usize).wrapping_shl(self.register.read(rs2 as usize) as u32)),
                    // SLT
                    0b010   => {
                        if (self.register.read(rs1 as usize)  as i64) < self.register.read(rs2 as usize) as i64 {
                            self.register.write(rd as usize, 1);
                        }
                        else {
                            self.register.write(rd as usize, 0);
                        }
                    },
                    // SLTU
                    0b011   => {
                        if (self.register.read(rs1 as usize)) < self.register.read(rs2 as usize) {
                            self.register.write(rd as usize, 1);
                        }
                        else {
                            self.register.write(rd as usize, 0);
                        }
                    },
                    // XOR
                    0b100   => self.register.write(rd as usize, self.register.read(rs1 as usize) ^ self.register.read(rs2 as usize)),
                    // SRL
                    0b101   => self.register.write(rd as usize, self.register.read(rs1 as usize).wrapping_shr(self.register.read(rs2 as usize) as u32)),
                    // OR
                    0b110   => self.register.write(rd as usize, self.register.read(rs1 as usize) | self.register.read(rs2 as usize)),
                    // AND
                    0b111   => self.register.write(rd as usize, self.register.read(rs1 as usize) & self.register.read(rs2 as usize)),
                    _       => unimplemented!(),
                }
            },
            0b010_0000       => {
                match funct3 {
                    // SUB
                    0b000   => self.register.write(rd as usize, (self.register.read(rs1 as usize) as i64 - self.register.read(rs2 as usize) as i64) as u64),
                    // SRA
                    0b101   => self.register.write(rd as usize, ((self.register.read(rs1 as usize) as i64).wrapping_shr(self.register.read(rs2 as usize) as u32)) as u64),
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
        imm = ((imm + (0b1000_0000_0000)) & (0xFFF)) - 0b1000_0000_0000;     // sign extension
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;
        let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;

        match funct3 {
            // ADDI
            0b000   => self.register.write(rd as usize, ((self.register.read(rs1 as usize) as i64).wrapping_add(imm as i64)) as u64),
            // SLLI
            0b001   => self.register.write(rd as usize, ((self.register.read(rs1 as usize) as u64).wrapping_shl(imm as u32)) as u64),
            // SLTI
            0b010   => {
                if (self.register.read(rs1 as usize)  as i64 ) < imm as i64 {
                    self.register.write(rd as usize, 1);
                }
                else {
                    self.register.write(rd as usize, 0);
                }
            },
            // SLTIU
            0b011   => {
                if (self.register.read(rs1 as usize)  as u64 ) < imm as u64 {
                    self.register.write(rd as usize, 1);
                }
                else {
                    self.register.write(rd as usize, 0);
                }
            },
            // XORI
            0b100   => self.register.write(rd as usize, (self.register.read(rs1 as usize) as i64 ^ (imm as i64)) as u64),
            0b101   => {
                match (imm >> 5) & 0x7F {
                    // SRLI
                    0b0000_0000 => self.register.write(rd as usize, ((self.register.read(rs1 as usize) as u64).wrapping_shr((imm & 0x1F) as u32)) as u64),
                    // SRAI
                    0b0010_0000 => self.register.write(rd as usize, ((self.register.read(rs1 as usize) as i64) >> (imm & 0x1F)) as u64),
                    _           => unimplemented!(),
                }
            },
            // ORI
            0b110   => self.register.write(rd as usize, (self.register.read(rs1 as usize) as i64 | (imm as i64)) as u64),
            // ANDI
            0b111   => self.register.write(rd as usize, (self.register.read(rs1 as usize) as i64 & (imm as i64)) as u64),
            _       => unimplemented!(),
        }

        Ok(())
    }

    fn decode_btype(&mut self) -> Result<(), Exception> {
        // Decode instruction
        let mut imm: i16    = (((self.instruction & 0x8000_0000) >> 18) |
                               ((self.instruction & 0x80) << 4) |
                               ((self.instruction & 0x7E00_0000) >> 20) |
                               ((self.instruction & 0xF00) >> 7)) as i16;
        imm = ((imm + (0b1000_0000_0000)) & (0xFFF)) - 0b1000_0000_0000;     // sign extension
        let rs2:    usize   = ((self.instruction >> 20) & 0x1F) as usize;
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;

        match funct3 {
            // BEQ
            0b000   => {
                if self.register.read(rs1 as usize) == self.register.read(rs2 as usize) {
                    self.pc = (self.pc as i64 + imm as i64) as usize;
                    if self.pc == 0 {
                        std::process::exit(0);
                    }
                    self.pc -= 4;
                }
            },
            // BNE
            0b001   => {
                if self.register.read(rs1 as usize) != self.register.read(rs2 as usize) {
                    self.pc = (self.pc as i64 + imm as i64) as usize;
                    if self.pc == 0 {
                        std::process::exit(0);
                    }
                    self.pc -= 4;
                }
            },
            // BLT
            0b100   => {
                if (self.register.read(rs1 as usize) as i64) < (self.register.read(rs2 as usize) as i64) {
                    self.pc = (self.pc as i64 + imm as i64) as usize;
                    if self.pc == 0 {
                        std::process::exit(0);
                    }
                    self.pc -= 4;
                }
            },
            // BGE
            0b101   => {
                if (self.register.read(rs1 as usize) as i64) >= (self.register.read(rs2 as usize) as i64) {
                    self.pc = (self.pc as i64 + imm as i64) as usize;
                    if self.pc == 0 {
                        std::process::exit(0);
                    }
                    self.pc -= 4;
                }
            },
            // BLTU
            0b110   => {
                if self.register.read(rs1 as usize) < self.register.read(rs2 as usize) {
                    self.pc = (self.pc as i64 + imm as i64) as usize;
                    if self.pc == 0 {
                        std::process::exit(0);
                    }
                    self.pc -= 4;
                }
            },
            // BGEU
            0b111   => {
                if self.register.read(rs1 as usize) >= self.register.read(rs2 as usize) {
                    self.pc = (self.pc as i64 + imm as i64) as usize;
                    if self.pc == 0 {
                        std::process::exit(0);
                    }
                    self.pc -= 4;
                }
            },
            _       => unimplemented!(),
        }

        Ok(())
    }

    fn decode_load(&mut  self) -> Result<(), Exception> {
        // Decode instruction
        let mut imm:    i16 = ((self.instruction >> 20) & 0xFFF) as i16;
        imm = ((imm + (0b1000_0000_0000)) & 0xFFF) - 0b1000_0000_0000;     // sign extension
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;
        let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;

        match funct3 {
            // LB
            0b000   => {
                let addr: usize     = (self.register.read(rs1 as usize) as i64 + imm as i64) as usize;
                let byte: u8        = self.mmu.read8(self.csr, addr)?;
                let data: i64       = ((byte as i64 + 0b1000_0000) & 0xFF) - 0b1000_0000;   // sign extension
                self.register.write(rd as usize, data as u64);
            },
            // LH
            0b001   => {
                let addr: usize     = (self.register.read(rs1 as usize) as i64 + imm as i64) as usize;
                let hword: u16      = self.mmu.read16(self.csr, addr)?;
                let data: i64       = ((hword as i64 + 0b1000_0000_0000_0000) & 0xFFFF) - 0b1000_0000_0000_0000;   // sign extension
                self.register.write(rd as usize, data as u64);
            },
            // LW
            0b010   => {
                let addr: usize     = (self.register.read(rs1 as usize) as i64 + imm as i64) as usize;
                let word: u32       = self.mmu.read32(self.csr, addr)?;
                let data: i64       = word as i32 as i64;   // sign extension
                self.register.write(rd as usize, data as u64);
            },
            _       => unimplemented!(),
        }

        Ok(())
    }

    fn decode_store(&mut self) -> Result<(), Exception> {
        // Decode instruction
        let mut imm: i16    = (((self.instruction & 0xFE00_0000) >> 20) |
                               ((self.instruction & 0xF80) >> 7)) as i16;
        imm = ((imm + (0b1000_0000_0000)) & (0xFFF)) - 0b1000_0000_0000;     // sign extension
        let rs2:    usize   = ((self.instruction >> 20) & 0x1F) as usize;
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;

        match funct3{
            // SB
            0b000   => {
                let addr: usize = (self.register.read(rs1 as usize) as i64 + imm as i64) as usize;
                let byte: u8    = (self.register.read(rs2 as usize) & 0xFF) as u8;
                self.mmu.write8(self.csr, addr, byte)?;
            },
            // SH
            0b001   => {
                let addr: usize = (self.register.read(rs1 as usize) as i64 + imm as i64) as usize;
                let hword: u16  = (self.register.read(rs2 as usize) & 0xFFFF) as u16;
                self.mmu.write16(self.csr, addr, hword)?;
            },
            // SW
            0b010   => {
                let addr: usize = (self.register.read(rs1 as usize) as i64 + imm as i64) as usize;
                let word: u32   = (self.register.read(rs2 as usize) & 0xFFFF_FFFF) as u32;
                self.mmu.write32(self.csr, addr, word)?;
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
                    // URET
                    0b0000_0000_0010    => unimplemented!(),
                    // SRET
                    0b0001_0000_0010    => unimplemented!(),
                    // MRET
                    0b0011_0000_0010    => {
                        let mpp = self.csr.read_bits(MSTATUS, 11..12+1);    // Get MPP bits
                        let mpie = self.csr.read_bit(MSTATUS, 7);           // Get MPIE bit
                        self.csr.write_bit(MSTATUS, 3, mpie);          // Set MIE bit
                        self.csr.set_priv_level(mpp as u8);
                        self.csr.write_bit(MSTATUS, 7, true);
                        self.csr.write_bits(MSTATUS, 11..12+1, PrivLevel::USER as u64);

                        self.pc = self.csr.read(MEPC) as usize;
                        if self.pc == 0 {
                            std::process::exit(0);
                        }

                        self.pc -= 4;
                    },
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
                    self.register.write(rd as usize, data);
                }
                self.csr.write(csr, self.register.read(rs1 as usize));
            },
            // CSRRS
            0b010   => {
                let data: u64 = self.csr.read(csr) as u64;
                self.register.write(rd as usize, data);
                self.csr.write(csr, data | self.register.read(rs1 as usize));
            },
            // CSRRC
            0b011   => {
                let mut data: u64 = self.csr.read(csr);
                self.register.write(rd as usize, data);
                data &= !(self.register.read(rs1 as usize));
                self.csr.write(csr, data);
            },
            // CSRRWI
            0b101   => {
                if rd != 0 {
                    let data: u64 = self.csr.read(csr) as u64;
                    self.register.write(rd as usize, data);
                }
                self.csr.write(csr, uimm as u64);
            },
            // CSRRSI
            0b110   => {
                let data: u64 = self.csr.read(csr) as u64;
                self.register.write(rd as usize, data);
                self.csr.write(csr, data | (uimm as u64));
            },
            // CSRRCI
            0b111   => {
                let mut data: u64 = self.csr.read(csr);
                self.register.write(rd as usize, data);
                data &= !(uimm) as u64;
                self.csr.write(csr, data);
            },
            _       => unimplemented!(),
        }

        Ok(())
    }

    fn decode_rv64i_itype(&mut self) -> Result<(), Exception> {
        // Decode instruction
        let mut imm:    i16 = ((self.instruction >> 20) & 0xFFF) as i16;
        imm = ((imm + (0b1000_0000_0000)) & (0xFFF)) - 0b1000_0000_0000;     // sign extension
        let funct7: u8      = ((self.instruction >> 25) & 0x7F) as u8;
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;
        let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;

        match funct3 {
            // ADDIW
            0b000   => self.register.write(rd as usize, ((self.register.read(rs1 as usize) as i32).wrapping_add(imm as i32)) as u64),
            // SLLIW
            0b001   => self.register.write(rd as usize, ((self.register.read(rs1 as usize) as i32).wrapping_shl(imm as u32)) as u64),
            0b101   => {
                match funct7 {
                    // SRLIW
                    0b000_0000  => self.register.write(rd as usize, ((self.register.read(rs1 as usize) as u32).wrapping_shr(imm as u32))  as i32 as u64),
                    // SRAIW
                    0b010_0000  => self.register.write(rd as usize, ((self.register.read(rs1 as usize) as i32).wrapping_shr(imm as u32)) as u64),
                    _           => unimplemented!(),
                }
            },
            _       => unimplemented!(),
        }

        Ok(())
    }

    fn decode_rv64i_rtype(&mut self) -> Result<(), Exception> {
        let funct7: u8      = ((self.instruction >> 25) & 0x7F) as u8;
        let rs2:    usize   = ((self.instruction >> 20) & 0x1F) as usize;
        let rs1:    usize   = ((self.instruction >> 15) & 0x1F) as usize;
        let funct3: u8      = ((self.instruction >> 12) & 0x7) as u8;
        let rd:     usize   = ((self.instruction >> 7) & 0xF) as usize;

        match funct7 {
            0b000_0000  => match funct3 {
                // ADDW
                0b000       => self.register.write(rd as usize, (self.register.read(rs1 as usize) as i32).wrapping_add(self.register.read(rs2 as usize) as i32) as u64),
                // SLLW
                0b001       => self.register.write(rd as usize, (self.register.read(rs1 as usize) as i32).wrapping_shl(self.register.read(rs2 as usize) as u32) as u64),
                // SRLW
                0b101       => self.register.write(rd as usize, (self.register.read(rs1 as usize) as u32).wrapping_shr(self.register.read(rs2 as usize) as u32) as i32 as u64),
                _           => unimplemented!(),
            },
            0b010_0000  => match funct3 {
                // SUBW
                0b000       => self.register.write(rd as usize, ((self.register.read(rs1 as usize) as i32).wrapping_sub(self.register.read(rs2 as usize) as i32)) as i64 as u64),
                // SRAW
                0b101       => self.register.write(rd as usize, ((self.register.read(rs1 as usize) as i32).wrapping_shr((self.register.read(rs2 as usize) & 0x1F) as u32)) as u64),
                _           => unimplemented!(),
            },
            _           => unimplemented!(),
        }

        Ok(())
    }

    // Setting Watchpoints
    pub fn watch(&mut self, register: Registers, val: u64) {
        self.watchpoint.0 = register;
        self.watchpoint.1 = val;
    }

}

fn inspect_instruciton(instruction: Instruction) -> String {
    let funct12:    u16 = ((instruction >> 20) & 0xFFF) as u16;
    let funct7: u8      = ((instruction >> 25) & 0x7F) as u8;
    let _rs2:    usize   = ((instruction >> 20) & 0x1F) as usize;
    let _rs1:    usize   = ((instruction >> 15) & 0x1F) as usize;
    let funct3: u8      = ((instruction >> 12) & 0x7) as u8;
    let _rd:     usize   = ((instruction >> 7) & 0xF) as usize;
    let opcode: u8      = (instruction & 0x7F) as u8;

    let mut output = format!("[INFO] instruction(0x{:08x})", instruction);

    match opcode {
        0b011_0111  => output = format!("{}: LUI", output),
        0b001_0111  => output = format!("{}: AUIPC", output),
        0b110_1111  => output = format!("{}: JAL", output),
        0b110_0111  => output = format!("{}: JALR", output),
        0b110_0011  => match funct3 {
            0b000       => output = format!("{}: BEQ", output),
            0b001       => output = format!("{}: BNE", output),
            0b100       => output = format!("{}: BLT", output),
            0b101       => output = format!("{}: BGE", output),
            0b110       => output = format!("{}: BLTU", output),
            0b111       => output = format!("{}: BGEU", output),
            _           => return format!("{}: unknown", output),
        },
        0b000_0011  => match funct3 {
            0b000       => output = format!("{}: LB", output),
            0b001       => output = format!("{}: LH", output),
            0b010       => output = format!("{}: LW", output),
            0b100       => output = format!("{}: LBU", output),
            0b101       => output = format!("{}: LHU", output),
            _           => return format!("{}: unknown", output),
        },
        0b010_0011  => match funct3 {
            0b000       => output = format!("{}: SB", output),
            0b001       => output = format!("{}: SH", output),
            0b010       => output = format!("{}: SW", output),
            _           => return format!("{}: unknown", output),
        },
        0b001_0011  => match funct3 {
            0b000       => output = format!("{}: ADDI", output),
            0b010       => output = format!("{}: SLTI", output),
            0b011       => output = format!("{}: SLTIU", output),
            0b100       => output = format!("{}: XORI", output),
            0b110       => output = format!("{}: ORI", output),
            0b111       => output = format!("{}: ANDI", output),
            0b001       => output = format!("{}: SLLI", output),
            0b101       =>  match funct7 {
                0b000_0000  => output = format!("{}: SRLI", output),
                0b010_0000  => output = format!("{}: SRAI", output),
                _           => return format!("{}: unknown", output),
            },
            _           => return format!("{}: unknown", output),
        },
        0b011_0011  => match funct3 {
            0b000       =>  match funct7 {
                0b000_0000  => output = format!("{}: ADD", output),
                0b010_0000  => output = format!("{}: SUB", output),
                _           => return format!("{}: unknown", output),
            },
            0b001       => output = format!("{}: SLL", output),
            0b010       => output = format!("{}: SLT", output),
            0b011       => output = format!("{}: SLTU", output),
            0b100       => output = format!("{}: XOR", output),
            0b101       =>  match funct7 {
                0b000_0000  => output = format!("{}: SRL", output),
                0b010_0000  => output = format!("{}: SRA", output),
                _           => return format!("{}: unknown", output),
            },
            0b110       => output = format!("{}: OR", output),
            0b111       => output = format!("{}: AND", output),
            _           => return format!("{}: unknown", output),
        },
        0b000_1111  => match funct3 {
            0b000       => output = format!("{}: FENCE", output),
            0b001       => output = format!("{}: FENCE.I", output),
            _           => return format!("{}: unknown", output),
        },
        0b111_0011  => match funct3 {
            0b000       =>  match funct12 {
                0b0000_0000_0000    => output = format!("{}: ECALL", output),
                0b0000_0000_0001    => output = format!("{}: EBREAK", output),
                _                   => match funct7 {
                    0b000_0000          => output = format!("{}: URET", output),
                    0b000_1000          => output = format!("{}: SRET", output),
                    0b001_1000          => output = format!("{}: MRET", output),
                    _                   => return format!("{}: unknown", output),
                },
            },
            0b001       => output = format!("{}: CSRRW", output),
            0b010       => output = format!("{}: CSRRS", output),
            0b011       => output = format!("{}: CSRRC", output),
            0b101       => output = format!("{}: CSRRWI", output),
            0b110       => output = format!("{}: CSRRSI", output),
            0b111       => output = format!("{}: CSRRCI", output),
            _           => return format!("{}: unknown", output),
        },        
        0b001_1011  => match funct3 {
            0b000       => output = format!("{}: ADDIW", output),
            0b001       => output = format!("{}: SLLIW", output),
            0b101       => {
                match funct7 {
                    0b000_0000  => output = format!("{}: SRLIW", output),
                    0b010_0000  => output = format!("{}: SRAIW", output),
                    _           => return format!("{}: unknown", output),
                }
            },
            _           => return format!("{}: unknown", output),
        },
        0b011_1011  => match funct7 {
            0b000_0000  => match funct3 {
                0b000       => output = format!("{}: ADDW", output),
                0b001       => output = format!("{}: SLLW", output),
                0b101       => output = format!("{}: SRLW", output),
                _           => return format!("{}: unknown", output),
            }
            0b010_0000  => match funct3 {
                0b000       => output = format!("{}: SUBW", output),
                0b101       => output = format!("{}: SRAW", output),
                _           => return format!("{}: unknown", output),
            }
            _           => return format!("{}: unknown", output),
        }
        _           => return format!("{}: unknown", output),
    }

    output
}