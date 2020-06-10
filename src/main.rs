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

    /// Kernel
    #[structopt(short, long)]
    pub kernel: String,

    /// disk image
    #[structopt(short, long)]
    pub disk: String,
}

fn main() {

    let opt = Opt::from_args();

    let mut cpu = Cpu::new();
    cpu.debug = opt.debug;
    cpu.step = opt.step;
    cpu.load_dram(&opt.kernel);
    cpu.load_disk(&opt.disk);
    //cpu.watch(Registers::PC, 0x800029cc, WatchExec::STOP);

    cpu.run();
}
