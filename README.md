# RISC-V emulator
This is an experimental RISC-V emulator written in Rust.

## 💻 Useage
```
cargo run -- [filename]
```
## 🧪 Test
```
make test
```
## 🛠 Features
- [ ] RV32/RV64G
    - [x] RV32I (without fence/ebreak)
    - [ ] RV64I
    - [x] RV32/RV64 *Zicsr*
- [x] CSRs
- [ ] Virtual Memory

## 📚 References
Documents
- [RISC-V: Specifications](https://riscv.org/specifications/)

Emulators
- [d0iasmrvemu](https://github.com/d0iasm/rvemu)
- [riscv/riscvOVPsim](https://github.com/riscv/riscv-ovpsim)