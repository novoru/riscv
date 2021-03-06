use crate::emulator::csr::{ Csr, SATP, PrivLevel };
use crate::emulator::exception::{ Exception };
use crate::emulator::bus::Bus;
use crate::emulator::interrupt::IrqNumber;

pub const PAGE_SIZE: usize  = 1024 * 4;     // Page size: 4KiB (2**12)
pub const LEVELS: i8        = 3;            // Paging levels (Sv39)
pub const PTE_SIZE: usize   = 8;            // Page teble entry size (Sv39)

#[derive(PartialEq)]
enum ACCESS {
    NONE,
    LOAD,
    STORE,
    EXEC,
}

// Memory Management Unit
pub struct Mmu {
    bus: Bus,
    access: ACCESS,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            bus: Bus::new(),
            access: ACCESS::NONE,
        }
    }

    pub fn load_dram(&mut self, binary: Vec<u8>) {
        self.bus.load_dram(binary);
    }

    pub fn load_disk(&mut self, binary: Vec<u8>) {
        self.bus.load_disk(binary);
    }

    pub fn tick(&mut self, mip: &mut u64) {
        self.bus.tick(mip);
    }

    pub fn get_irqno(&self) -> Option<IrqNumber> {
        self.bus.get_irqno()
    }
    
    pub fn read8(&mut self, csr: &Csr, vaddr: usize) -> Result<u8, Exception> {
        self.access = ACCESS::LOAD;
        let paddr = self.translate_addr(&csr, vaddr)?;
        Ok(self.bus.read8(paddr))
    }
    
    pub fn read16(&mut self, csr: &Csr, vaddr: usize) -> Result<u16, Exception> {
        self.access = ACCESS::LOAD;
        let paddr = self.translate_addr(&csr, vaddr)?;
        Ok(self.bus.read16(paddr))
    }

    pub fn read32(&mut self, csr: &Csr, vaddr: usize) -> Result<u32, Exception> {
        self.access = ACCESS::LOAD;
        let paddr = self.translate_addr(&csr, vaddr)?;
        Ok(self.bus.read32(paddr))
    }
    
    pub fn read64(&mut self, csr: &Csr, vaddr: usize) -> Result<u64, Exception> {
        self.access = ACCESS::LOAD;
        let paddr = self.translate_addr(&csr, vaddr)?;
        Ok(self.bus.read64(paddr))
    }

    pub fn fetch32(&mut self, csr: &Csr, vaddr: usize) -> Result<u32, Exception> {
        self.access = ACCESS::EXEC;
        let paddr = self.translate_addr(&csr, vaddr)?;
        Ok(self.bus.read32(paddr))
    }

    pub fn write8(&mut self, csr: &Csr, vaddr: usize, data: u8) -> Result<(), Exception>  {
        self.access = ACCESS::STORE;
        let paddr = self.translate_addr(&csr, vaddr)?;
        self.bus.write8(paddr, data);
        Ok(())
    }

    pub fn write16(&mut self, csr: &Csr, vaddr: usize, data: u16) -> Result<(), Exception> {
        self.access = ACCESS::STORE;
        let paddr = self.translate_addr(&csr, vaddr)?;
        self.bus.write16(paddr, data);
        Ok(())
    }

    pub fn write32(&mut self, csr: &Csr, vaddr: usize, data: u32) -> Result<(), Exception> {
        self.access = ACCESS::STORE;
        let paddr = self.translate_addr(&csr, vaddr)?;
        self.bus.write32(paddr, data);
        Ok(())
    }

    pub fn write64(&mut self, csr: &Csr, vaddr: usize, data: u64) -> Result<(), Exception> { 
        self.access = ACCESS::STORE;
        let paddr = self.translate_addr(&csr, vaddr)?;
        self.bus.write64(paddr, data);
        Ok(())
    }

    // Translate virtual address to physical address (Sv39)
    // Reference:   RISC-V Privileged ISA Specification p.71~
    //              https://riscv.org/specifications/privileged-isa/
    pub fn translate_addr(&mut self, csr: &Csr, vaddr: usize) -> Result<usize, Exception> {
        /*
         *  Sv39 virtual address
         * 
         *  38     30 29    21 20    12 11           0
         *  +--------+--------+--------+-------------+
         *  | VPN[2] | VPN[1] | VPN[0] | page offset |
         *  +--------+--------+--------+-------------+
         * 
         * 
         *  Sv39 physical address
         * 
         *  55         30 29        21 20        12 11           0
         *  +------------+------------+------------+-------------+
         *  |   PPN[2]   |   PPN[1]   |   PPN[0]   | page offset |
         *  +------------+------------+------------+-------------+
         * 
         * 
         *  Sv39 page table entry
         * 
         *  63       54 53    28 27    19 18    10 9   8 7 6 5 4 3 2 1 0
         *  +----------+--------+--------+--------+-----+-+-+-+-+-+-+-+-+
         *  | Reserved | PPN[2] | PPN[1] | PPN[0] | RSW |D|A|G|U|X|W|R|V|
         *  +----------+--------+--------+--------+-----+-+-+-+-+-+-+-+-+
         * 
         */

        if csr.priv_level == PrivLevel::MACHINE {
            return Ok(vaddr);
        }

        if csr.read_bits(SATP, 60..64) == 0 {
            return Ok(vaddr);
        }

        let vpn = [ (vaddr >> 12) & 0x1FF,
                    (vaddr >> 21) & 0x1FF,
                    (vaddr >> 30) & 0x1FF
                ];
        let pte_v       = |pte: u64| (pte & 1u64);
        let pte_r       = |pte: u64| ((pte >> 1) & 1u64);
        let pte_w       = |pte: u64| ((pte >> 2) & 1u64);
        let pte_x       = |pte: u64| ((pte >> 3) & 1u64);
        let pte_u       = |pte: u64| ((pte >> 4) & 1u64);
        let _pte_g      = |pte: u64| ((pte >> 5) & 1u64);
        let pte_a       = |pte: u64| ((pte >> 6) & 1u64);
        let pte_d       = |pte: u64| ((pte >> 7) & 1u64);

        // Step 1
        let satp_ppn = csr.read(SATP) & 0x3F_FFFF;
        
        let mut a = satp_ppn as usize * PAGE_SIZE;
        let mut i: i8 = LEVELS - 1;
        let mut pte: u64 = 0;
        let mut ppn: usize = 0;
        let mut vpte = [0; LEVELS as usize];
        let mut addr = 0;
        
        // Step 2
        while i >=0 {
            addr = a + vpn[i as usize] * PTE_SIZE;

            /* ToDo: implement PMA
            if violate_pma(addr) || violate_pmp(addr) {
                page_fault_exception();
            }
            */

            pte = self.bus.read64(addr);
            vpte[i as usize] = pte;
            //eprintln!("[DEBUG] vaddr: 0x{:x}, level: {}, pte addr: 0x{:x}, pte: 0x{:x}", vaddr, i, addr, pte);

            // Step 3
            if pte_v(pte) == 0u64 || (pte_r(pte) == 0u64 && pte_w(pte) == 1u64) {
                eprintln!("[DEBUG] {}-{}: page fault exception", file!(), line!());
                self.page_fault_exception()?;
            }

            ppn = ((pte >> 10) & 0xFFF_FFFF_FFFF) as usize;
            a = ppn * PAGE_SIZE;

            // Step 4
            if pte_r(pte) == 1u64 || pte_x(pte) == 1u64 {
                break;
            }

            i -= 1;

            if i < 0 {
                eprintln!("[DEBUG] {}-{}: page fault exception", file!(), line!());
                self.page_fault_exception()?;
            }

        }

        // Step 5
        if csr.priv_level == PrivLevel::USER && pte_u(pte) == 0u64 {
            eprintln!("[DEBUG] {}-{}: page fault exception", file!(), line!());
            self.page_fault_exception()?;
        }

        if self.access == ACCESS::LOAD && pte_r(pte) == 0u64 {
            eprintln!("[DEBUG] {}-{}: page fault exception", file!(), line!());
            self.page_fault_exception()?;
        }

        if self.access == ACCESS::STORE && pte_w(pte) == 0u64 {
            eprintln!("[DEBUG] {}-{}: page fault exception", file!(), line!());
            self.page_fault_exception()?;
        }

        if self.access == ACCESS::EXEC && pte_x(pte) == 0u64 {
            eprintln!("[DEBUG] {}-{}: page fault exception", file!(), line!());
            self.page_fault_exception()?;
        }

        // Step 6
        /* ToDo
        if i > 0 && pte.ppn[i-1:0] != 0 {
            page_fault_exception();
        }
        */

        // Step 7
        if pte_a(pte) == 0u64 || (self.access == ACCESS::STORE && pte_d(pte) == 0u64) {
            self.bus.write64(addr, pte | (1 << 6));     // set pte.a to 1
            if self.access == ACCESS::STORE {
                self.bus.write64(addr, pte | (1 << 7));     // also set pte.d to 1 
            }
        }

        // Step 8

        let pa_pgoff = vaddr & 0xFFF;
        let ppns = [ ((vpte[0] >> 10) & 0x1FF) as usize,
                     ((vpte[1] >> 19) & 0x1FF) as usize,
                     ((vpte[2] >> 28) & 0x3FF_FFFF) as usize
                ];
        
        //eprintln!("[DEBUG] ppn2: 0x{:x}, ppn1: 0x{:x}, ppn0: 0x{:x}", ppns[2], ppns[1], ppns[0]);
        //eprintln!("[DEBUG] vpn2: 0x{:x}, vpn1: 0x{:x}, vpn0: 0x{:x}", vpn[2], vpn[1], vpn[0]);
        //eprintln!("[DEBUG] pa_pgoff: 0x{:x}", pa_pgoff);

        //eprintln!("[DEBUG] level: {}", i);

        let paddr = match i {
            2   => (ppns[2] << 30) | (vpn[1]  << 21) | (vpn[0] << 12) + pa_pgoff,
            1   => (ppns[2] << 30) | (ppns[1] << 21) | (vpn[0] << 12) + pa_pgoff,
            0   => (ppn << 12) + pa_pgoff,
            _   => panic!(),
        };

        //eprintln!("[DEBUG] paddr: 0x{:16x}", paddr);

        Ok(paddr)
    }

    fn page_fault_exception(&self) -> Result<(), Exception> {
        match self.access {
            ACCESS::LOAD    => Err(Exception::LoadPageFault),
            ACCESS::STORE   => Err(Exception::StorePageFault),
            ACCESS::EXEC    => Err(Exception::InstPageFault),
            _               => unimplemented!(),
        }
    }

}