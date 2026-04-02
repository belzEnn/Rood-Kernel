// ATA PIO driver

// ATA ports

const PORT_DATA:         u16 = 0x1F0; // Data (16-bit)
const PORT_ERROR:        u16 = 0x1F1; // Errors (unused for now)
const PORT_SECTOR_COUNT: u16 = 0x1F2; // Sector count
const PORT_LBA_LOW:      u16 = 0x1F3; // LBA bits 0-7
const PORT_LBA_MID:      u16 = 0x1F4; // LBA bits 8-15
const PORT_LBA_HIGH:     u16 = 0x1F5; // LBA bits 16-23
const PORT_DRIVE:        u16 = 0x1F6; // Drive select
const PORT_STATUS:       u16 = 0x1F7; // Status (read)
const PORT_COMMAND:      u16 = 0x1F7; // Command (write)

// commands

const CMD_READ:     u8 = 0x20; // Read sectors
const CMD_WRITE:    u8 = 0x30; // Write sectors
const CMD_IDENTIFY: u8 = 0xEC; // Identify disk
const CMD_FLUSH:    u8 = 0xE7; // Flush cache

// bits status

const STATUS_ERR: u8 = 0x01; // Error
const STATUS_DRQ: u8 = 0x08; // Data ready
const STATUS_BSY: u8 = 0x80; // Busy

// Sector size

pub const SECTOR_SIZE: usize = 512;

// Active drive (0xA0 = Master, 0xB0 = Slave)
static mut ACTIVE_DRIVE: u8 = 0xA0;

pub unsafe fn set_active_drive(drive: u8) {
    ACTIVE_DRIVE = drive;
}

pub unsafe fn get_active_drive() -> u8 {
    ACTIVE_DRIVE
}

// Port helpers

unsafe fn inb(port: u16) -> u8 {
    let v: u8;
    core::arch::asm!("in al, dx", out("al") v, in("dx") port, options(nomem, nostack));
    v
}

unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack));
}

unsafe fn inw(port: u16) -> u16 {
    let v: u16;
    core::arch::asm!("in ax, dx", out("ax") v, in("dx") port, options(nomem, nostack));
    v
}

unsafe fn outw(port: u16, val: u16) {
    core::arch::asm!("out dx, ax", in("dx") port, in("ax") val, options(nomem, nostack));
}

// Waiting

// Wait until disk is not busy
unsafe fn wait_not_busy() {
    while inb(PORT_STATUS) & STATUS_BSY != 0 {}
}

// Wait for data ready
unsafe fn wait_drq() -> Result<(), &'static str> {
    loop {
        let status = inb(PORT_STATUS);
        if status & STATUS_ERR != 0 { return Err("ATA: disk error"); }
        if status & STATUS_DRQ != 0 { return Ok(()); }
    }
}

// LBA setup

// Select Primary Master and set LBA
unsafe fn select_lba(lba: u32) {
    outb(PORT_DRIVE, ACTIVE_DRIVE | ((lba >> 24) as u8 & 0x0F));
    outb(PORT_SECTOR_COUNT, 1);
    outb(PORT_LBA_LOW,      (lba & 0xFF) as u8);
    outb(PORT_LBA_MID,      ((lba >> 8)  & 0xFF) as u8);
    outb(PORT_LBA_HIGH,     ((lba >> 16) & 0xFF) as u8);
}

// Public API

// Detect ATA disk
pub unsafe fn detect() -> bool {
    // Try Primary Master
    if detect_drive(0xA0) { return true; }
    // Try Primary Slave  
    if detect_drive(0xB0) { return true; }
    false
}

pub unsafe fn detect_drive(drive_select: u8) -> bool {
    outb(PORT_DRIVE, drive_select);
    for _ in 0..15 { inb(PORT_STATUS); }
    outb(PORT_COMMAND, CMD_IDENTIFY);

    let s1 = inb(PORT_STATUS);
    if s1 == 0 || s1 == 0xFF { return false; }

    for _ in 0..100_000u32 {
        if inb(PORT_STATUS) & STATUS_BSY == 0 { break; }
    }

    let mid  = inb(PORT_LBA_MID);
    let high = inb(PORT_LBA_HIGH);

    if mid == 0x14 && high == 0xEB {
        // if is CD-ROM, skip
        return false;
    }

    // mid=04 — Some QEMU disks give this

    for _ in 0..100_000u32 {
        let s = inb(PORT_STATUS);
        if s & STATUS_ERR != 0 { return false; }
        if s & STATUS_DRQ != 0 { break; }
    }

    for _ in 0..256 { inw(PORT_DATA); }
    true
}

/// Read one sector (512 bytes)
pub unsafe fn read_sector(lba: u32, buf: &mut [u8; SECTOR_SIZE]) -> Result<(), &'static str> {
    wait_not_busy();
    select_lba(lba);
    outb(PORT_COMMAND, CMD_READ);
    wait_drq()?;

    // Read 256 words = 512 bytes
    for i in 0..256 {
        let word = inw(PORT_DATA);
        buf[i * 2]     = (word & 0xFF) as u8;
        buf[i * 2 + 1] = (word >> 8)   as u8;
    }
    Ok(())
}

pub unsafe fn read_status() -> u8 {
    inb(PORT_STATUS)
}

// Write one sector (512 bytes)
pub unsafe fn write_sector(lba: u32, buf: &[u8; SECTOR_SIZE]) -> Result<(), &'static str> {
    wait_not_busy();
    select_lba(lba);
    outb(PORT_COMMAND, CMD_WRITE);
    wait_drq()?;

    // Write 256 words = 512 bytes
    for i in 0..256 {
        let word = buf[i * 2] as u16 | ((buf[i * 2 + 1] as u16) << 8);
        outw(PORT_DATA, word);
    }

    // Flush disk cache
    outb(PORT_COMMAND, CMD_FLUSH);
    wait_not_busy();
    Ok(())
}

/// Print byte as hex string
pub fn byte_to_hex(byte: u8) -> [u8; 2] {
    let hex = b"0123456789ABCDEF";
    [hex[(byte >> 4) as usize], hex[(byte & 0xF) as usize]]
}
