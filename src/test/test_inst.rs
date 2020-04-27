#[test]
pub fn test_addi() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/addi") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }

    assert_eq!(cpu.register[Registers::T0 as usize], 0xFFFF_FFFF_FFFF_FFFF);
}

#[test]
pub fn test_add() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/add") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }

    assert_eq!(cpu.register[Registers::T2 as usize], 3);

}

#[test]
pub fn test_slti() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/slti") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 1);

}

#[test]
pub fn test_sltiu() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/sltiu") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0);

}

#[test]
pub fn test_andi() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/andi") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0b100_0100_0100);

}

#[test]
pub fn test_ori() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/ori") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0b111_0100_0111);

}

#[test]
pub fn test_xori() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/xori") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0b111_0000_0011);

}

#[test]
pub fn test_slli() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/slli") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0b101_1100);

}

#[test]
pub fn test_srli() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/srli") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0x3FFF_FFFF_FFFF_FFFF);

}

#[test]
pub fn test_srai() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/srai") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 0xFFFF_FFFF_FFFF_FFFF);

}

#[test]
pub fn test_lui() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/lui") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 0xFFFF_F000);

}

#[test]
pub fn test_auipc() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/auipc") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T0 as usize], 0xFFFF_F000);

}

#[test]
pub fn test_slt() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/slt") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 1);

}

#[test]
pub fn test_sltu() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/sltu") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0);

}

#[test]
pub fn test_and() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/and") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0b100_0100_0100);

}

#[test]
pub fn test_or() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/or") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0b111_0100_0111);

}

#[test]
pub fn test_xor() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/xor") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0b111_0000_0011);

}

#[test]
pub fn test_sll() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/sll") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0b101_1100);

}

#[test]
pub fn test_srl() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/srl") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0x3FFF_FFFF_FFFF_FFFF);

}

#[test]
pub fn test_sub() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/sub") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }

    assert_eq!(cpu.register[Registers::T2 as usize], 3);

}

#[test]
pub fn test_sra() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/sra") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 0xFFFF_FFFF_FFFF_FFFF);

}

#[test]
pub fn test_nop() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/nop") / 4;

    assert_eq!(cpu.register[Registers::ZERO as usize], 0);

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::ZERO as usize], 0);

}

#[test]
pub fn test_beq() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/beq") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
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

    let len = cpu.load("./src/test/testcase/bne") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
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

    let len = cpu.load("./src/test/testcase/jal") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
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

    let len = cpu.load("./src/test/testcase/jalr") / 4;

    for _ in 0 .. len {
        cpu.fetch();
        cpu.execute();
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

    let len = cpu.load("./src/test/testcase/blt") / 4;

    for _ in 0 .. len {
        eprintln!("[INFO] pc: {}", cpu.pc);
        cpu.fetch();
        cpu.execute();
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

    let len = cpu.load("./src/test/testcase/bltu") / 4;

    for _ in 0 .. len {
        eprintln!("[INFO] pc: 0x{:08x}", cpu.pc);
        cpu.fetch();
        cpu.execute();
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

    let len = cpu.load("./src/test/testcase/bge") / 4;

    for _ in 0 .. len {
        eprintln!("[INFO] pc: 0x{:08x}", cpu.pc);
        cpu.fetch();
        cpu.execute();
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

    let len = cpu.load("./src/test/testcase/bgeu") / 4;

    for _ in 0 .. len {
        eprintln!("[INFO] pc: 0x{:08x}", cpu.pc);
        cpu.fetch();
        cpu.execute();
        if cpu.pc >= (len * 4) {
            break;
        }
    }
    
    assert_eq!(cpu.register[Registers::T2 as usize], 32);

}