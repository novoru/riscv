use crate::emulator::register::Register;

pub struct Cpu {
    pub register: Register,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { register: Register::new() }
    }

    pub fn fetch(& self) {
        unimplemented!();
    }

    pub fn decode(& self) {
        unimplemented!();
    }

    pub fn execute(& self) {
        unimplemented!();
    }

}