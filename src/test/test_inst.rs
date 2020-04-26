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
    
    assert_eq!(cpu.register[Registers::T0 as usize], 0xFFFF_F004);

}