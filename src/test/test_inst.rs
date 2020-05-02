#[test]
pub fn test_addi() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/addi".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }

    assert_eq!(cpu.register[Registers::T0 as usize], 0xFFFF_FFFF_FFFF_FFFF);
}

#[test]
pub fn test_add() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/add".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }

    assert_eq!(cpu.register[Registers::T2 as usize], 3);

}

#[test]
pub fn test_slti() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/slti".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 1);

}

#[test]
pub fn test_sltiu() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/sltiu".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0);

}

#[test]
pub fn test_andi() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/andi".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0b100_0100_0100);

}

#[test]
pub fn test_ori() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/ori".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0b111_0100_0111);

}

#[test]
pub fn test_xori() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/xori".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0b111_0000_0011);

}

#[test]
pub fn test_slli() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/slli".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0b101_1100);

}

#[test]
pub fn test_srli() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/srli".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0x3FFF_FFFF_FFFF_FFFF);

}

#[test]
pub fn test_srai() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/srai".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0xFFFF_FFFF_FFFF_FFFF);

}

#[test]
pub fn test_lui() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/lui".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 0xFFFF_F000);

}

#[test]
pub fn test_auipc() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/auipc".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 0xFFFF_F000);

}

#[test]
pub fn test_slt() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/slt".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 1);

}

#[test]
pub fn test_sltu() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/sltu".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0);

}

#[test]
pub fn test_and() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/and".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0b100_0100_0100);

}

#[test]
pub fn test_or() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/or".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0b111_0100_0111);

}

#[test]
pub fn test_xor() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/xor".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0b111_0000_0011);

}

#[test]
pub fn test_sll() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/sll".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0b101_1100);

}

#[test]
pub fn test_srl() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/srl".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0x3FFF_FFFF_FFFF_FFFF);

}

#[test]
pub fn test_sub() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/sub".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }

    assert_eq!(cpu.register[Registers::T2 as usize], 3);

}

#[test]
pub fn test_sra() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/sra".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0xFFFF_FFFF_FFFF_FFFF);

}

#[test]
pub fn test_nop() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/nop".to_string()) / 4;

    assert_eq!(cpu.register[Registers::ZERO as usize], 0);

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
    }
    
    assert_eq!(cpu.register[Registers::ZERO as usize], 0);

}

#[test]
pub fn test_beq() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/beq".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 2);

}

#[test]
pub fn test_bne() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/bne".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 2);

}

#[test]
pub fn test_jal() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/jal".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 11);
    assert_eq!(cpu.register[Registers::RA as usize], 8);

}

#[test]
pub fn test_jalr() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/jalr".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 11);
    assert_eq!(cpu.register[Registers::T1 as usize], 12);

}

#[test]
pub fn test_blt() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/blt".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 32);

}

#[test]
pub fn test_bltu() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/bltu".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 24);

}

#[test]
pub fn test_bge() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/bge".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 24);

}

#[test]
pub fn test_bgeu() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/bgeu".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 32);

}

#[test]
pub fn test_lsb() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/lsb".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }

    let addr = cpu.register[Registers::A0 as usize] as usize;
    assert_eq!(cpu.mmu.read64(cpu.csr, addr), 0x0000_0000_0000_00DD);
    assert_eq!(cpu.register[Registers::T1 as usize], 0xFFFF_FFFF_FFFF_FFDD);

}

#[test]
pub fn test_lsh() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/lsh".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }

    let addr = cpu.register[Registers::A0 as usize] as usize;
    assert_eq!(cpu.mmu.read64(cpu.csr, addr), 0x0000_0000_0000_FFFF);
    assert_eq!(cpu.register[Registers::T1 as usize], 0xFFFF_FFFF_FFFF_FFFF);

}

#[test]
pub fn test_lsw() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load(&"./src/test/testcase/lsw".to_string()) / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
		cpu.pc += 4;
        if cpu.pc >= (len * 4) {
            break;
        }
    }

    let addr = cpu.register[Registers::A0 as usize] as usize;
    assert_eq!(cpu.mmu.read64(cpu.csr, addr), 0x0000_0000_FFFF_F000);
    assert_eq!(cpu.register[Registers::T1 as usize], 0xFFFF_FFFF_FFFF_F000);

}