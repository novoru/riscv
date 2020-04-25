#[test]
pub fn test_addi() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/addi") / 4;

    for _i in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }

    assert_eq!(cpu.register[Registers::T0 as usize], 0xFFF);
}

#[test]
pub fn test_add() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    let len = cpu.load("./src/test/testcase/add") / 4;

    for _i in 0 .. len {
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

    for _i in 0 .. len {
        cpu.fetch();
        cpu.execute();
    }
    
    assert_eq!(cpu.register[Registers::T1 as usize], 1);

}