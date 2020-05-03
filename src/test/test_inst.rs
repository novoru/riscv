use crate::emulator::exception::Exception;

#[test]
pub fn test_addi() -> Result<(), Exception> {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/addi".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }

    assert_eq!(cpu.register[Registers::T0 as usize], 0xFFFF_FFFF_FFFF_FFFF);
	Ok(())
}

#[test]
pub fn test_add() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/add".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }

    assert_eq!(cpu.register[Registers::T2 as usize], 3);
	Ok(())
}

#[test]
pub fn test_slti() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/slti".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 1);
	Ok(())
}

#[test]
pub fn test_sltiu() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/sltiu".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0);
	Ok(())
}

#[test]
pub fn test_andi() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/andi".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0b100_0100_0100);
	Ok(())
}

#[test]
pub fn test_ori() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/ori".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0b111_0100_0111);
	Ok(())
}

#[test]
pub fn test_xori() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/xori".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0b111_0000_0011);
	Ok(())
}

#[test]
pub fn test_slli() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/slli".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0b101_1100);
	Ok(())
}

#[test]
pub fn test_srli() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/srli".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0x3FFF_FFFF_FFFF_FFFF);
	Ok(())
}

#[test]
pub fn test_srai() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/srai".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0xFFFF_FFFF_FFFF_FFFF);
	Ok(())
}

#[test]
pub fn test_lui() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/lui".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 0xFFFF_F000);
	Ok(())
}

#[test]
pub fn test_auipc() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/auipc".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 0xFFFF_F000);
	Ok(())
}

#[test]
pub fn test_slt() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/slt".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 1);
	Ok(())
}

#[test]
pub fn test_sltu() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/sltu".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0);
	Ok(())
}

#[test]
pub fn test_and() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/and".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0b100_0100_0100);
	Ok(())
}

#[test]
pub fn test_or() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/or".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0b111_0100_0111);
	Ok(())
}

#[test]
pub fn test_xor() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/xor".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0b111_0000_0011);
	Ok(())
}

#[test]
pub fn test_sll() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/sll".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0b101_1100);
	Ok(())
}

#[test]
pub fn test_srl() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/srl".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0x3FFF_FFFF_FFFF_FFFF);
	Ok(())
}

#[test]
pub fn test_sub() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/sub".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }

    assert_eq!(cpu.register[Registers::T2 as usize], 3);
	Ok(())
}

#[test]
pub fn test_sra() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/sra".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0xFFFF_FFFF_FFFF_FFFF);
	Ok(())
}

#[test]
pub fn test_nop() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/nop".to_string()) / 4;

    assert_eq!(cpu.register[Registers::ZERO as usize], 0);

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::ZERO as usize], 0);
	Ok(())
}

#[test]
pub fn test_beq() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/beq".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 2);
	Ok(())
}

#[test]
pub fn test_bne() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/bne".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 2);
	Ok(())
}

#[test]
pub fn test_jal() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/jal".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 11);
    assert_eq!(cpu.register[Registers::RA as usize], 8);
	Ok(())
}

#[test]
pub fn test_jalr() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/jalr".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 11);
    assert_eq!(cpu.register[Registers::T1 as usize], 12);
	Ok(())
}

#[test]
pub fn test_blt() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/blt".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 32);
	Ok(())
}

#[test]
pub fn test_bltu() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/bltu".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 24);
	Ok(())
}

#[test]
pub fn test_bge() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/bge".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 24);
	Ok(())
}

#[test]
pub fn test_bgeu() -> Result<(), Exception>  {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/bgeu".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 32);
	Ok(())
}

#[test]
pub fn test_lsb() -> Result<(), Exception> {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/lsb".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }

    let addr = cpu.register[Registers::A0 as usize] as usize;
    assert_eq!(cpu.mmu.read64(cpu.csr, addr)?, 0x0000_0000_0000_00DD);
    assert_eq!(cpu.register[Registers::T1 as usize], 0xFFFF_FFFF_FFFF_FFDD);

    Ok(())
}

#[test]
pub fn test_lsh() -> Result<(), Exception> {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/lsh".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }

    let addr = cpu.register[Registers::A0 as usize] as usize;
    assert_eq!(cpu.mmu.read64(cpu.csr, addr)?, 0x0000_0000_0000_FFFF);
    assert_eq!(cpu.register[Registers::T1 as usize], 0xFFFF_FFFF_FFFF_FFFF);

    Ok(())
}

#[test]
pub fn test_lsw() -> Result<(), Exception> {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/lsw".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch()?;
        cpu.execute()?;
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }

    let addr = cpu.register[Registers::A0 as usize] as usize;
    assert_eq!(cpu.mmu.read64(cpu.csr, addr)?, 0x0000_0000_FFFF_F000);
    assert_eq!(cpu.register[Registers::T1 as usize], 0xFFFF_FFFF_FFFF_F000);

    Ok(())
}