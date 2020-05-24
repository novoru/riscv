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

## 💾 Memory layout

Physical Memory (based on qemu's hw/riscv/virt.c:)

Base|Top|Description
---|---|---
0x0000_1000|0x0000_10FF|boot ROM
0x0000_1100|0x01FF_FFFF|Reserved
0x0200_0000|0x0200_FFFF|CLINT
0x0201_0000|0x0BFF_FFFF|Reserved
0x0c00_0000|0x0FFF_FFFF|PLIC
0x1000_0000|0x1000_00FF|UART0
0x1000_1000|0x1000_1FFF|VIRTIO
0x1000_2000|0x7FFF_FFFF|Reserved
0x8000_0000|0x87FF_FFFF|DRAM (128MiB)
0x8800_0000|0xFFFF_FFFF|Reserved


## 🧪 Test
```
make test
```
## 🛠 Features
- [x] RV32/RV64G
    - [x] RV32I/RV64I (without fence)
    - [x] RV32M/RV64M
    - [x] RV32A/RV64A
    - [x] RV32/RV64 *Zicsr*
- [x] CSRs
- [x] Virtual Memory (Sv39 only)
- [ ] CLINT
- [ ] PLIC
- [ ] UART
- [ ] VIRTIO

## 📚 References
Documents
- [RISC-V: Specifications](https://riscv.org/specifications/)

Emulators
- [riscv/riscv-isa-sim ](https://github.com/riscv/riscv-isa-sim)
- [d0iasm/rvemu](https://github.com/d0iasm/rvemu)
- [msyksphinz-self/swimmer_rust](https://github.com/msyksphinz-self/swimmer_rust)
- [takahirox/riscv-rust](https://github.com/takahirox/riscv-rust)

Tests
- [riscv/riscv-tests](https://github.com/riscv/riscv-tests)