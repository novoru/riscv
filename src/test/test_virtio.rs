
#[test]
pub fn test_virtio_magic_value() {
    use super::super::emulator;
    use crate::emulator::bus::VIRTIO_BASE;
    use emulator::cpu::Cpu;

    let mut cpu = Cpu::new();

    assert_eq!(cpu.mmu.read32(&cpu.csr, VIRTIO_BASE).unwrap(), 0x74726976);
}

#[test]
pub fn test_virtio_version() {
    use super::super::emulator;
    use crate::emulator::bus::VIRTIO_BASE;
    use emulator::cpu::Cpu;

    let mut cpu = Cpu::new();

    assert_eq!(cpu.mmu.read8(&cpu.csr, VIRTIO_BASE + 0x004).unwrap(), 0x1);
}

#[test]
pub fn test_virtio_device_id() {
    use super::super::emulator;
    use crate::emulator::bus::VIRTIO_BASE;
    use emulator::cpu::Cpu;

    let mut cpu = Cpu::new();

    assert_eq!(cpu.mmu.read8(&cpu.csr, VIRTIO_BASE + 0x008).unwrap(), 0x2);
}

#[test]
pub fn test_virtio_vendor_id() {
    use super::super::emulator;
    use crate::emulator::bus::VIRTIO_BASE;
    use emulator::cpu::Cpu;

    let mut cpu = Cpu::new();

    assert_eq!(cpu.mmu.read32(&cpu.csr, VIRTIO_BASE + 0x00C).unwrap(), 0x554d4551);
}
