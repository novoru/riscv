mod test;
pub mod emulator;

use emulator::cpu::{ Cpu };

fn main() {
    let mut cpu = Cpu::new();

    loop {
        cpu.fetch();
        cpu.execute();
    }
}
