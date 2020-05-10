use crate::emulator::cpu::Cpu;
use crate::emulator::csr::*;

#[derive(Debug)]
pub enum Exception {
    InstAddrMisalign,
    InstAccessFault,
    IllegalInst,
    Breakpoint,
    LoadAddrMislign,
    LoadAccessFault,
    StoreAddrMisalign,
    StoreAccessFault,
    EnvCallUmode,
    EnvCallSmode,
    // 10: Reserved
    EnvCallMmode,
    InstPageFault,
    LoadPageFault,
    // 14: Reserved for future standard use
    StorePageFault,
    // 16~: Reserved
}

impl Exception {
    fn exc_code(&self) -> u8 {
        match self {
            Exception::InstAddrMisalign     =>  1,
            Exception::InstAccessFault      =>  2,
            Exception::IllegalInst          =>  3,
            Exception::Breakpoint	        =>  4,
            Exception::LoadAddrMislign      =>  5,
            Exception::LoadAccessFault	    =>  6,
            Exception::StoreAddrMisalign    =>  7,
            Exception::StoreAccessFault     =>  8,
            Exception::EnvCallUmode         =>  9,
            Exception::EnvCallSmode         =>  11,
            Exception::EnvCallMmode         =>  12,
            Exception::InstPageFault	    =>  13,
            Exception::LoadPageFault	    =>  14,
            Exception::StorePageFault	    =>  15,
        }
    }

    pub fn take_trap(&self, cpu: &mut Cpu) {
        let cur_pc = cpu.pc as u64;
        let prev_level  = cpu.csr.priv_level;

        let medeleg = cpu.csr.read(MEDELEG);
        let sedeleg = cpu.csr.read(SEDELEG);

        let pos = self.exc_code() & 0xFF;

        if ((medeleg >> pos) & 1) == 0 {
            cpu.csr.priv_level = PrivLevel::MACHINE;
        }
        else {
            if (sedeleg >> pos & 0b1) == 0 {
                cpu.csr.priv_level = PrivLevel::SUPERVISOR;
            }
            else {
                cpu.csr.priv_level = PrivLevel::USER;
            }
        }

        match cpu.csr.priv_level {
            PrivLevel::MACHINE      => {
                cpu.csr.write(MEPC, cur_pc);
                cpu.pc = cpu.csr.read(MTVEC) as usize;
                cpu.pc -= 4;
                cpu.csr.write(MCAUSE, self.exc_code() as u64);
                match self {
                    Exception::InstAddrMisalign     |
                    Exception::LoadAddrMislign      |
                    Exception::StoreAddrMisalign    |
                    Exception::InstPageFault        |
                    Exception::LoadPageFault        |
                    Exception::StorePageFault       => cpu.csr.write(MTVAL, cur_pc),
                    _                               => cpu.csr.write(MTVAL, 0),
                }
                let mstatus = cpu.csr.read_bit(MSTATUS, 3);
                cpu.csr.write_bit(MSTATUS, 7, mstatus);
                cpu.csr.write_bit(MSTATUS, 3, false);
                cpu.csr.write_bits(MSTATUS, 11..13, 0b00);
            },
            PrivLevel::SUPERVISOR   => {
                cpu.csr.write(SEPC, cur_pc);
                cpu.pc = cpu.csr.read(STVEC) as usize;
                cpu.pc -= 4;
                cpu.csr.write(SCAUSE, self.exc_code() as u64);
                match self {
                    Exception::InstAddrMisalign     |
                    Exception::LoadAddrMislign      |
                    Exception::StoreAddrMisalign    |
                    Exception::InstPageFault        |
                    Exception::LoadPageFault        |
                    Exception::StorePageFault       => cpu.csr.write(STVAL, cur_pc),
                    _                               => cpu.csr.write(STVAL, 0),
                }
                let sstatus = cpu.csr.read_bit(SSTATUS, 1);
                cpu.csr.write_bit(SSTATUS, 5, sstatus);
                cpu.csr.write_bit(SSTATUS, 1, false);
                match prev_level {
                    PrivLevel::USER => cpu.csr.write_bit(SSTATUS, 8, false),
                    _               => cpu.csr.write_bit(SSTATUS, 8, true),
                }
            },
            PrivLevel::USER         => {
                cpu.csr.write(UEPC, cur_pc);
                cpu.pc = cpu.csr.read(UTVEC) as usize;
                cpu.pc -= 4;
                cpu.csr.write(UCAUSE, self.exc_code() as u64);
                match self {
                    Exception::InstAddrMisalign     |
                    Exception::LoadAddrMislign      |
                    Exception::StoreAddrMisalign    |
                    Exception::InstPageFault        |
                    Exception::LoadPageFault        |
                    Exception::StorePageFault       => cpu.csr.write(UTVAL, cur_pc),
                    _                               => cpu.csr.write(UTVAL, 0),
                }
            },
            _                       => unimplemented!(),
        }
    }
}