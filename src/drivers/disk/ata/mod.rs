use crate::drivers::port::{Port, PortRead, PortWrite};

// ATA ports

struct AtaPorts {
    data:         Port<u16>,      // 0x1F0 data
    sector_count: Port<u8>,       // 0x1F2 sector count
    lba_low:      Port<u8>,       // 0x1F3 LBA 0-7
    lba_mid:      Port<u8>,       // 0x1F4 LBA 8-15
    lba_high:     Port<u8>,       // 0x1F5 LBA 16-23
    drive:        Port<u8>,       // 0x1F6 drive select
    status:       PortRead<u8>,   // 0x1F7 status (read)
    command:      PortWrite<u8>,  // 0x1F7 command (write)
}

impl AtaPorts {
    const fn new() -> Self {
        Self {
            data:         Port::new(0x1F0),
            sector_count: Port::new(0x1F2),
            lba_low:      Port::new(0x1F3),
            lba_mid:      Port::new(0x1F4),
            lba_high:     Port::new(0x1F5),
            drive:        Port::new(0x1F6),
            status:       PortRead::new(0x1F7),
            command:      PortWrite::new(0x1F7),
        }
    }
}

// Global ports
static PORTS: AtaPorts = AtaPorts::new();

// Constants

const CMD_READ:     u8 = 0x20;
const CMD_WRITE:    u8 = 0x30;
const CMD_FLUSH:    u8 = 0xE7;
const CMD_IDENTIFY: u8 = 0xEC;

const STATUS_ERR: u8 = 0x01;
const STATUS_DRQ: u8 = 0x08;
const STATUS_BSY: u8 = 0x80;

pub const SECTOR_SIZE: usize = 512;

// Active drive
static mut ACTIVE_DRIVE: u8 = 0xA0;

pub fn set_active_drive(drive: u8) {
    unsafe { ACTIVE_DRIVE = drive; }
}

pub fn get_active_drive() -> u8 {
    unsafe { ACTIVE_DRIVE }
}

// Helpers

fn wait_not_busy() {
    while PORTS.status.read() & STATUS_BSY != 0 {}
}

fn wait_drq() -> Result<(), &'static str> {
    for _ in 0..100_000u32 {
        let s = PORTS.status.read();
        if s & STATUS_ERR != 0 { return Err("ATA: disk error"); }
        if s & STATUS_DRQ != 0 { return Ok(()); }
    }
    Err("ATA: timeout")
}

fn select_lba(lba: u32) {
    let drive = unsafe { ACTIVE_DRIVE };
    PORTS.drive.write(drive | ((lba >> 24) as u8 & 0x0F));
    PORTS.sector_count.write(1);
    PORTS.lba_low.write((lba & 0xFF) as u8);
    PORTS.lba_mid.write(((lba >> 8)  & 0xFF) as u8);
    PORTS.lba_high.write(((lba >> 16) & 0xFF) as u8);
}

// Public API

pub fn detect_drive(drive_select: u8) -> bool {
    PORTS.drive.write(drive_select);

    // 400ns delay — read status 4 times
    for _ in 0..4 { PORTS.status.read(); }

    // Reset LBA
    PORTS.lba_low.write(0);
    PORTS.lba_mid.write(0);
    PORTS.lba_high.write(0);
    PORTS.sector_count.write(0);

    PORTS.command.write(CMD_IDENTIFY);

    // If status 0: no disk
    let s = PORTS.status.read();
    if s == 0 || s == 0xFF { return false; }

    // Wait BSY
    for _ in 0..100_000u32 {
        if PORTS.status.read() & STATUS_BSY == 0 { break; }
    }

    // Check ATAPI
    let mid  = PORTS.lba_mid.read();
    let high = PORTS.lba_high.read();
    if mid == 0x14 && high == 0xEB { return false; } // ATAPI
    if mid == 0x69 && high == 0x96 { return false; } // ATAPI

    // Wait DRQ or ERR
    for _ in 0..100_000u32 {
        let s = PORTS.status.read();
        if s & STATUS_ERR != 0 { return false; }
        if s & STATUS_DRQ != 0 { break; }
    }

    for _ in 0..256 { PORTS.data.read(); }
    true
}

// Helpers for IDENTIFY
fn inb_lba_mid()  -> u8 { PORTS.lba_mid.read()  }
fn inb_lba_high() -> u8 { PORTS.lba_high.read() }

pub fn read_sector(lba: u32, buf: &mut [u8; SECTOR_SIZE]) -> Result<(), &'static str> {
    wait_not_busy();
    select_lba(lba);
    PORTS.command.write(CMD_READ);
    wait_drq()?;

    for i in 0..256 {
        let word = PORTS.data.read();
        buf[i * 2]     = (word & 0xFF) as u8;
        buf[i * 2 + 1] = (word >> 8)   as u8;
    }
    Ok(())
}

pub fn write_sector(lba: u32, buf: &[u8; SECTOR_SIZE]) -> Result<(), &'static str> {
    wait_not_busy();
    select_lba(lba);
    PORTS.command.write(CMD_WRITE);
    for _ in 0..4 { PORTS.status.read(); }
    wait_drq()?;

    for i in 0..256 {
        let word = buf[i * 2] as u16 | ((buf[i * 2 + 1] as u16) << 8);
        PORTS.data.write(word);
        PORTS.status.read();
    }

    PORTS.command.write(CMD_FLUSH);
    wait_not_busy();

    if PORTS.status.read() & STATUS_ERR != 0 {
        return Err("ATA: write error");
    }
    Ok(())
}

pub fn byte_to_hex(byte: u8) -> [u8; 2] {
    let hex = b"0123456789ABCDEF";
    [hex[(byte >> 4) as usize], hex[(byte & 0xF) as usize]]
}

pub fn detect() -> bool {
    detect_drive(0xA0) || detect_drive(0xB0)
}
