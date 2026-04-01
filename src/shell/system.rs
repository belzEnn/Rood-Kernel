// uname, reboot, shutdown, diskinfo

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

// Информация о диске / Disk info
pub unsafe fn diskinfo() {
    use crate::drivers::disk::ata;
    use crate::framebuffer::{self, WHITE, GREEN, RED, YELLOW};

    // Проверяем Primary Master / Check Primary Master
    if ata::detect_drive(0xA0) {
        framebuffer::print_str(b"Disk:   ATA Primary Master\n", GREEN);
        framebuffer::print_str(b"Status: OK\n", GREEN);
    // Проверяем Primary Slave / Check Primary Slave
    } else if ata::detect_drive(0xB0) {
        framebuffer::print_str(b"Disk:   ATA Primary Slave\n", GREEN);
        framebuffer::print_str(b"Status: OK\n", GREEN);
    } else {
        framebuffer::print_str(b"No disk detected\n", RED);
        return;
    }

    // Читаем первый сектор чтобы проверить / Read first sector to verify
    let mut buf = [0u8; ata::SECTOR_SIZE];
    match ata::read_sector(0, &mut buf) {
        Ok(_) => {
            framebuffer::print_str(b"Sector: 512 bytes\n", WHITE);
            // Проверяем boot signature / Check boot signature
            if buf[510] == 0x55 && buf[511] == 0xAA {
                framebuffer::print_str(b"Boot:   bootable\n", YELLOW);
            } else {
                framebuffer::print_str(b"Boot:   not bootable\n", WHITE);
            }
        }
        Err(e) => {
            framebuffer::print_str(b"Read:   error - ", RED);
            framebuffer::print_str(e.as_bytes(), RED);
            framebuffer::print_byte(b'\n', WHITE);
        }
    }
}