mod test;
pub mod emulator;

use std::env;
use emulator::cpu::{ Cpu };

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("useage: ./riscv [filename]");
        std::process::exit(0);
    }

    let mut cpu = Cpu::new();
    cpu.load(&args[1]);

    cpu.run();
}
