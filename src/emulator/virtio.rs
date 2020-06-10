// Virtual I/O Device (VIRTIO) Version 1.1 
// https://docs.oasis-open.org/virtio/virtio/v1.1/cs01/virtio-v1.1-cs01.html

use crate::emulator::bus::DRAM_BASE;
use crate::emulator::dram::*;

const MAX_DISK:                 usize   = 1024 * 1024 * 128;  // 128MiB
const SECTOR_SIZE:              usize   = 512;

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

/* This marks a buffer as continuing via the next field. */ 
const VIRTQ_DESC_F_NEXT:            u8  = 1;
/* This marks a buffer as write-only (otherwise read-only). */
const VIRTQ_DESC_F_WRITE:           u8  = 2;
/* This means the buffer contains a list of buffer descriptors. */
const _VIRTQ_DESC_F_INDIRECT:        u8  = 4;

/* The device uses this in used->flags to advise the driver: don’t kick me 
 * when you add a buffer.  It’s unreliable, so it’s simply an 
 * optimization. */ 
const _VIRTQ_USED_F_NO_NOTIFY:       u8  = 1;
/* The driver uses this in avail->flags to advise the device: don’t 
 * interrupt me when you consume a buffer.  It’s unreliable, so it’s 
 * simply an optimization.  */ 
const _VIRTQ_AVAIL_F_NO_INTERRUPT:   u8  = 1;

/* Support for indirect descriptors */
const _VIRTIO_F_INDIRECT_DESC:       u8  = 28;

/* Support for avail_event and used_event fields */
const _VIRTIO_F_EVENT_IDX:           u8  = 29;

/* Arbitrary descriptor layouts. */
const _VIRTIO_F_ANY_LAYOUT:          u8  = 27;

// Device Status Field
const _STATUS_ACK:               u8  = 1;
const _STATUS_DRIVER:            u8  = 2;
const _STATUS_FAILED:            u8  = 128;
const _STATUS_FEATURED:          u8  = 8;
const _STATUS_DRIVER_OK:         u8  = 4;
const _STATUS_DEVICE_NEED_RESET: u8  = 64;

// Feature bits
const _VIRTIO_BLK_F_BARRIER:         u8  = 0;    // only legacy interface
const _VIRTIO_BLK_F_SIZE_MAX:        u8  = 1;
const _VIRTIO_BLK_F_SEG_MAX:         u8  = 2;
const _VIRTIO_BLK_F_GEOMETRY:        u8  = 4;
const _VIRTIO_BLK_F_RO:              u8  = 5;
const _VIRTIO_BLK_F_BLK_SIZE:        u8  = 6;
const _VIRTIO_BLK_F_SCSI:            u8  = 7;    // only legacy interface
const _VIRTIO_BLK_F_FLUSH:           u8  = 9;    // also called  VIRTIO_BLK_F_WCE
const _VIRTIO_BLK_F_TOPOLOGY:        u8  = 10;
const _VIRTIO_BLK_F_CONFIG_WCE:      u8  = 11;
const _VIRTIO_BLK_F_DISCARD:         u8  = 13;
const _VIRTIO_BLK_F_WRITE_ZEROES:    u8  = 14;

// Reserved Feature Bits
const _VIRTIO_F_RING_INDIRECT_DESC : u8  = 28;
const _VIRTIO_F_RING_EVENT_IDX:      u8  = 29;
const _VIRTIO_F_VERSION_1:           u8  = 32;
const _VIRTIO_F_ACCESS_PLATFORM:     u8  = 33;
const _VIRTIO_F_RING_PACKED:         u8  = 34;
const _VIRTIO_F_IN_ORDER:            u8  = 35;
const _VIRTIO_F_ORDER_PLATFORM:      u8  = 36;
const _VIRTIO_F_SR_IOV:              u8  = 37;
const _VIRTIO_F_NOTIFICATION_DATA:   u8  = 38;

// The type of the request (virtio_blk_req.type)
// ToDo: rewrite to enum
const _VIRTIO_BLK_T_IN:              u8  = 0;
const _VIRTIO_BLK_T_OUT:             u8  = 1;
const _VIRTIO_BLK_T_FLUSH:           u8  = 4;
const _VIRTIO_BLK_T_DISCARD:         u8  = 11;
const _VIRTIO_BLK_T_WRITE_ZEROES:    u8  = 13;

// The final status byte (virtio_blk_req.status)
const _VIRTIO_BLK_S_OK:              u8  = 0;
const _VIRTIO_BLK_S_IOERR:           u8  = 1;
const _VIRTIO_BLK_S_UNSUPP:          u8  = 2;

// For legacy interface
const _VIRTIO_BLK_T_FLUSH_OUT:       u8  = 5;
const _VIRTIO_BLK_T_BARRIER:         u32 = 0x80000000;
const _VIRTIO_BLK_T_SCSI_CMD:        u8  = 2;
const _VIRTIO_BLK_T_SCSI_CMD_OUT:    u8  = 3;

#[derive(Debug)]
#[repr(C)]
#[repr(align(4096))]
struct virtq {
    desc:   Vec<virtq_desc>,
    avail:  Vec<virtq_avail>,
    used:   Vec<virtq_used>,
}

/* Virtqueue descriptors: 16 bytes. 
 * These can chain together via "next". */ 
#[derive(Debug)]
#[repr(C)]
struct virtq_desc {
    /* Address (guest-physical). */ 
    addr:   usize,
    /* Length. */ 
    len:    u32,

    /*
     * flags    
     *  0x1: This marks a buffer as continuing via the next field.
     *  0x2: This marks a buffer as write-only (otherwise read-only).
     *  0x4: This means the buffer contains a list of buffer descriptors.
     */
    flags:  u16,


    /* Next descriptor number */ 
    next:   u16,
}

#[derive(Debug)]
#[repr(C)]
struct virtq_avail {
    /*
     * Flags
     *  0x1:    The driver uses this in avail->flags to advise the device: don’t
     *          interrupt me when you consume a buffer.  It’s unreliable, so it’s
     *          simply an optimization.
     */
    flags:      u16,
    idx:        u16,
    ring:       Vec<u16>,
    used_event: u16,
}

#[derive(Debug)]
#[repr(C)]
struct virtq_used_elem {
    /* Index of start of used descriptor chain. */ 
    id:     u32,
    /* Total length of the descriptor chain which was written to. */ 
    len:    u32,
}

#[derive(Debug)]
#[repr(C)]
struct virtq_used {
    /*
     * Flags
     *  0x1:    The device uses this in used->flags to advise the driver: don’t kick me
     *          when you add a buffer.  It’s unreliable, so it’s simply an optimization. 
     */
    flags:          u16,
    idx:            u16,
    ring:           Vec<virtq_used_elem>,
    avail_event:    u16,
}

#[derive(Debug)]
#[repr(C)]
struct virtio_blk_config {
    capacity:   u64,
    size_max:   u32,
    seg_max:    u32,
    geometry:   vertio_blk_geometry,
    blk_size:   u32,
    topology:   virtio_blk_topology,
    writeback:  u8,
    unuesed0:   [u8; 3],
    max_discard_sectors:        u32,
    max_discard_seg:            u32,
    discard_sector_alignment:   u32,
    max_write_zeroes_sectors:   u32,
    max_write_zeroes_seg:       u32,
    write_zeroes_may_unmap:     u8,
    unuesed1:   [u8; 3],
}

#[derive(Debug)]
#[repr(C)]
struct vertio_blk_geometry {
    cylinders:  u16,
    heads:      u8,
    sectors:    u8,
}

#[derive(Debug)]
#[repr(C)]
struct virtio_blk_topology {
    // # of logical blocks per physical block (log2) 
    physical_block_exp: u8,
    // offset of first aligned logical block 
    alignment_ofset:    u8,
    // suggested minimum I/O size in blocks 
    min_io_size:        u16,
    // optimal (suggested maximum) I/O size in blocks 
    opt_io_size:        u32,
}

#[derive(Debug)]
#[repr(C)]
struct virtio_blk_req {
    r#type:     u32,
    reserved:   u32,        // also called ioprio(legacy interface)
    sector:     u64,
    data:       Vec<u8>,
    status:     u8,
}

#[derive(Debug)]
#[repr(C)]
struct virtio_blk_discard_write_zeroes {
    sectors:        u64,
    num_sectors:    u32,
    flags:          (u32, u32),     // (unmap, reserved)
}

#[derive(Debug)]
#[repr(C)]
struct virtio_scsi_pc_req {
    r#type:     u32,
    ioprio:     u32,
    sector:     u64,
    cmd:        Vec<u8>,
    data:       Vec<Vec<u8>>,
    sense:      Vec<u8>,
    errors:     u32,
    data_len:   u32,
    sense_len:  u32,
    residual:   u32,
    status:     u8,
}

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
    clock:              u64,
    disk:               Vec<u8>,
    last_avail_idx:     usize,      // The number of the last entry on the Available Ring
    notify_changed:     bool,       // dirty bit of the queue notifier register

    // MMIO Device Legacy Register
    magic_value:        u32,        // Magic value
    vendor_id:          u32,        // Virtio Subsystem Vendor ID
    version:            u8,         // Device version number
    device_id:          DeviceID,   // Virtio Subsystem Device ID 
    host_features:      u32,        // Flags representing features the device supports
    host_features_sel:  u32,        // Device (host) features word selection 
    guest_features:     u32,        // Flags representing device features understood and activated by the driver 
    guest_features_sel: u32,        // Activated (guest) features word selection 
    guest_page_size:    u16,        // Guest page size 
    queue_sel:          u32,        // Virtual queue index 
    queue_num_max:      u64,        // Maximum virtual queue size 
    queue_num:          u32,        // Virtual queue size 
    queue_align:        u32,        // Used Ring alignment in the virtual queue 
    queue_pfn:          u32,        // Guest physical page number of the virtual queue
    queue_notify:       u32,        // Queue notifier
    interrupt_status:   u64,        // Interrupt status
    status:             u32,        // Device status 
}

impl Virtio {
    pub fn new(device_id: DeviceID) -> Self {
        Virtio {
            clock:              0,
            disk:               Vec::new(),
            last_avail_idx:     0,
            notify_changed:     false,
            
            magic_value:        0x74726976,     // 0x74726976 (A Little Endian equivalent of the “virt” string)
            vendor_id:          0x554d4551,     // QEMU's Vendor ID
            version:            0x1,            // Legacy devices used 0x1.
            device_id:          device_id,      // Virtio Subsystem Device ID
            host_features:      0,
            host_features_sel:  0,
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
        }
    }

    pub fn load(&mut self, binary: Vec<u8>) {
        if binary.len() > MAX_DISK {
            panic!("[ERROR] too large binary: {}[Byte] (limit: {}[Byte])", binary.len(), MAX_DISK);
        }

        for byte in binary {
            self.disk.push(byte);
        }
    }

    pub fn tick(&mut self, dram: &mut Dram) {
        if self.notify_changed {
            self.disk_access(dram);
            self.notify_changed = false;
        }
        self.clock = self.clock.wrapping_add(1);
    }

    pub fn write8(&mut self, addr: usize, data: u8) {
        match addr {
            HOST_FEATURES_SEL_BASE ..= HOST_FEATURES_SEL_TOP   => {
                let shift = (addr - HOST_FEATURES_SEL_BASE) * 8;
                self.host_features_sel = (self.host_features_sel & !(0xFF << shift)) | ((data as u32) << shift);
            },
            GUEST_FEATURES_BASE ..= GUEST_FEATURES_TOP  => {
                let shift = (addr - GUEST_FEATURES_BASE) * 8;
                self.guest_features = (self.guest_features & !(0xFF << shift)) | ((data as u32) << shift);
            },
            GUEST_FEATURES_SEL_BASE ..= GUEST_FEATURES_SEL_TOP  => {
                let shift = (addr - GUEST_FEATURES_SEL_BASE) * 8;
                self.guest_features_sel = (self.guest_features_sel & !(0xFF << shift)) | ((data as u32) << shift);
            },
            GUEST_PAGE_SIZE_BASE ..= GUEST_PAGE_SIZE_TOP  => {
                let shift = (addr - GUEST_PAGE_SIZE_BASE) * 8;
                self.guest_page_size = (self.guest_page_size & !(0xFF << shift)) | ((data as u16) << shift);
            },
            QUEUE_SEL_BASE ..= QUEUE_SEL_TOP    => {
                let shift = (addr - QUEUE_SEL_BASE) * 8;
                self.queue_sel = (self.queue_sel & !(0xFF << shift)) | ((data as u32) << shift);
            },
            QUEUE_NUM_BASE ..= QUEUE_NUM_TOP   => {
                let shift = (addr - QUEUE_NUM_BASE) * 8;
                self.queue_num = (self.queue_num & !(0xFF << shift)) | ((data as u32) << shift);
            },
            QUEUE_ALIGN_BASE ..= QUEUE_ALIGN_TOP   => {
                let shift = (addr - QUEUE_ALIGN_BASE) * 8;
                self.queue_align = (self.queue_align & !(0xFF << shift)) | ((data as u32) << shift);
            },
            QUEUE_PFN_BASE ..= QUEUE_PFN_TOP => {
                let shift = (addr - QUEUE_PFN_BASE) * 8;
                self.queue_pfn = (self.queue_pfn & !(0xFF << shift)) | ((data as u32) << shift);
            },
            QUEUE_NOTIFY_BASE ..= QUEUE_NOTIFY_TOP  => {
                let shift = (addr - QUEUE_NOTIFY_BASE) * 8;
                self.queue_notify = (self.queue_notify & !(0xFF << shift)) | ((data as u32) << shift);
                self.notify_changed = true;
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
            MAGIC_VALUE_BASE ..= MAGIC_VALUE_TOP    =>{
                let shift = addr * 8;
                ((self.magic_value >> shift) & 0xFF) as u8
            },
            VERSION     => self.version,
            DEVICE_ID   => self.device_id as u8,
            VENDOR_ID_BASE ..= VENDOR_ID_TOP    => {
                let shift = (addr - VENDOR_ID_BASE) * 8;
                ((self.vendor_id >> shift) & 0xFF) as u8
            },
            HOST_FEATURES_BASE ..= HOST_FEATURES_TOP    => {
                let shift = (addr - HOST_FEATURES_BASE) * 8;
                ((self.host_features >> shift) & 0xFF) as u8
            },
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

    fn write8_disk(&mut self, addr: usize, data: u8) {
        self.disk[addr] = data;
    }

    fn read8_disk(&self, addr: usize) -> u8 {
        self.disk[addr]
    }

    fn get_page_addr(&self) -> usize {
        self.queue_pfn as usize * self.guest_page_size as usize - DRAM_BASE
    }
    
    fn get_base_used_addr(&self) -> usize {
        self.get_page_addr() + 4 + (self.queue_num as usize) * 2 + (self.queue_align as usize) - 1
    }

    fn get_virtq_desc(&mut self, mem: &mut Dram, desc_idx: usize) -> virtq_desc {
        let base_addr = self.get_page_addr() + desc_idx * 16;
        // println!("[DEBUG] {}-{}\tbase_addr:\t0x{:x}", file!(), line!(), base_addr);

        virtq_desc {
            addr:   mem.read64(base_addr) as usize,
            len:    mem.read32(base_addr + 8),
            flags:  mem.read16(base_addr + 12),
            next:   mem.read16(base_addr + 14),
        }
    } 

    fn get_virtq_avail(&mut self, mem: &mut Dram) -> virtq_avail {
        let base_addr = self.get_page_addr() + (self.queue_num as usize) * 16;
        // println!("[DEBUG] {}-{}\tbase_addr:\t0x{:x}", file!(), line!(), base_addr);

        virtq_avail {
            flags:      mem.read16(base_addr),
            idx:        mem.read16(base_addr + 2) % self.queue_num as u16,
            ring:       Vec::new(), // ToDo
            used_event: 0,          // ToDo
        }
    }
    
    fn get_virtq_used(&mut self, mem: &mut Dram) -> virtq_used {
        let base_addr = self.get_base_used_addr();
        // println!("[DEBUG] {}-{}\tbase_addr:\t0x{:x}", file!(), line!(), base_addr);

        virtq_used {
            flags:          mem.read16(base_addr),
            idx:            mem.read16(base_addr + 2),
            ring:           Vec::new(),     // ToDo
            avail_event:    0,              // ToDo
        }
    }

    fn set_virtq_used_ring_idx(&self, mem: &mut Dram, idx: u32) {
        mem.write32(self.get_base_used_addr() + 4 + self.last_avail_idx as usize * 8, idx);
    }

    fn get_virtio_blk_req(&mut self, mem: &mut Dram, addr: usize) -> virtio_blk_req {
        // println!("[DEBUG] {}-{}\taddr:\t0x{:x}", file!(), line!(), addr);

        virtio_blk_req {
            r#type:     mem.read32(addr),
            reserved:   mem.read32(addr + 4),
            sector:     mem.read64(addr + 8),
            data:       Vec::new(),     // ToDo
            status:     0,              // ToDo
        }
    }

    fn disk_access(&mut self, mem: &mut Dram) {
        let vq_avail    = self.get_virtq_avail(mem);
        // println!("[DEBUG] {}-{}\tvq_avail:\t{:?}", file!(), line!(), vq_avail);

        let desc_idx_addr =   self.get_page_addr()
                            + self.queue_num as usize * 16
                            + self.last_avail_idx as usize * 2
                            + 4;
        let desc_head_idx = mem.read16(desc_idx_addr) as usize % self.queue_num as usize;
        let mut desc_idx = desc_head_idx;
        let mut desc_num = 0;
        let mut vq_blk_req = virtio_blk_req {
            r#type:     0,
            reserved:   0,
            sector:     0,
            data:       Vec::new(),
            status:     0,
        };
        
        loop {
            let vq_desc = self.get_virtq_desc(mem, desc_idx);
            // println!("[DEBUG] {}-{}\tvq_desc:\t{:?}", file!(), line!(), vq_desc);
            desc_idx = vq_desc.next as usize;
            
            match desc_num {
                0   => {
                    vq_blk_req = self.get_virtio_blk_req(mem, vq_desc.addr - DRAM_BASE);
                    // println!("[DEBUG] {}-{}\tvq_blk_req:\t{:?}", file!(), line!(), vq_blk_req);
                },
                1   => {
                    // write to disk
                    if (vq_desc.flags & VIRTQ_DESC_F_WRITE as u16) == 0 {
                        for i in 0 .. vq_desc.len {
                            let data = mem.read8(vq_desc.addr - DRAM_BASE + i as usize);
                            self.write8_disk(vq_blk_req.sector as usize * SECTOR_SIZE + i as usize, data);
                        }
                    }
                    // read from disk
                    else {
                        for i in 0 .. vq_desc.len {
                            let data = self.read8_disk(vq_blk_req.sector as usize * SECTOR_SIZE + i as usize);
                            mem.write8(vq_desc.addr - DRAM_BASE + i as usize, data);
                        }
                    }
                },
                2   => {
                    if (vq_desc.flags & VIRTQ_DESC_F_WRITE as u16) == 0 {
                        panic!();
                    }
                    if vq_desc.len != 1 {
                        panic!();
                    }
                    mem.write8(vq_desc.addr - DRAM_BASE, 0);
                },
                _   => panic!(),
            }

            desc_num += 1;

            if (vq_desc.flags & VIRTQ_DESC_F_NEXT as u16) == 0 {
                break;
            }
        }

        let vq_used     = self.get_virtq_used(mem);
        self.set_virtq_used_ring_idx(mem, desc_head_idx as u32);
        self.last_avail_idx = (self.last_avail_idx + 1) % self.queue_num as usize;
        mem.write16(self.get_base_used_addr() + vq_used.idx as usize, self.last_avail_idx as u16);
    }
}