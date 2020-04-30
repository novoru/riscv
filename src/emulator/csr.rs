pub const CSR_SIZE: usize       = 4096;     // Maximum number of Control and Status registers

// User Trap Setup
pub const USTATUS: u16          = 0x000;    // User status register.
pub const UIE: u16              = 0x004;    // User interrupt-enable register.
pub const UTVEC: u16            = 0x005;    // User trap handler base address.

// User Trap Handling
pub const USCRATCH: u16         = 0x040;    // Scratch register for user trap handlers.
pub const UEPC: u16             = 0x041;    // User exception program counter.
pub const UCAUSE: u16           = 0x042;    // User trap cause.
pub const UTVAL: u16            = 0x043;    // User bad address or instruction.
pub const UIP: u16              = 0x044;    // User interrupt pending.

// User Floating-Point CSRs
pub const FFLAGS: u16           = 0x001;    // Floating-Point Accrued Exceptions.
pub const FRM: u16              = 0x002;    // Floating-Point Dynamic Rounding Mode.
pub const FCSR: u16             = 0x003;    // Floating-Point Control and Status Register (frm + fflags).

// User Counter/Timers
pub const CYCLE: u16            = 0xC00;    // Cycle counter for RDCYCLE instruction.
pub const TIME: u16             = 0xC01;    // Timer for RDTIME instruction.
pub const INSTRET: u16          = 0xC02;    // Instructions-retired counter for RDINSTRET instruction.
pub const HPMCOUNTER3: u16      = 0xC03;    // Performance-monitoring counter.
pub const HPMCOUNTER4: u16      = 0xC04;
pub const HPMCOUNTER5: u16      = 0xC05;
pub const HPMCOUNTER6: u16      = 0xC06;
pub const HPMCOUNTER7: u16      = 0xC07;
pub const HPMCOUNTER8: u16      = 0xC08;
pub const HPMCOUNTER9: u16      = 0xC09;
pub const HPMCOUNTER10: u16     = 0xC0A;
pub const HPMCOUNTER11: u16     = 0xC0B;
pub const HPMCOUNTER12: u16     = 0xC0C;
pub const HPMCOUNTER13: u16     = 0xC0D;
pub const HPMCOUNTER14: u16     = 0xC0E;
pub const HPMCOUNTER15: u16     = 0xC0F;
pub const HPMCOUNTER16: u16     = 0xC10;
pub const HPMCOUNTER17: u16     = 0xC11;
pub const HPMCOUNTER18: u16     = 0xC12;
pub const HPMCOUNTER19: u16     = 0xC13;
pub const HPMCOUNTER20: u16     = 0xC14;
pub const HPMCOUNTER21: u16     = 0xC15;
pub const HPMCOUNTER22: u16     = 0xC16;
pub const HPMCOUNTER23: u16     = 0xC17;
pub const HPMCOUNTER24: u16     = 0xC18;
pub const HPMCOUNTER25: u16     = 0xC19;
pub const HPMCOUNTER26: u16     = 0xC1A;
pub const HPMCOUNTER27: u16     = 0xC1B;
pub const HPMCOUNTER28: u16     = 0xC1C;
pub const HPMCOUNTER29: u16     = 0xC1D;
pub const HPMCOUNTER30: u16     = 0xC1E;
pub const HPMCOUNTER31: u16     = 0xC1F;

// Upper 32 bites of User Counter/Timers (RV32I only)
pub const CYCLEH: u16           = 0xC80;   // Upper 32 bits of cycle, RV32I only.
pub const TIMEH: u16            = 0xC81;   // Upper 32 bits of time, RV32I only.
pub const INSTRETH: u16         = 0xC82;   // Upper 32 bits of instret, RV32I only.
pub const HPMCOUNTER3H: u16     = 0xC83;   // Upper 32 bits of hpmcounter, RV32I only.
pub const HPMCOUNTER4H: u16     = 0xC84;
pub const HPMCOUNTER5H: u16     = 0xC85;
pub const HPMCOUNTER6H: u16     = 0xC86;
pub const HPMCOUNTER7H: u16     = 0xC87;
pub const HPMCOUNTER8H: u16     = 0xC88;
pub const HPMCOUNTER9H: u16     = 0xC89;
pub const HPMCOUNTER10H: u16    = 0xC8A;
pub const HPMCOUNTER11H: u16    = 0xC8B;
pub const HPMCOUNTER12H: u16    = 0xC8C;
pub const HPMCOUNTER13H: u16    = 0xC8D;
pub const HPMCOUNTER14H: u16    = 0xC8E;
pub const HPMCOUNTER15H: u16    = 0xC8F;
pub const HPMCOUNTER16H: u16    = 0xC90;
pub const HPMCOUNTER17H: u16    = 0xC91;
pub const HPMCOUNTER18H: u16    = 0xC92;
pub const HPMCOUNTER19H: u16    = 0xC93;
pub const HPMCOUNTER20H: u16    = 0xC94;
pub const HPMCOUNTER21H: u16    = 0xC95;
pub const HPMCOUNTER22H: u16    = 0xC96;
pub const HPMCOUNTER23H: u16    = 0xC97;
pub const HPMCOUNTER24H: u16    = 0xC98;
pub const HPMCOUNTER25H: u16    = 0xC99;
pub const HPMCOUNTER26H: u16    = 0xC9A;
pub const HPMCOUNTER27H: u16    = 0xC9B;
pub const HPMCOUNTER28H: u16    = 0xC9C;
pub const HPMCOUNTER29H: u16    = 0xC9D;
pub const HPMCOUNTER30H: u16    = 0xC9E;
pub const HPMCOUNTER31H: u16    = 0xC9F;

// Supervisor Trap Setup
pub const SSTATUS: u16          = 0x100;    // Supervisor status register.
pub const SEFELEG: u16          = 0x102;    // Supervisor exception delegation register.
pub const SIDELEG: u16          = 0x103;    // Supervisor interrupt delegation register.
pub const SIE: u16              = 0x104;    // Supervisor interrupt-enable register.
pub const STVEC: u16            = 0x105;    // Supervisor trap handler base address.
pub const SCOUNTEREN: u16       = 0x106;    // Supervisor counter enable.

// Supervisor Trap Handling
pub const SSCRATCH: u16         = 0x140;    // Scratch register for supervisor trap handlers.
pub const SEPC: u16             = 0x141;    // Supervisor exception program counter.
pub const SCAUSE: u16           = 0x142;    // Supervisor trap cause.
pub const STVAL: u16            = 0x143;    // Supervisor bad address or instruction.
pub const SIP: u16              = 0x144;    // Supervisor interrupt pending.

// Supervisor Protection and Translation
pub const SATP: u16             = 0x180;    // Supervisor address translation and protection.

// Machine Information Registers
pub const MVENDORID: u16        = 0xF11;    // Vendor ID.
pub const MARCHID: u16          = 0xF12;    // Architecture ID.
pub const MIMPID: u16           = 0xF13;    // Implementation ID.
pub const MHARTID: u16          = 0xF14;    // Hardware thread ID.

// Machine Trap Setup
pub const MSTATUS: u16          = 0x300;    // Machine status register.
pub const MISA: u16             = 0x301;    // ISA and extensions
pub const MEDELEG: u16          = 0x302;    // Machine exception delegation register.
pub const MIDELEG: u16          = 0x303;    // Machine interrupt delegation register.
pub const MIE: u16              = 0x304;    // Machine interrupt-enable register.
pub const MTVEC: u16            = 0x305;    // Machine trap-handler base address.
pub const MCOUNTEREN: u16       = 0x306;    // Machine counter enable.

// Machine Trap Handling
pub const MSCRATCH: u16         = 0x340;    // Scratch register for machine trap handlers.
pub const MEPC: u16             = 0x341;    // Machine exception program counter.
pub const MCAUSE: u16           = 0x342;    // Machine trap cause.
pub const MTVAL: u16            = 0x343;    // Machine bad address or instruction.
pub const MIP: u16              = 0x344;    // Machine interrupt pending.

// Machine Memory Protection
pub const PMPCFG0: u16          = 0x3A0;    // Physical memory protection configuration.
pub const PMPCFG1: u16          = 0x3A1;    // Physical memory protection configuration, RV32 only.
pub const PMPCFG2: u16          = 0x3A2;    // Physical memory protection configuration.
pub const PMPCFG3: u16          = 0x3A3;    // Physical memory protection configuration, RV32 only.
pub const PMPADDR0: u16         = 0x3B0;    // Physical memory protection address register.
pub const PMPADDR1: u16         = 0x3B1;
pub const PMPADDR2: u16         = 0x3B2;
pub const PMPADDR3: u16         = 0x3B3;
pub const PMPADDR4: u16         = 0x3B4;
pub const PMPADDR5: u16         = 0x3B5;
pub const PMPADDR6: u16         = 0x3B6;
pub const PMPADDR7: u16         = 0x3B7;
pub const PMPADDR8: u16         = 0x3B8;
pub const PMPADDR9: u16         = 0x3B9;
pub const PMPADDR10: u16        = 0x3BA;
pub const PMPADDR11: u16        = 0x3BB;
pub const PMPADDR12: u16        = 0x3BC;
pub const PMPADDR13: u16        = 0x3BD;
pub const PMPADDR14: u16        = 0x3BE;
pub const PMPADDR15: u16        = 0x3BF;

// Machine Counter/Timers
pub const MCYCLE: u16           = 0xB00;    // Machine cycle counter.
pub const MINSTRET: u16         = 0xB02;    // Machine instructions-retired counter.
pub const MHPMCOUNTER3: u16     = 0xB03;    // Machine performance-monitoring counter.
pub const MHPMCOUNTER4: u16     = 0xB04;
pub const MHPMCOUNTER5: u16     = 0xB05;
pub const MHPMCOUNTER6: u16     = 0xB06;
pub const MHPMCOUNTER7: u16     = 0xB07;
pub const MHPMCOUNTER8: u16     = 0xB08;
pub const MHPMCOUNTER9: u16     = 0xB09;
pub const MHPMCOUNTER10: u16    = 0xB0A;
pub const MHPMCOUNTER11: u16    = 0xB0B;
pub const MHPMCOUNTER12: u16    = 0xB0C;
pub const MHPMCOUNTER13: u16    = 0xB0D;
pub const MHPMCOUNTER14: u16    = 0xB0E;
pub const MHPMCOUNTER15: u16    = 0xB0F;
pub const MHPMCOUNTER16: u16    = 0xB10;
pub const MHPMCOUNTER17: u16    = 0xB11;
pub const MHPMCOUNTER18: u16    = 0xB12;
pub const MHPMCOUNTER19: u16    = 0xB13;
pub const MHPMCOUNTER20: u16    = 0xB14;
pub const MHPMCOUNTER21: u16    = 0xB15;
pub const MHPMCOUNTER22: u16    = 0xB16;
pub const MHPMCOUNTER23: u16    = 0xB17;
pub const MHPMCOUNTER24: u16    = 0xB18;
pub const MHPMCOUNTER25: u16    = 0xB19;
pub const MHPMCOUNTER26: u16    = 0xB1A;
pub const MHPMCOUNTER27: u16    = 0xB1B;
pub const MHPMCOUNTER28: u16    = 0xB1C;
pub const MHPMCOUNTER29: u16    = 0xB1D;
pub const MHPMCOUNTER30: u16    = 0xB1E;
pub const MHPMCOUNTER31: u16    = 0xB1F;

pub const MCYCLEH: u16          = 0xB80;    // Upper 32 bits of mcycle, RV32I only.
pub const MINSTRETH: u16        = 0xB82;    // Upper 32 bits of minstret, RV32I only.
pub const MHPMCOUNTER3H: u16    = 0xB83;    // Upper 32 bits of mhpmcounter3, RV32I only.
pub const MHPMCOUNTER4H: u16    = 0xB84;
pub const MHPMCOUNTER5H: u16    = 0xB85;
pub const MHPMCOUNTER6H: u16    = 0xB86;
pub const MHPMCOUNTER7H: u16    = 0xB87;
pub const MHPMCOUNTER8H: u16    = 0xB88;
pub const MHPMCOUNTER9H: u16    = 0xB89;
pub const MHPMCOUNTER10H: u16   = 0xB8A;
pub const MHPMCOUNTER11H: u16   = 0xB8B;
pub const MHPMCOUNTER12H: u16   = 0xB8C;
pub const MHPMCOUNTER13H: u16   = 0xB8D;
pub const MHPMCOUNTER14H: u16   = 0xB8E;
pub const MHPMCOUNTER15H: u16   = 0xB8F;
pub const MHPMCOUNTER16H: u16   = 0xB90;
pub const MHPMCOUNTER17H: u16   = 0xB91;
pub const MHPMCOUNTER18H: u16   = 0xB92;
pub const MHPMCOUNTER19H: u16   = 0xB93;
pub const MHPMCOUNTER20H: u16   = 0xB94;
pub const MHPMCOUNTER21H: u16   = 0xB95;
pub const MHPMCOUNTER22H: u16   = 0xB96;
pub const MHPMCOUNTER23H: u16   = 0xB97;
pub const MHPMCOUNTER24H: u16   = 0xB98;
pub const MHPMCOUNTER25H: u16   = 0xB99;
pub const MHPMCOUNTER26H: u16   = 0xB9A;
pub const MHPMCOUNTER27H: u16   = 0xB9B;
pub const MHPMCOUNTER28H: u16   = 0xB9C;
pub const MHPMCOUNTER29H: u16   = 0xB9D;
pub const MHPMCOUNTER30H: u16   = 0xB9E;
pub const MHPMCOUNTER31H: u16   = 0xB9F;

// Machine Counter Setup
pub const MCOUNTINHIBIT: u16    = 0x320;    // Machine counter-inhibit register.
pub const MHPMEVENT3: u16       = 0x323;    // Machine performance-monitoring event selector.
pub const MHPMEVENT4: u16       = 0x324;
pub const MHPMEVENT5: u16       = 0x325;
pub const MHPMEVENT6: u16       = 0x326;
pub const MHPMEVENT7: u16       = 0x327;
pub const MHPMEVENT8: u16       = 0x328;
pub const MHPMEVENT9: u16       = 0x329;
pub const MHPMEVENT10: u16      = 0x32A;
pub const MHPMEVENT11: u16      = 0x32B;
pub const MHPMEVENT12: u16      = 0x32C;
pub const MHPMEVENT13: u16      = 0x32D;
pub const MHPMEVENT14: u16      = 0x32E;
pub const MHPMEVENT15: u16      = 0x32F;
pub const MHPMEVENT16: u16      = 0x330;
pub const MHPMEVENT17: u16      = 0x331;
pub const MHPMEVENT18: u16      = 0x332;
pub const MHPMEVENT19: u16      = 0x333;
pub const MHPMEVENT20: u16      = 0x334;
pub const MHPMEVENT21: u16      = 0x335;
pub const MHPMEVENT22: u16      = 0x336;
pub const MHPMEVENT23: u16      = 0x337;
pub const MHPMEVENT24: u16      = 0x338;
pub const MHPMEVENT25: u16      = 0x339;
pub const MHPMEVENT26: u16      = 0x33A;
pub const MHPMEVENT27: u16      = 0x33B;
pub const MHPMEVENT28: u16      = 0x33C;
pub const MHPMEVENT29: u16      = 0x33D;
pub const MHPMEVENT30: u16      = 0x33E;
pub const MHPMEVENT31: u16      = 0x33F;

// Debug/Trace Registers (shared with Debug Mode)
pub const TSELECT: u16          = 0x7A0;    // Debug/Trace trigger register select.
pub const TDATA1: u16           = 0x7A1;    // First Debug/Trace trigger data register.
pub const TDATA2: u16           = 0x7A2;    // Second Debug/Trace trigger data register.
pub const TDATA3: u16           = 0x7A3;    // Third Debug/Trace trigger data register.

// Debug Mode Registers
pub const DCSR: u16             = 0x7B0;    // Debug control and status register.
pub const DPC: u16              = 0x7B1;    // Debug PC.
pub const DSCRATCH0: u16        = 0x7B2;    // Debug scratch register 0.
pub const DSCRATCH1: u16        = 0x7B3;    // Debug scratch register 1.

pub struct Csr {
    csr: [u64; CSR_SIZE],
}

impl Csr {
    pub fn new() -> Self {
        Csr { csr: [0; CSR_SIZE] }
    }
    
    pub fn write(&mut self, csr: u16, data: u64) {
        match (csr & 0xC00) >> 10 {     // CSR Address [11:10]
            0b11    => {},              // Read only
            _       => self.csr[csr as usize] = data,
        }
    }

    pub fn read(&self, csr: u16) -> u64 {
        match csr {
            _       => self.csr[csr as usize],
        }
    }
}