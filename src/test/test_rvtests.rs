use crate::emulator::cpu::*;
use std::io;


#[macro_export]
macro_rules! add_test {
($str: ident) => {
        #[test]
        fn $str() -> io::Result<()> {
            let mut filename = "./src/test/rvtests/".to_string();
            filename.push_str(stringify!($str));
            println!("{}", filename);

            let mut cpu = Cpu::new();
            cpu.load(&filename);

            cpu.debug = true;
            cpu.watch(Registers::GP, 1);

            cpu.run();

            Ok(())
        }
    };
}

add_test!(rv64ui_p_add);
add_test!(rv64ui_p_addi);
add_test!(rv64ui_p_addiw);
add_test!(rv64ui_p_and);
add_test!(rv64ui_p_andi);
add_test!(rv64ui_p_auipc);
add_test!(rv64ui_p_beq);
add_test!(rv64ui_p_bge);
add_test!(rv64ui_p_bgeu);
add_test!(rv64ui_p_blt);
add_test!(rv64ui_p_bltu);
add_test!(rv64ui_p_bne);
add_test!(rv64ui_p_jal);
add_test!(rv64ui_p_jalr);
add_test!(rv64ui_p_sub);
//add_test!(rv64ui_p_fence_i);