// MMIO Device Legacy Register Layout
const MAGIC_VALUE_BASE:         usize   = 0x000;
const MAGIC_VALUE_TOP:          usize   = 0x003;
const VERSION:                  usize   = 0x004;
const DEVICE_ID:                usize   = 0x008;
const VENDOR_ID_BASE:           usize   = 0x00C;
const VENDOR_ID_TOP:            usize   = 0x00F;
const HOST_FEATURES_BASE:       usize   = 0x010;
const HOST_FEATURES_TOP:        usize   = 0x013;
const HOST_FEATURES_SEL_BASE:   usize   = 0x014;
const HOST_FEATURES_SEL_TOP:    usize   = 0x017;
const GUEST_FEATURES_BASE:      usize   = 0x020;
const GUEST_FEATURES_TOP:       usize   = 0x023;
const GUEST_FEATURES_SEL_BASE:  usize   = 0x024;
const GUEST_FEATURES_SEL_TOP:   usize   = 0x027;
const GUEST_PAGE_SIZE_BASE:     usize   = 0x028;
const GUEST_PAGE_SIZE_TOP:      usize   = 0x029;
const QUEUE_SEL_BASE:           usize   = 0x030;
const QUEUE_SEL_TOP:            usize   = 0x033;
const QUEUE_NUM_MAX_BASE:       usize   = 0x034;
const QUEUE_NUM_MAX_TOP:        usize   = 0x037;
const QUEUE_NUM_BASE:           usize   = 0x038;
const QUEUE_NUM_TOP:            usize   = 0x03B;
const QUEUE_ALIGN_BASE:         usize   = 0x03C;
const QUEUE_ALIGN_TOP:          usize   = 0x03F;
const QUEUE_PFN_BASE:           usize   = 0x040;
const QUEUE_PFN_TOP:            usize   = 0x043;
const QUEUE_NOTIFY_BASE:        usize   = 0x050;
const QUEUE_NOTIFY_TOP:         usize   = 0x053;
const INTERRUPT_STATUS_BASE:    usize   = 0x060;
const INTERRUPT_STATUS_TOP:     usize   = 0x063;
const INTERRUPT_ACK:            usize   = 0x064;
const STATUS_BASE:              usize   = 0x070;
const STATUS_TOP:               usize   = 0x073;

#[derive(Copy, Clone, Debug)]
pub enum DeviceID {
    Reserved                    = 0,
    NetworkCard                 = 1,
    BlockDevice                 = 2,
    Console                     = 3,
    EntropySource               = 4,
    MomoryBalooning             = 5,
    IoMemory                    = 6,
    Rpmsg                       = 7,
    ScsiHost                    = 8,
    _9pTransport                = 9,
    Mac80211Wlan                = 10,
    RprocSerial                 = 11,
    VirtioCaif                  = 12,
    MemoryBaloon                = 13,
    GpuDevice                   = 16,
    TimerClockDevice            = 17,
    InputDevice                 = 18,
    SocketDevic                 = 19,
    CryptoDevice                = 20,
    SignalDistrivutionModule    = 21,
    PstoreDevice                = 22,
    IommuDevice                 = 23,
    MemoryDevice                = 24,
}

#[derive(Debug)]
pub struct Virtio {
    magic_value:        u32,
    vendor_id:          u32,
    version:            u8,
    device_id:          DeviceID,
    host_features_sel:  u32,
    feature_bits:       u32,
    guest_features:     u32,
    guest_features_sel: u32,
    guest_page_size:    u16,
    queue_sel:          u32,
    queue_num_max:      u64,
    queue_num:          u32,
    queue_align:        u32,
    queue_pfn:          u32,
    queue_notify:       u32,
    interrupt_status:   u64,
    status:             u32,
    disk:               Vec<u8>,
}

impl Virtio {
    pub fn new(device_id: DeviceID) -> Self {
        Virtio {
            magic_value:        0x74726976,     // 0x74726976 (A Little Endian equivalent of the “virt” string)
            vendor_id:          0x554d4551,     // QEMU's Vendor ID 0x1AF4
            version:            0x1,            // Legacy devices used 0x1.
            device_id:          device_id,      // Virtio Subsystem Device ID
            host_features_sel:  0,
            feature_bits:       0,
            guest_features:     0,
            guest_features_sel: 0,
            guest_page_size:    0,
            queue_sel:          0,
            queue_num_max:      0x2000,
            queue_num:          0,
            queue_align:        0,
            queue_pfn:          0,
            queue_notify:       0,
            interrupt_status:   0,
            status:             0,
            disk:               Vec::new(),
        }
    }

    pub fn write8(&mut self, addr: usize, data: u8) {
        match addr {
            HOST_FEATURES_SEL_BASE ..= HOST_FEATURES_SEL_TOP   => {
                let shift = (addr - HOST_FEATURES_SEL_BASE) * 8;
                self.host_features_sel = self.host_features_sel & !(0xFF << shift) | (data as u32) << shift;
            },
            GUEST_FEATURES_BASE ..= GUEST_FEATURES_TOP  => {
                let shift = (addr - GUEST_FEATURES_BASE) * 8;
                self.guest_features = self.guest_features & !(0xFF << shift) | (data as u32) << shift;
            },
            GUEST_FEATURES_SEL_BASE ..= GUEST_FEATURES_SEL_TOP  => {
                let shift = (addr - GUEST_FEATURES_SEL_BASE) * 8;
                self.guest_features_sel = self.guest_features_sel & !(0xFF << shift) | (data as u32) << shift;
            },
            GUEST_PAGE_SIZE_BASE ..= GUEST_PAGE_SIZE_TOP  => {
                let shift = (addr - GUEST_PAGE_SIZE_BASE) * 8;
                self.guest_page_size = self.guest_page_size & !(0xFF << shift) | (data as u16) << shift;
            },
            QUEUE_SEL_BASE ..= QUEUE_SEL_TOP    => {
                let shift = (addr - QUEUE_SEL_BASE) * 8;
                self.queue_sel = self.queue_sel & !(0xFF << shift) | (data as u32) << shift;
            },
            QUEUE_NUM_BASE ..= QUEUE_NUM_TOP   => {
                let shift = (addr - QUEUE_NUM_BASE) * 8;
                self.queue_num = self.queue_num & !(0xFF << shift) | (data as u32) << shift;
            },
            QUEUE_ALIGN_BASE ..= QUEUE_ALIGN_TOP   => {
                let shift = (addr - QUEUE_ALIGN_BASE) * 8;
                self.queue_align = self.queue_align & !(0xFF << shift) | (data as u32) << shift;
            },
            QUEUE_PFN_BASE ..= QUEUE_PFN_TOP => {
                let shift = (addr - QUEUE_PFN_BASE) * 8;
                self.queue_pfn = self.queue_pfn & !(0xFF << shift) | (data as u32) << shift;
            },
            QUEUE_NOTIFY_BASE ..= QUEUE_NOTIFY_TOP  => {
                let shift = (addr - QUEUE_NOTIFY_BASE) * 8;
                self.queue_notify = self.queue_notify & !(0xFF << shift) | (data as u32) << shift;
            },
            INTERRUPT_ACK       => {
                if data & 0x1 == 1 {
                    self.interrupt_status &= !0x1;
                }
                else {
                    panic!();
                }
            },
            STATUS_BASE ..= STATUS_TOP  => {
                let shift = (addr - STATUS_BASE) * 8;
                self.status = self.status & !(0xFF << shift) | (data as u32) << shift;
            },
            _                   => (),
        }
    }


    pub fn write16(&mut self, addr: usize, data: u16) {
        self.write8(addr, (data as u8) & 0xFF);
        self.write8(addr + 1, ((data >> 8) as u8) & 0xFF);
    }

    pub fn write32(&mut self, addr: usize, data: u32) {
        self.write16(addr, (data as u16) & 0xFFFF);
        self.write16(addr + 2, ((data >> 16) as u16) & 0xFFFF);
    }

    pub fn write64(&mut self, addr: usize, data: u64) {
        self.write32(addr, (data as u32) & 0xFFFF_FFFF);
        self.write32(addr + 4, ((data >> 32) as u32) & 0xFFFF_FFFF);
    }

    pub fn read8(&self, addr: usize) -> u8 {
        match addr {
            // Magic value
            MAGIC_VALUE_BASE ..= MAGIC_VALUE_TOP    =>{
                let shift = addr * 8;
                ((self.magic_value >> shift) & 0xFF) as u8
            },
            // Device version number
            VERSION     => self.version,
            // Virtio Subsystem Device ID 
            DEVICE_ID   => self.device_id as u8,
            // Virtio Subsystem Vendor ID 
            VENDOR_ID_BASE ..= VENDOR_ID_TOP    => {
                let shift = (addr - VENDOR_ID_BASE) * 8;
                ((self.vendor_id >> shift) & 0xFF) as u8
            },
            // Flags representing features the device supports 
            HOST_FEATURES_BASE ..= HOST_FEATURES_TOP    => {
                let shift = (addr - HOST_FEATURES_BASE) * 8;
                ((self.feature_bits >> shift) & 0xFF) as u8
            },
            // Maximum virtual queue size
            QUEUE_NUM_MAX_BASE ..= QUEUE_NUM_MAX_TOP    => {
                let shift = (addr - QUEUE_NUM_MAX_BASE) * 8;
                ((self.queue_num_max >> shift) & 0xFF) as u8
            },
            INTERRUPT_STATUS_BASE ..= INTERRUPT_STATUS_TOP  => {
                let shift = (addr - INTERRUPT_STATUS_BASE) * 8;
                ((self.interrupt_status >> shift) & 0xFF) as u8
            },
            STATUS_BASE ..= STATUS_TOP  => {
                let shift = (addr - STATUS_BASE) * 8;
                ((self.status >> shift) & 0xFF) as u8
            },
            _                   => 0,
        }
    }
    pub fn read16(&self, addr: usize) -> u16 {
          self.read8(addr) as u16 |
        ((self.read8(addr + 1) as u16) << 8)
    }

    pub fn read32(&self, addr: usize) -> u32 {
        self.read16(addr) as u32 |
      ((self.read16(addr + 2) as u32) << 16)
    }

    pub fn read64(&self, addr: usize) -> u64 {
        self.read32(addr) as u64 |
      ((self.read32(addr + 4) as u64) << 32)
    }
}