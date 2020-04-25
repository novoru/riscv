#[test]
pub fn test_addi() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    cpu.load("./src/test/testcase/addi");

    cpu.fetch();
    cpu.execute();

    assert_eq!(cpu.register[Registers::T0 as usize], 0xFFF);
}

#[test]
pub fn test_add() {
    use super::super::emulator;
    use emulator::cpu::{ Cpu, Registers };
    let mut cpu = Cpu::new();

    cpu.load("./src/test/testcase/add");

    cpu.fetch();
    cpu.execute();

    cpu.fetch();
    cpu.execute();
    
    cpu.fetch();
    cpu.execute();

    assert_eq!(cpu.register[Registers::T2 as usize], 3);

}