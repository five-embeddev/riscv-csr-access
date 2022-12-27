# RISC-V CSR Access Routines for Rust, Macro Based

The `src/riscv_csr_macros.rs` file is generated from `templates/riscv_csr_macros.rs`.

This implements a macro for each register and each access mode. It is based on the C code.

## Building Example

The `examples/test_csr.rs` file 

Setup Rust and RISC-V target:
~~~
curl https://sh.rustup.rs -sSf | sh
 
rustup target add riscv32imac-unknown-none-elf
~~~

Build the example:

~~~
cargo build --examples --release
~~~


