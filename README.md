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
    - [x] RV32A/RV64A (without lr/sc)
    - [x] RV32/RV64 *Zicsr*
- [x] CSRs
- [x] Virtual Memory (Sv39 only)
- [ ] CLINT
- [ ] PLIC
- [ ] UART
- [ ] VIRTIO

## 📚 References
Documents
- RISC-V
    - [RISC-V: Specifications](https://riscv.org/specifications/)
    - [RISC-V Platform-Level Interrupt Controller Specification](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc)
    - [SiFive S51 Manual v19.08p2p0](https://sifive.cdn.prismic.io/sifive/8daf40b4-a369-4490-ab5b-509ee68f6bc2_sifive_coreip_S51_AXI4_rtl_v19_08p2p0_release_manual.pdf)
    - [An Introduction to the RISC-V Architecture](https://sifive.cdn.prismic.io/sifive/25f3cf28-47ae-4cea-9e64-ecd43dea7f11_An+Introduction+to+the+RISC-V+Architecture.pdf)
    - [RISC-V原典 オープンアーキテクチャのススメ](https://www.nikkeibp.co.jp/atclpubmkt/book/18/269170/)
- UART
    - [TECHNICAL DATA ON 16550](http://byterunner.com/16550.html)
- VIRTIO
    - [Virtual I/O Device (VIRTIO) Version 1.1 Committee Specification 01](https://docs.oasis-open.org/virtio/virtio/v1.1/cs01/virtio-v1.1-cs01.html)
    - [ハイパーバイザの作り方～ちゃんと理解する仮想化技術～ 第１２回virtioによる準仮想化デバイス その２「Virtqueueとvirtio-netの実現」](https://syuu1228.github.io/howto_implement_hypervisor/part12.pdf)
- Others
    - [Freedom Metal](https://sifive.github.io/freedom-metal-docs/)
    - [The Adventures of OS: Making a RISC-V Operating System using Rust](http://osblog.stephenmarz.com/index.html)


Emulators
- [riscv/riscv-isa-sim ](https://github.com/riscv/riscv-isa-sim)
- [d0iasm/rvemu](https://github.com/d0iasm/rvemu)
- [msyksphinz-self/swimmer_rust](https://github.com/msyksphinz-self/swimmer_rust)
- [takahirox/riscv-rust](https://github.com/takahirox/riscv-rust)

Tests
- [riscv/riscv-tests](https://github.com/riscv/riscv-tests)