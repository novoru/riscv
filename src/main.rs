mod test;
pub mod emulator;

use structopt::StructOpt;
use emulator::cpu::{ Cpu, Registers, WatchExec };

#[derive(Debug, StructOpt)]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,
    
    /// Step execution mode
    #[structopt(short, long)]
    pub step: bool,

    /// Input file
    #[structopt(short, long)]
    pub file: String,

}

fn main() {

    let opt = Opt::from_args();

    let mut cpu = Cpu::new();
    cpu.debug = opt.debug;
    cpu.step = opt.step;
    cpu.load(&opt.file);
    //cpu.watch(Registers::PC, 0x800012bc, WatchExec::STOP);

    cpu.run();
}
