use crate::emulator::memory::Memory;
use crate::emulator::csr::{ Csr, SATP, PrivLevel };
use crate::emulator::exception::{ Exception };

pub const PAGE_SIZE: usize  = 1024 * 4;     // Page size: 4KiB (2**12)
pub const LEVELS: i8        = 3;            // Paging levels (Sv39)
pub const PTE_SIZE: u8      = 8;            // Page teble entry size (Sv39)

#[derive(PartialEq)]
enum ACCESS {
    NONE,
    LOAD,
    STORE,
    EXEC,
}

// Memory Management Unit
pub struct Mmu {
    memory: Memory,
    access: ACCESS,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            memory: Memory::new(),
            access: ACCESS::NONE,
        }
    }

    pub fn read8(&mut self, csr: Csr, vaddr: usize) -> Result<u8, Exception> {
        self.access = ACCESS::LOAD;
        let paddr = self.translate_addr(csr, vaddr)?;
        Ok(self.memory.read8(paddr))
    }
    
    pub fn read16(&mut self, csr: Csr, vaddr: usize) -> Result<u16, Exception> {
        let hi = self.read8(csr, vaddr)?;
        let lo = self.read8(csr, vaddr + 1)?;
        Ok(hi as u16 | (lo as u16) << 8)
    }

    pub fn read32(&mut self, csr: Csr, vaddr: usize) -> Result<u32, Exception> {
        let hi = self.read16(csr, vaddr)?;
        let lo = self.read16(csr, vaddr + 2)?;
        Ok(hi as u32 | (lo as u32) << 16)
    }
    
    pub fn read64(&mut self, csr: Csr, vaddr: usize) -> Result<u64, Exception> {
        let hi = self.read32(csr, vaddr)?;
        let lo = self.read32(csr, vaddr + 4)?;
        Ok(hi as u64 | (lo as u64) << 32)
    }

    pub fn write8(&mut self, vaddr: usize, data: u8) {
        self.access = ACCESS::STORE;
        self.memory.rom[vaddr] = data;
    }

    pub fn write16(&mut self, vaddr: usize, data: u16) {
        self.write8(vaddr, (data & 0xFF) as u8);
        self.write8(vaddr + 1, (data >> 8 & 0xFF) as u8);
    }

    pub fn write32(&mut self, vaddr: usize, data: u32) {
        self.write16(vaddr, (data & 0xFFFF) as u16);
        self.write16(vaddr + 2, (data >> 16 & 0xFFFF) as u16);
    }

    pub fn write64(&mut self, vaddr: usize, data: u64) {
        self.write32(vaddr, (data & 0xFFFF_FFFF) as u32);
        self.write32(vaddr + 4, (data >> 32 & 0xFFFF_FFFF) as u32);
    }

    // Translate virtual address to physical address (Sv39)
    // Reference:   RISC-V Privileged ISA Specification p.71~
    //              https://riscv.org/specifications/privileged-isa/
    fn translate_addr(&mut self, csr: Csr, vaddr: usize) -> Result<usize, Exception> {

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

        let vpn         = |i| ((vaddr >> (12+9*i)) & 0x1FF);
        let pte_ppn     = |pte, i| ((pte >> (10+9*i)) & 0x1FF);
        let pte_v       = |pte: u64| (pte & 1u64);
        let pte_r       = |pte: u64| ((pte >> 1) & 1u64);
        let pte_w       = |pte: u64| ((pte >> 2) & 1u64);
        let pte_x       = |pte: u64| ((pte >> 3) & 1u64);
        let pte_u       = |pte: u64| ((pte >> 4) & 1u64);
        let _pte_g      = |pte: u64| ((pte >> 5) & 1u64);
        let pte_a       = |pte: u64| ((pte >> 6) & 1u64);
        let pte_d       = |pte: u64| ((pte >> 7) & 1u64);
        let va_pgoff    = |vaddr| (vaddr & 0xFFF);

        // Step 1
        let satp_ppn = csr.read(SATP) & 0x3F_FFFF;

        let mut a = satp_ppn as usize * PAGE_SIZE;
        let mut i: i8 = LEVELS - 1;
        let mut pte: u64 = 0;

        // Step 2
        while i >= 0 {

            let addr = a + vpn(i) * (PTE_SIZE as usize);

            /* ToDo
            if violate_pma(addr) || violate_pmp(addr) {
                page_fault_exception();
            }
            */

            pte = self.memory.read64(addr);

            // Step 3
            if pte_v(pte) == 0u64 || pte_w(pte) == 1u64 {
                self.page_fault_exception()?;
            }

            // Step 4
            if pte_r(pte) == 1u64 || pte_x(pte) == 1u64 {
                break;
            }

            i -= 1;

            if i < 0 {
                self.page_fault_exception()?;
            }

            a = (pte_ppn(pte, i) as usize) * PAGE_SIZE;
        }

        // Step 5
        if csr.priv_level == PrivLevel::USER && pte_u(pte) == 0 {
            self.page_fault_exception()?;
        }

        if self.access == ACCESS::LOAD && pte_r(pte) == 0u64 {
            self.page_fault_exception()?;
        }

        if self.access == ACCESS::STORE && pte_x(pte) == 0u64 {
            self.page_fault_exception()?;
        }

        if self.access == ACCESS::EXEC && pte_x(pte) == 0u64 {
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
            self.page_fault_exception()?;
        }

        // Step 8

        let pa_pgoff = va_pgoff(vaddr);

        /* ToDo
        if i > 0 {
            pa.ppn[i-1:0] = va.vpn[i-1:0];
        }
        */

        let pa_ppn: usize = pte_ppn(pte, i) as usize;

        Ok((pa_ppn << 22) + pa_pgoff)   // Physical address
    }

    fn page_fault_exception(&self) -> Result<(), Exception> {
        match self.access {
            ACCESS::LOAD    => return Err(Exception::LoadPageFault),
            ACCESS::STORE   => return Err(Exception::StorePageFault),
            _               => unimplemented!(),
        }
    }

}