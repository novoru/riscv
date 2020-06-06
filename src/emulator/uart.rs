
/*
 * UART 16550
 * Reference:   TECHNICAL DATA ON 16550
 *              http://byterunner.com/16550.html
 */

// Write mode
const THR: usize    = 0b000;    // Transmit Holding Register
const IER: usize    = 0b001;    // Interrupt Enable Register
const FCR: usize    = 0b010;    // FIFO Control Register
const LCR: usize    = 0b011;    // Line Control Register
const MCR: usize    = 0b100;    // Modem Control Register

// Read mode
const RHR: usize    = 0b000;    // Receive Holding Register
const ISR: usize    = 0b010;    // Interrupt Status Register
const LSR: usize    = 0b101;    // Line Status Register
const MSR: usize    = 0b110;    // Modem Status Register

// Read/Write
const SPR: usize    = 0b111;    // Scatchpad Register

// Flag bit
const IER_RHR_IRQ:              u8  = 0b0000_0001;
const IER_THR_IRQ:              u8  = 0b0000_0010;
const _IER_RCVLINE_STATUS_IRQ:  u8  = 0b0000_0100;
const _IER_MODEM_STATUS_IRQ:    u8  = 0b0000_1000;

const LSR_RX_DATA_READY:        u8  = 0b0000_0001;
const LSR_TX_HOLDING_EMPTY:     u8  = 0b0010_0000;

// Parameter
const BAUDRATE: u16 = 38400;    // ToDo: enable to change baudrate

pub struct Uart {
    clock:  u64,
    thr:    u8,         // Transmit Holding Register
    rhr:    u8,         // Receive Holding Register
    ier:    u8,         // Interrupt Enable Register
    fcr:    u8,         // FIFO control Register
    isr:    u8,         // Interrupt Status Register
    lcr:    u8,         // Line Status Register
    mcr:    u8,         // Line Control Register
    lsr:    u8,         // Modem Control Register
    msr:    u8,         // Modem Status Register
    spr:    u8,         // Scratchpad Register
}

impl Uart {
    pub fn new() -> Self {
        Uart {
            clock:  0,
            thr:    0,
            rhr:    0,
            ier:    0,
            fcr:    0,
            isr:    0,
            lcr:    0,
            mcr:    0,
            lsr:    LSR_TX_HOLDING_EMPTY,
            msr:    0,
            spr:    0,
        }
    }

    pub fn write16(&mut self, addr: usize, data: u16) {
        self.write8(addr, (data & 0xFF) as u8);
        self.write8(addr + 1, ((data >> 8) & 0xFF) as u8);
    }

    pub fn write32(&mut self, addr: usize, data: u32) {
        self.write16(addr, (data & 0xFFFF) as u16);
        self.write16(addr + 2, ((data >> 16) & 0xFFFF) as u16);
    }
    
    pub fn write64(&mut self, addr: usize, data: u64) {
        self.write32(addr, (data & 0xFFFF_FFFF) as u32);
        self.write32(addr + 4, ((data >> 32) & 0xFFFF_FFFF) as u32);
    }

    pub fn write8(&mut self, addr: usize, data: u8) {
        match addr {
            THR => {
                //print!("{}", data as char);
                self.thr = data;
                self.lsr &= !LSR_TX_HOLDING_EMPTY;
            },
            IER => self.ier = data,
            FCR => self.fcr = data,
            LCR => self.lcr = data,
            MCR => self.mcr = data,
            SPR => self.spr = data,
            _   => unimplemented!(),
        }
    }

    pub fn read8(&mut self, addr: usize) -> u8 {
        match addr {
            RHR => {
                // DoDo: update irq
                let rhr = self.rhr;
                self.rhr = 0;
                self.lsr &= !LSR_RX_DATA_READY; 
                rhr
            },
            ISR => self.isr,
            LSR => self.lsr,
            MSR => self.msr,
            SPR => self.spr,
            _   => unimplemented!(),
        }
    }
    
    pub fn read16(&mut self, addr: usize) -> u16 {
        self.read8(addr) as u16 | (self.read8(addr + 1)  as u16) << 8
    }

    pub fn read32(&mut self, addr: usize) -> u32 {
        self.read16(addr) as u32 | (self.read16(addr + 2)  as u32) << 16
    }
    
    pub fn read64(&mut self, addr: usize) -> u64 {
        self.read32(addr) as u64 | (self.read32(addr + 4) as u64) << 32
    }

    pub fn tick(&mut self) {
        self.clock = self.clock.wrapping_add(1);

        /*
        if (self.clock % BAUDRATE) == 0 && self.rhr == 0 {
            let data = input;
            if data != 0 {
                self.rhr = data;
                self.lsr |= LSR_RX_DATA_READY;
            }
        }
        */
        
        if (self.clock % BAUDRATE as u64) == 0 && self.thr != 0 {
            print!("{}", self.thr as char);
            self.thr = 0;
            self.lsr |= LSR_TX_HOLDING_EMPTY;
        }
    }

    pub fn is_interrupting(&mut self) -> bool {
        if (self.ier & IER_RHR_IRQ) != 0 {
            if self.rhr != 0 {
                return true;
            }
        }

        if (self.ier & IER_THR_IRQ) != 0 {
            return true;
        }

        false
    }
}