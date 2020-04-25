#[test]
pub fn test_addi() {
    use super::super::emulator;
    use emulator::cpu::Cpu;
    let mut cpu = Cpu::new();

    // x1 = 0xFFF;
    let imm = 0xFFF;
    let rs1 = 1;
    let funct3 = 0b0000000;     // ADDI
    let rd  = 1;
    let opcode = 0b0010011;     // I-type

    let addi_inst: u64 = ((imm & 0xFFF) << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode;
    cpu.mmu.write64(0, addi_inst);

    cpu.fetch();
    cpu.execute();

    assert_eq!(cpu.register[1], 0xFFF);
}

#[test]
pub fn test_add() {
    use super::super::emulator;
    use emulator::cpu::Cpu;
    let mut cpu = Cpu::new();

    // x1 = 0xAAA;
    let imm_1 = 0xAAA;
    let rs1_1 = 1;
    let funct3 = 0b0000000;     // ADDI
    let rd_1  = 1;
    let opcode_i = 0b0010011;     // I-type

    let addi_inst1: u64 = ((imm_1 & 0xFFF) << 20) | (rs1_1 << 15) | (funct3 << 12) | (rd_1 << 7) | opcode_i;
    cpu.mmu.write64(0, addi_inst1);

    println!("inst1");

    cpu.fetch();
    cpu.execute();

    // x2 = 0xDDD;
    let imm_2 = 0xDDD;
    let rs1_2 = 2;
    let rd_2  = 2;

    let addi_inst2: u64 = ((imm_2 & 0xFFF) << 20) | (rs1_2 << 15) | (funct3 << 12) | (rd_2 << 7) | opcode_i;
    cpu.mmu.write64(4, addi_inst2);

    println!("inst2");
    
    cpu.fetch();
    cpu.execute();

    // x1 = x1 + x2;
    let funct7 = 0b0000000;
    let rs2 = 2;
    let rs1_3 = 1;
    let funct3 = 0b000;         // ADD
    let rd_3 = 1;
    let opcode_r = 0b0110011;    // R-type

    let addi_inst3: u64 = ((funct7 & 0x7F) << 25) | (rs2 << 20) | (rs1_3 << 15) | (funct3 << 12) | (rd_3 << 7) |opcode_r;
    cpu.mmu.write64(8, addi_inst3);

    println!("inst3");
    
    cpu.fetch();
    cpu.execute();

    assert_eq!(cpu.register[1], 0x1887);

}