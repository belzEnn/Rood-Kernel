use bootloader::{BiosBoot, UefiBoot};
use std::path::PathBuf;

fn main() {
    let kernel_path = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("Usage: rs-runner <path-to-kernel-binary> [bios|uefi]"),
    );

    // Boot mode (defualt bios)
    let mode = std::env::args().nth(2).unwrap_or("bios".to_string());

    if !kernel_path.exists() {
        eprintln!("Kernel binary not found: {}", kernel_path.display());
        eprintln!("Did you run `cargo build` in RS-kernel first?");
        std::process::exit(1);
    }

    let out_path = PathBuf::from("disk.img");

    println!("Boot mode: {}", mode);
    println!("Creating disk image from {}...", kernel_path.display());

    match mode.as_str() {
        "uefi" => {
            UefiBoot::new(&kernel_path)
                .create_disk_image(&out_path)
                .expect("Failed to create UEFI disk image");
        }
        _ => {
            BiosBoot::new(&kernel_path)
                .create_disk_image(&out_path)
                .expect("Failed to create BIOS disk image");
        }
    }

    println!("Disk image created: {}", out_path.display());
    println!("Launching QEMU...");

    let mut qemu_args = vec![
        "-drive".to_string(),
        format!("format=raw,file={}", out_path.display()),
        "-drive".to_string(),
        "format=raw,file=hdd.img,if=ide,media=disk".to_string(),
        "-m".to_string(), "128M".to_string(),
        "-no-reboot".to_string(),
        "-no-shutdown".to_string(),
    ];

    // UEFI requires OVMF firmware
    if mode == "uefi" {
        qemu_args.extend([
        "-bios".to_string(),
        "/usr/share/edk2/x64/OVMF.4m.fd".to_string(),
        ]);
    }

    let status = std::process::Command::new("qemu-system-x86_64")
        .args(&qemu_args)
        .status()
        .expect("Failed to launch QEMU. Is it installed?");

    std::process::exit(status.code().unwrap_or(1));
}