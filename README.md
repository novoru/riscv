# RISC-V emulator
This is an experimental RISC-V emulator written in Rust.

## 💻 Useage
```
cargo run -- [filename]
```
## 🐞 Debug
To display debug information, launch emulator with -d:
```
cargo run -- -d [filename]
```
With the -s option, the program can be executed in steps.
```
cargo run -- -d -s [filename]
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
- [riscv/riscv-isa-sim ](https://github.com/riscv/riscv-isa-sim)
- [d0iasm/rvemu](https://github.com/d0iasm/rvemu)

Tests
- [riscv/riscv-tests](https://github.com/riscv/riscv-tests)