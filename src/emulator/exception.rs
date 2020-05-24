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
            Exception::InstAddrMisalign     =>  0,
            Exception::InstAccessFault      =>  1,
            Exception::IllegalInst          =>  2,
            Exception::Breakpoint	        =>  3,
            Exception::LoadAddrMislign      =>  4,
            Exception::LoadAccessFault	    =>  5,
            Exception::StoreAddrMisalign    =>  6,
            Exception::StoreAccessFault     =>  7,
            Exception::EnvCallUmode         =>  8,
            Exception::EnvCallSmode         =>  9,
            Exception::EnvCallMmode         =>  11,
            Exception::InstPageFault	    =>  12,
            Exception::LoadPageFault	    =>  13,
            Exception::StorePageFault	    =>  15,
        }
    }

    pub fn take_trap(&self, cpu: &mut Cpu) {
        if cpu.debug { eprintln!("[DEBUG] {:?}-{:?} take trap : {:?} {:?}", file!(), line!(), self, cpu.csr.priv_level);}
        let cur_priv_level = cpu.csr.priv_level;
        let cur_pc = cpu.pc; 
        let cause = self.exc_code()  as u64;

        let mdeleg = cpu.csr.read(MEDELEG);
        let sdeleg = cpu.csr.read(SEDELEG);

        let pos = cause & 0xFFFF;

        let new_priv_level = match ((mdeleg >> pos) & 1) == 0 {
            true    => PrivLevel::MACHINE,
            false   => match ((sdeleg >> pos) & 1) == 0 {
                true    => PrivLevel::SUPERVISOR,
                false   => PrivLevel::USER,
            }
        };

        cpu.csr.priv_level = new_priv_level;

         match cpu.csr.priv_level {
            PrivLevel::MACHINE      => {
                cpu.csr.write(MEPC, cur_pc as u64);
                cpu.csr.write(MCAUSE, cause as u64);
                cpu.csr.write(MTVAL, cur_pc as u64);    // ToDo: branch based on exception code
                cpu.pc = cpu.csr.read(MTVEC) as usize;

                let status = cpu.csr.read(MSTATUS);
				let mie = (status >> 3) & 1;
                let new_status = (status & !0x1888) | (mie << 7) | ((cur_priv_level as u64) << 11) as u64;
                cpu.csr.write(MSTATUS, new_status);
                
            },
            PrivLevel::SUPERVISOR   => {
                cpu.csr.write(SEPC, cur_pc as u64);
                cpu.csr.write(SCAUSE, cause as u64);
                cpu.csr.write(STVAL, cur_pc as u64);    // ToDo: branch based on exception code
                cpu.pc = cpu.csr.read(STVEC) as usize;
                
                let status = cpu.csr.read(SSTATUS);
				let sie = (status >> 1) & 1;
                let new_status = (status & !0x122) | (sie << 5) | (((cur_priv_level as u64) & 1) << 8) as u64;
                cpu.csr.write(SSTATUS, new_status);
            },
            PrivLevel::USER         => {
                cpu.csr.write(UEPC, cur_pc as u64);
                cpu.csr.write(UCAUSE, cause as u64);
                cpu.csr.write(UTVAL, cur_pc as u64);    // ToDo: branch based on exception code
                cpu.pc = cpu.csr.read(UTVEC) as usize;

                unimplemented!();
            },
            PrivLevel::RESERVED     => panic!(),
        }
    }
}