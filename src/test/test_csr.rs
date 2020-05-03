use crate::emulator::exception::Exception;

#[test]
pub fn test_write_bit() -> Result<(), Exception> {
    use super::super::emulator;
    use emulator::cpu::Cpu;
    use emulator::csr::*;

    let mut cpu = Cpu::new();

    cpu.csr.write_bit(USTATUS, 2, true);
    assert_eq!(cpu.csr.read(USTATUS), 0b100);

    cpu.csr.write_bit(USTATUS, 15, true);
    assert_eq!(cpu.csr.read(USTATUS), 0b1000_0000_0000_0100);
    
    cpu.csr.write_bit(USTATUS, 2, false);
    assert_eq!(cpu.csr.read(USTATUS), 0b1000_0000_0000_0000);

    Ok(())
}

#[test]
pub fn test_read_bit() -> Result<(), Exception> {
    use super::super::emulator;
    use emulator::cpu::Cpu;
    use emulator::csr::*;

    let mut cpu = Cpu::new();

    cpu.csr.write_bit(USTATUS, 2, true);
    assert_eq!(cpu.csr.read_bit(USTATUS, 2), true);

    cpu.csr.write_bit(USTATUS, 15, true);
    assert_eq!(cpu.csr.read_bit(USTATUS,15), true);
    
    cpu.csr.write_bit(USTATUS, 2, false);
    assert_eq!(cpu.csr.read_bit(USTATUS, 2), false);

    Ok(())
}

#[test]
pub fn test_write_bits() -> Result<(), Exception> {
    use super::super::emulator;
    use emulator::cpu::Cpu;
    use emulator::csr::*;

    let mut cpu = Cpu::new();

    cpu.csr.write_bits(USTATUS, 0..10, 0b0_1011_1101);
    assert_eq!(cpu.csr.read(USTATUS), 0b0_1011_1101);

    Ok(())
}

#[test]
pub fn test_read_bits() -> Result<(), Exception> {
    use super::super::emulator;
    use emulator::cpu::Cpu;
    use emulator::csr::*;

    let mut cpu = Cpu::new();

    cpu.csr.write_bits(USTATUS, 0..9, 0b1_1011_1101);
    assert_eq!(cpu.csr.read_bits(USTATUS, 0..9), 0b1_1011_1101);

    cpu.csr.write_bits(USTATUS, 10..22, 0xF0F);
    assert_eq!(cpu.csr.read_bits(USTATUS, 10..22), 0xF0F);

    Ok(())
}