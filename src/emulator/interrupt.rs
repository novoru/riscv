use crate::emulator::cpu::Cpu;
use crate::emulator::csr::*;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum IrqNumber {
    NONE    = 0,
    VIRTIO  = 1,
    UART    = 10,
}

#[derive(Debug)]
pub enum Interrupt {
    UserSoftwareIrq,
    SupervisorSoftwareIrq,
    MachineSoftwareIrq,
    UserTimerIrq,
    SupervisorTimerIrq,
    MachineTimerIrq,
    UserExtIrq(u64),
    SupervisorExtIrq(u64),
    MachineExtIrq(u64),
}

impl Interrupt {
    fn exc_code(&self) -> u64 {
        let code: u64 = 1 << 63;
        match self {
            Interrupt::UserSoftwareIrq          => code + 0,
            Interrupt::SupervisorSoftwareIrq    => code + 1,
            Interrupt::MachineSoftwareIrq       => code + 3,
            Interrupt::UserTimerIrq             => code + 4,
            Interrupt::SupervisorTimerIrq       => code + 5,
            Interrupt::MachineTimerIrq          => code + 7,
            Interrupt::UserExtIrq(_)            => code + 8,
            Interrupt::SupervisorExtIrq(_)      => code + 9,
            Interrupt::MachineExtIrq(_)         => code + 11,
        }
    }

    fn irq(&self) -> u64 {
        match self {
            Interrupt::UserExtIrq(irq)          |
            Interrupt::SupervisorExtIrq(irq)    |
            Interrupt::MachineExtIrq(irq)       => *irq,
            _                                   => 0,
        }
    }

    pub fn take_trap(&mut self, cpu: &mut Cpu) {
        println!("[DEBUG] {}-{}: pc= 0x{:x}, take trap ({:?}=0x{:x})", file!(), line!(), cpu.pc, self, self.exc_code());
        let cur_pc = cpu.pc;
        let cur_priv_level = cpu.csr.priv_level;

        let cause = self.exc_code();
        let pos = cause & 0xFF;

        let medeleg = cpu.csr.read(MEDELEG);
        let sedeleg = cpu.csr.read(SEDELEG);

        let new_priv_level = match ((medeleg >> pos) & 1) == 0 {
            true    => PrivLevel::MACHINE,
            false   => match ((sedeleg >> pos) & 1) == 0 {
                true    => PrivLevel::SUPERVISOR,
                false   => PrivLevel::USER,
            },
        };

        let cur_status = match cpu.csr.priv_level {
            PrivLevel::MACHINE      => cpu.csr.read(MSTATUS),
            PrivLevel::SUPERVISOR   => cpu.csr.read(SSTATUS),
            PrivLevel::USER         => cpu.csr.read(USTATUS),
            PrivLevel::RESERVED     => panic!(),
        };

        let ie = match new_priv_level {
            PrivLevel::MACHINE      => cpu.csr.read(MIE),
            PrivLevel::SUPERVISOR   => cpu.csr.read(SIE),
            PrivLevel::USER         => cpu.csr.read(UIE),
            PrivLevel::RESERVED     => panic!(),
        };

        let cur_mie = (cur_status >> 3) & 1;
        let cur_sie = (cur_status >> 1) & 1;
        let _cur_uie =  cur_status & 1;

        // Software interrupt enable
        let msie = (ie >> 3) & 1;
        let ssie = (ie >> 1) & 1;
        let usie =  ie & 1;

        // Timer interrupt enable
        let mtie = (ie >> 7) & 1;
        let stie = (ie >> 5) & 1;
        let utie = (ie >> 4) & 1;

        // External interrupt enable
        let meie = (ie >> 11) & 1;
        let seie = (ie >> 9) & 1;
        let ueie = (ie >> 8) & 1;

        if new_priv_level < cur_priv_level {
            return;
        }
        else if new_priv_level == cur_priv_level {
            match cpu.csr.priv_level {
                PrivLevel::MACHINE  => {
                    if cur_mie == 0 {
                        return;
                    }
                },
                PrivLevel::SUPERVISOR   => {
                    if cur_sie == 0 {
                        return;
                    }
                },
                PrivLevel::USER => {
                    if cur_sie == 0 {
                        return;
                    }
                },
                PrivLevel::RESERVED => unimplemented!(),            
            }
        }

        match self {
            Interrupt::UserSoftwareIrq  => {
                if usie == 0 {
                    return;
                }
            },
            Interrupt::SupervisorSoftwareIrq  => {
                if ssie == 0 {
                    return;
                }
            },
            Interrupt::MachineSoftwareIrq  => {
                if msie == 0 {
                    return;
                }
            },
            Interrupt::UserTimerIrq  => {
                if utie == 0 {
                    return;
                }
            },
            Interrupt::SupervisorTimerIrq  => {
                if stie == 0 {
                    return;
                }
            },
            Interrupt::MachineTimerIrq  => {
                if mtie == 0 {
                    return;
                }
            },
            Interrupt::UserExtIrq(_)  => {
                if ueie == 0 {
                    return;
                }
            },
            Interrupt::SupervisorExtIrq(_)  => {
                if seie == 0 {
                    return;
                }
            },
            Interrupt::MachineExtIrq(_)  => {
                if meie == 0 {
                    return;
                }
            },
        }

        cpu.csr.priv_level = new_priv_level;
        
        let epc_addr = match cpu.csr.priv_level {
            PrivLevel::MACHINE      => MEPC,
            PrivLevel::SUPERVISOR   => SEPC,
            PrivLevel::USER         => UEPC,
            PrivLevel::RESERVED     => panic!(),
        };
        
        let cause_addr = match cpu.csr.priv_level {
            PrivLevel::MACHINE      => MCAUSE,
            PrivLevel::SUPERVISOR   => SCAUSE,
            PrivLevel::USER         => UCAUSE,
            PrivLevel::RESERVED     => panic!(),
        };
        
        let tval_addr = match cpu.csr.priv_level {
            PrivLevel::MACHINE      => MTVAL,
            PrivLevel::SUPERVISOR   => STVAL,
            PrivLevel::USER         => UTVAL,
            PrivLevel::RESERVED     => panic!(),
        };
        
        let tvec_addr = match cpu.csr.priv_level {
            PrivLevel::MACHINE      => MTVEC,
            PrivLevel::SUPERVISOR   => STVEC,
            PrivLevel::USER         => UTVEC,
            PrivLevel::RESERVED     => panic!(),
        };

        cpu.csr.write(epc_addr, cur_pc as u64);
        cpu.csr.write(cause_addr, cause as u64);
        cpu.csr.write(tval_addr, self.irq());
        cpu.pc = cpu.csr.read(tvec_addr) as usize;

        if (cpu.pc & 0x3) != 0 {
            cpu.pc = (cpu.pc & !0x3) + 4 * (cause as usize & 0xFFFF);
        }

        match cpu.csr.priv_level {
            PrivLevel::MACHINE  => {
                let status  = cpu.csr.read(MSTATUS);
                let mie     = (status >> 3) & 1;
                let new_status = (status & !0x1888) | (mie << 7) | ((cur_priv_level as u64) << 11);
                cpu.csr.write(MSTATUS, new_status);
            },
            PrivLevel::SUPERVISOR   => {
                let status  = cpu.csr.read(SSTATUS);
                let sie     = (status >> 3) & 1;
                let new_status = (status & !0x1888) | (sie << 5) | ((cur_priv_level as u64) << 8);
                cpu.csr.write(SSTATUS, new_status);
            },
            PrivLevel::USER     => unimplemented!(),
            PrivLevel::RESERVED => panic!(),
        }

        match self {
            Interrupt::MachineExtIrq(_) => {
                let data = cpu.csr.read(MIP) & !MIP_MEIP;
                cpu.csr.write(MIP, data);
            },
            Interrupt::MachineSoftwareIrq   => {
                let data = cpu.csr.read(MIP) & !MIP_MSIP;
                cpu.csr.write(MIP, data);
            },
            Interrupt::MachineTimerIrq  => {
                let data = cpu.csr.read(MIP) & !MIP_MTIP;
                cpu.csr.write(MIP, data);
            },
            Interrupt::SupervisorExtIrq(_)  => {
                let data = cpu.csr.read(MIP) & !MIP_SEIP;
                cpu.csr.write(MIP, data);
            },
            Interrupt::SupervisorSoftwareIrq    => {
                let data = cpu.csr.read(MIP) & !MIP_SSIP;
                cpu.csr.write(MIP, data);
            },
            Interrupt::SupervisorTimerIrq   => {
                let data = cpu.csr.read(MIP) & !MIP_STIP;
                cpu.csr.write(MIP, data);
            },
            _   => unimplemented!(),
        }
    }
}