pub mod emulator;

use emulator::cpu::{ Cpu };

fn main() {
    let cpu = Cpu::new();

    loop {
        cpu.fetch();
        cpu.decode();
        cpu.execute();
    }
}
