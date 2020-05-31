#![feature(test)]

pub mod emulator;

extern crate test;
use test::Bencher;
use emulator::cpu::{ Cpu, Registers };

#[bench]
fn bench_addi(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.register.write(Registers::SP as usize, 0x8100_0000);
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0xfd010113).unwrap();  // 0x8000_0000: addi	sp,sp,-48
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

#[bench]
fn bench_add(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.register.write(Registers::SP as usize, 0x8100_0000);
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0x009504b3).unwrap();  // 0x8000_0000: add	s1,a0,s1
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

#[bench]
fn bench_slli(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.register.write(Registers::SP as usize, 0x8100_0000);
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0x01b79793).unwrap();  // 0x8000_0000: slli	a5,a5,0x1b
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

#[bench]
fn bench_lui(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.register.write(Registers::SP as usize, 0x8100_0000);
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0x000017b7).unwrap();  // 0x8000_0000: lui	a5,0x1
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

#[bench]
fn bench_auipc(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.register.write(Registers::SP as usize, 0x8100_0000);
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0xfffff097).unwrap();  // 0x8000_0000: auipc	ra,0xfffff
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

#[bench]
fn bench_bne(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.register.write(Registers::A5 as usize, 0x1);
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0xfee79ce3).unwrap();  // 0x8000_0000: bne	a5,a4,80000f78
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

#[bench]
fn bench_bnez(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.register.write(Registers::A5 as usize, 0x1);
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0xfe079ae3).unwrap();  // 0x8000_0000: bnez	a5,80000eac
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

#[bench]
fn bench_beq(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0x04060463).unwrap();  // 0x8000_0000: beqz	a2,80000fe4
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

#[bench]
fn bench_lbu(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.register.write(Registers::A0 as usize, 0x8000_0000);
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0x00054783).unwrap();  // 0x8000_0000: lbu	a5,0(a0)
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

#[bench]
fn bench_ld(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.register.write(Registers::SP as usize, 0x8100_0000);
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0x00013403).unwrap();  // 0x8000_0000: ld s0, 0(sp)
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

#[bench]
fn bench_sb(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.register.write(Registers::A5 as usize, 0x8000_0000);
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0x00b78023).unwrap();  // 0x8000_0000: sb	a1,0(a5)
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

#[bench]
fn bench_sd(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.register.write(Registers::SP as usize, 0x8100_0000);
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0x01413023).unwrap();  // 0x8000_0000: sd s4, 0(sp)
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

#[bench]
fn bench_amoswap(b: &mut Bencher) {
    let mut cpu = Cpu::new();
    cpu.register.write(Registers::S1 as usize, 0x8100_0000);
    cpu.mmu.write32(&cpu.csr, 0x8000_0000, 0x0cf4a7af).unwrap();  // 0x8000_0000: amoswap.w.aq	a5,a5,(s1)
    cpu.fetch().unwrap();

    b.iter(||  {
        cpu.execute().unwrap();
    });
}

/*
#[bench]
fn bench_dram_read8(b: &mut Bencher) {
    let cpu = Cpu::new();

    b.iter(||  {
        let paddr = test::black_box(0x0000_0000);
        cpu.mmu.bus.dram.read8(paddr);
    });
}

#[bench]
fn bench_dram_read64(b: &mut Bencher) {
    let cpu = Cpu::new();

    b.iter(||  {
        let paddr = test::black_box(0x0000_0000);
        cpu.mmu.bus.dram.read64(paddr);
    });
}

#[bench]
fn bench_bus_read8(b: &mut Bencher) {
    let mut cpu = Cpu::new();

    b.iter(||  {
        let paddr = test::black_box(0x8000_0000);
        cpu.mmu.bus.read8(paddr);
    });
}

#[bench]
fn bench_bus_read64(b: &mut Bencher) {
    let mut cpu = Cpu::new();

    b.iter(||  {
        let paddr = test::black_box(0x8000_0000);
        cpu.mmu.bus.read64(paddr);
    });
}
*/

#[bench]
fn bench_mmu_read8(b: &mut Bencher) {
    let mut cpu = Cpu::new();

    b.iter(||  {
        let vaddr = test::black_box(0x8000_0000);
        cpu.mmu.read8(&cpu.csr, vaddr).unwrap();
    });
}

#[bench]
fn bench_mmu_read64(b: &mut Bencher) {
    let mut cpu = Cpu::new();

    b.iter(||  {
        let vaddr = test::black_box(0x8000_0000);
        cpu.mmu.read64(&cpu.csr, vaddr).unwrap();
    });
}

/*
#[bench]
fn bench_dram_write8(b: &mut Bencher) {
    let mut cpu = Cpu::new();

    b.iter(||  {
        let paddr = test::black_box(0x0000_0000);
        cpu.mmu.bus.dram.write8(paddr, 0xFF);
    });
}

#[bench]
fn bench_dram_write64(b: &mut Bencher) {
    let mut cpu = Cpu::new();

    b.iter(||  {
        let paddr = test::black_box(0x0000_0000);
        cpu.mmu.bus.dram.write64(paddr, 0xFFFF_FFFF_FFFF_FFFF);
    });
}

#[bench]
fn bench_bus_write8(b: &mut Bencher) {
    let mut cpu = Cpu::new();

    b.iter(||  {
        let paddr = test::black_box(0x8000_0000);
        cpu.mmu.bus.write8(paddr, 0xFF);
    });
}

#[bench]
fn bench_bus_write64(b: &mut Bencher) {
    let mut cpu = Cpu::new();

    b.iter(||  {
        let paddr = test::black_box(0x8000_0000);
        cpu.mmu.bus.write64(paddr, 0xFFFF_FFFF_FFFF_FFFF);
    });
}
*/

#[bench]
fn bench_mmu_write8(b: &mut Bencher) {
    let mut cpu = Cpu::new();

    b.iter(||  {
        let vaddr = test::black_box(0x8000_0000);
        cpu.mmu.write8(&cpu.csr, vaddr, 0xFF).unwrap();
    });
}

#[bench]
fn bench_mmu_write64(b: &mut Bencher) {
    let mut cpu = Cpu::new();

    b.iter(||  {
        let vaddr = test::black_box(0x8000_0000);
        cpu.mmu.write64(&cpu.csr, vaddr, 0xFFFF_FFFF_FFFF_FFFF).unwrap();
    });
}

#[bench]
fn bench_reg_read(b: &mut Bencher) {
    let cpu = Cpu::new();

    b.iter(||  {
        cpu.register.read(Registers::RA as usize);
    });
}

#[bench]
fn bench_reg_write(b: &mut Bencher) {
    let mut cpu = Cpu::new();

    b.iter(||  {
        cpu.register.write(Registers::RA as usize, 0);
    });
}
