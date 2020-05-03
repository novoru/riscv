use crate::emulator::cpu::Cpu;

#[derive(Debug)]
pub enum Exception {
    InstAddrMisalign    = 0,
    InstAccessFault     = 1,
    IllegalInst         = 2,
    Breakpoint          = 3,
    LoadAddrMislign     = 4,
    LoadAccessFault     = 5,
    StoreAddrMisalign   = 6,
    StoreAccessFault    = 7,
    EnvCallUmode        = 8,
    EnvCallSmode        = 9,
    // 10: Reserved
    EnvCallMmode        = 11,
    InstPageFault       = 12,
    LoadPageFault       = 13,
    // 14: Reserved for future standard use
    StorePageFault      = 15,
    // 15~: Reserved
}

impl Exception {
    pub fn take_trap(&self, _cpu: &mut Cpu) {
        unimplemented!();
    }
}