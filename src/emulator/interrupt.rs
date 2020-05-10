use crate::emulator::cpu::Cpu;
use crate::emulator::csr::*;
use crate::emulator::plic::{ CONTEXT_BASE };

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
    fn exc_code(&self) -> u8 {
        match self {
            Interrupt::UserSoftwareIrq          => 0x00,
            Interrupt::SupervisorSoftwareIrq    => 0x01,
            Interrupt::MachineSoftwareIrq       => 0x03,
            Interrupt::UserTimerIrq             => 0x04,
            Interrupt::SupervisorTimerIrq       => 0x05,
            Interrupt::MachineTimerIrq          => 0x07,
            Interrupt::UserExtIrq(_irq)         => 0x08,
            Interrupt::SupervisorExtIrq(_irq)   => 0x09,
            Interrupt::MachineExtIrq(_irq)      => 0x0B,
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
        let cur_pc = cpu.pc as u64;
        let prev_level  = cpu.csr.priv_level;

        let mideleg = cpu.csr.read(MIDELEG);
        let sideleg = cpu.csr.read(SIDELEG);

        let pos = self.exc_code() & 0xFF;

        if ((mideleg >> pos) & 1) == 0 {
            cpu.csr.priv_level = PrivLevel::MACHINE;
        }
        else {
            if (sideleg >> pos & 0b1) == 0 {
                cpu.csr.priv_level = PrivLevel::SUPERVISOR;
            }
            else {
                cpu.csr.priv_level = PrivLevel::USER;
            }
        }

        cpu.mmu.write32(cpu.csr, CONTEXT_BASE + 0x1004, self.irq() as u32).unwrap();

        match cpu.csr.priv_level {
            PrivLevel::MACHINE      => {
                let vector = match cpu.csr.read_bit(MTVEC, 0) {
                    true    => (self.exc_code() * 4) as usize,
                    false   => 0,
                };

                cpu.pc = (cpu.csr.read(MTVEC) & !1) as usize + vector;
                cpu.pc -= 4;

                cpu.csr.write(MEPC, cur_pc & !1);
                cpu.csr.write(MCAUSE, (self.exc_code() as u64 | (1 << 63)) as u64);
                cpu.csr.write(MTVAL, 0);
                let mstatus = cpu.csr.read_bit(MSTATUS, 3);
                cpu.csr.write_bit(MSTATUS, 7, mstatus);
                cpu.csr.write_bit(MSTATUS, 3, false);
                cpu.csr.write_bits(MSTATUS, 11..13, 0b00);
            },
            PrivLevel::SUPERVISOR   => {
                let vector = match cpu.csr.read_bit(STVEC, 0) {
                    true    => (self.exc_code() * 4) as usize,
                    false   => 0,
                };

                cpu.pc = (cpu.csr.read(STVEC) & !1) as usize + vector;
                cpu.pc -= 4;

                cpu.csr.write(SEPC, cur_pc & !1);
                cpu.csr.write(SCAUSE, (self.exc_code() as u64 | (1 << 63)) as u64);
                cpu.csr.write(STVAL, 0);
                let sstatus = cpu.csr.read_bit(SSTATUS, 1);
                cpu.csr.write_bit(SSTATUS, 5, sstatus);
                cpu.csr.write_bit(SSTATUS, 1, false);
                match prev_level {
                    PrivLevel::USER => cpu.csr.write_bit(SSTATUS, 8, false),
                    _               => cpu.csr.write_bit(SSTATUS, 8, true),
                }
            },
            PrivLevel::USER         => {
                let vector = match cpu.csr.read_bit(UTVEC, 0) {
                    true    => (self.exc_code() * 4) as usize,
                    false   => 0,
                };

                cpu.pc = (cpu.csr.read(UTVEC) & !1) as usize + vector;
                cpu.pc -= 4;

                cpu.csr.write(UEPC, cur_pc);
                cpu.csr.write(UCAUSE, (self.exc_code() as u64 | (1 << 63)) as u64);
                cpu.csr.write(UTVAL, 0);
            },
            _                       => unimplemented!(),
        }
    }
}