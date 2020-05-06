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

            // If GP equals 1, test is passed.
            cpu.watch(Registers::GP, 1);

            cpu.run();

            Ok(())
        }
    };
}

// RV64 user-level, integer only, virtual memory is disabled, only core 0 boots up
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
add_test!(rv64ui_p_lui);
add_test!(rv64ui_p_or);
add_test!(rv64ui_p_ori);
add_test!(rv64ui_p_sll);
add_test!(rv64ui_p_sllw);
add_test!(rv64ui_p_slli);
add_test!(rv64ui_p_slliw);
add_test!(rv64ui_p_srl);
add_test!(rv64ui_p_srlw);
add_test!(rv64ui_p_srli);
add_test!(rv64ui_p_srliw);
add_test!(rv64ui_p_sra);
add_test!(rv64ui_p_sraw);
add_test!(rv64ui_p_srai);
add_test!(rv64ui_p_sraiw);
add_test!(rv64ui_p_addw);
add_test!(rv64ui_p_sub);
add_test!(rv64ui_p_subw);
add_test!(rv64ui_p_xor);
add_test!(rv64ui_p_xori);
add_test!(rv64ui_p_lb);
add_test!(rv64ui_p_sb);
add_test!(rv64ui_p_lh);
add_test!(rv64ui_p_sh);
add_test!(rv64ui_p_lw);
add_test!(rv64ui_p_sw);
add_test!(rv64ui_p_ld);
add_test!(rv64ui_p_sd);
add_test!(rv64ui_p_lbu);
add_test!(rv64ui_p_lhu);
add_test!(rv64ui_p_lwu);
add_test!(rv64ui_p_simple);

//RV64 user-level, integer multiplication and division
add_test!(rv64um_p_mul);
add_test!(rv64um_p_mulh);
add_test!(rv64um_p_mulhsu);
add_test!(rv64um_p_mulhu);