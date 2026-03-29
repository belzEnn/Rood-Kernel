# Rood
[Rust](https://rust-lang.org/tools/install/)

#### Compiling the kernel:
```bash
CARGO_UNSTABLE_BUILD_STD="core,compiler_builtins" cargo build -Zbuild-std=core,compiler_builtins -Zbuild-std-features=compiler-builtins-mem
```
#### Running via QEMU
```bash
cd runner/
cargo run -- ../Rood/target/x86_64-unknown-none/debug/kernel
```