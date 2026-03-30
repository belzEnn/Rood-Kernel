// uname, reboot, shutdown

use crate::framebuffer::{self, WHITE, GREEN, RED};

// OS version
const OS_NAME:    &[u8] = b"Rood";
const OS_VERSION: &[u8] = b"0.1.0";
const OS_ARCH:    &[u8] = b"x86_64";

// System info
pub unsafe fn uname() {
    framebuffer::print_str(OS_NAME,    GREEN);
    framebuffer::print_byte(b' ',      WHITE);
    framebuffer::print_str(OS_VERSION, WHITE);
    framebuffer::print_byte(b' ',      WHITE);
    framebuffer::print_str(OS_ARCH,    WHITE);
    framebuffer::print_byte(b'\n',     WHITE);
}

// Reboot via PS/2 controller
pub unsafe fn reboot() {
    framebuffer::print_str(b"Rebooting...\n", RED);

    // Wait for PS/2 buffer to be empty
    loop {
        let status: u8;
        core::arch::asm!(
            "in al, dx",
            out("al") status,
            in("dx") 0x64u16,
            options(nomem, nostack)
        );
        if status & 0x02 == 0 { break; }
    }

    // Send reset command
    core::arch::asm!(
        "out dx, al",
        in("dx") 0x64u16,
        in("al") 0xFEu8,
        options(nomem, nostack)
    );

    loop { core::arch::asm!("hlt"); }
}

// Shutdown via QEMU port / Выключение через порт QEMU
pub unsafe fn shutdown() {
    framebuffer::print_str(b"Shutting down...\n", RED);
    core::arch::asm!(
        "out dx, ax",
        in("dx") 0x604u16,
        in("ax") 0x2000u16,
        options(nomem, nostack)
    );
    loop { core::arch::asm!("hlt"); }
}