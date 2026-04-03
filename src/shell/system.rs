// System commands

use crate::framebuffer::{self, WHITE, GREEN, RED, YELLOW};

const OS_NAME:    &str = "Rood";
const OS_VERSION: &str = "0.1.0";
const OS_ARCH:    &str = "x86_64";

pub fn uname() {
    framebuffer::print_str(OS_NAME.as_bytes(),    GREEN);
    framebuffer::print_byte(b' ',                 WHITE);
    framebuffer::print_str(OS_VERSION.as_bytes(), WHITE);
    framebuffer::print_byte(b' ',                 WHITE);
    framebuffer::print_str(OS_ARCH.as_bytes(),    WHITE);
    framebuffer::print_byte(b'\n',                WHITE);
}

pub fn reboot() {
    framebuffer::print_str(b"Rebooting...\n", RED);
    unsafe {
        // Wait PS/2 buffer
        loop {
            let s: u8;
            core::arch::asm!("in al, dx", out("al") s, in("dx") 0x64u16, options(nomem, nostack));
            if s & 0x02 == 0 { break; }
        }
        core::arch::asm!("out dx, al", in("dx") 0x64u16, in("al") 0xFEu8, options(nomem, nostack));
        loop { core::arch::asm!("hlt"); }
    }
}

pub fn shutdown() {
    framebuffer::print_str(b"Shutting down...\n", RED);
    unsafe {
        core::arch::asm!("out dx, ax", in("dx") 0x604u16, in("ax") 0x2000u16, options(nomem, nostack));
        loop { core::arch::asm!("hlt"); }
    }
}

pub fn diskinfo(args: &[&str]) {
    use crate::drivers::disk::ata;

    let verbose = args.first() == Some(&"-v");

    if verbose {
        let status = crate::drivers::port::PortRead::<u8>::new(0x1F7).read();
        framebuffer::print_str(b"Raw status: ", WHITE);
        let hex = ata::byte_to_hex(status);
        framebuffer::print_byte(hex[0], YELLOW);
        framebuffer::print_byte(hex[1], YELLOW);
        framebuffer::print_byte(b'\n', WHITE);
        framebuffer::print_str(b"Master: ", WHITE);
        framebuffer::print_byte(if ata::detect_drive(0xA0) { b'Y' } else { b'N' }, WHITE);
        framebuffer::print_byte(b'\n', WHITE);
        framebuffer::print_str(b"Slave:  ", WHITE);
        framebuffer::print_byte(if ata::detect_drive(0xB0) { b'Y' } else { b'N' }, WHITE);
        framebuffer::print_byte(b'\n', WHITE);
        return;
    }

    if ata::detect_drive(0xB0) {
        framebuffer::print_str(b"Disk:   ATA Primary Slave\n",  GREEN);
        framebuffer::print_str(b"Status: OK\n",                  GREEN);
    } else if ata::detect_drive(0xA0) {
        framebuffer::print_str(b"Disk:   ATA Primary Master\n", GREEN);
        framebuffer::print_str(b"Status: OK\n",                  GREEN);
    } else {
        framebuffer::print_str(b"No disk detected\n", RED);
        return;
    }
    framebuffer::print_str(b"FS:     ROOD format\n",   YELLOW);
    framebuffer::print_str(b"Sector: 512 bytes\n",     WHITE);
}
