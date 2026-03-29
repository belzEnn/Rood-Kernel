use bootloader::BiosBoot;
use std::path::PathBuf;

fn main() {
    let kernel_path = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("Usage: rs-runner <path-to-kernel-binary>"),
    );

    if !kernel_path.exists() {
        eprintln!("Kernel binary not found: {}", kernel_path.display());
        eprintln!("Did you run `cargo build` in RS-kernel first?");
        std::process::exit(1);
    }

    let out_path = PathBuf::from("disk.img");

    println!("Creating disk image from {}...", kernel_path.display());
    BiosBoot::new(&kernel_path)
        .create_disk_image(&out_path)
        .expect("Failed to create disk image");
    println!("Disk image created: {}", out_path.display());

    println!("Launching QEMU...");
    let status = std::process::Command::new("qemu-system-x86_64")
        .args([
            "-drive", &format!("format=raw,file={}", out_path.display()),
            "-m", "64M",
            "-no-reboot",
            "-no-shutdown",
        ])
        .status()
        .expect("Failed to launch QEMU. Is it installed?");

    std::process::exit(status.code().unwrap_or(1));
}
