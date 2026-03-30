# Rood
[Rust](https://rust-lang.org/tools/install/)

#### Compiling the kernel:
```bash
CARGO_UNSTABLE_BUILD_STD="core,compiler_builtins" cargo build -Zbuild-std=core,compiler_builtins -Zbuild-std-features=compiler-builtins-mem
```
#### Running via QEMU
UEFI
```bash rust
cd runner/
cargo run -- ../target/x86_64-unknown-none/debug/kernel uefi
```
Legacy BIOS:
```bash
cd runner/

cargo run -- ../target/x86_64-unknown-none/debug/kernel bios

```