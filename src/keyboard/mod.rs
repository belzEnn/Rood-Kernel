mod scancodes;
use scancodes::scancode_to_char;
use crate::framebuffer::{self, WHITE, GREEN};
use crate::shell;

// ps/2 ports

const KBD_DATA:   u16 = 0x60;
const KBD_STATUS: u16 = 0x64;

// Read a byte from a port
unsafe fn inb(port: u16) -> u8 {
    let v: u8;
    core::arch::asm!(
        "in al, dx",
        out("al") v,
        in("dx") port,
        options(nomem, nostack)
    );
    v
}

// State

static mut SHIFT:     bool       = false;
static mut INPUT_BUF: [u8; 256]  = [0u8; 256];
static mut INPUT_LEN: usize      = 0;

// API

// Try to read a scancode from PS/2 port
pub unsafe fn try_read_scancode() -> Option<u8> {
    if inb(KBD_STATUS) & 1 != 0 { Some(inb(KBD_DATA)) } else { None }
}

// Handle one scancode
pub unsafe fn handle_scancode(sc: u8) {
    match sc {
        // shift pressed
        0x2A | 0x36 => { SHIFT = true; }
        // shift released
        0xAA | 0xB6 => { SHIFT = false; }
        // Backspace
        0x0E => {
            if INPUT_LEN > 0 {
                INPUT_LEN -= 1;
                framebuffer::backspace();
            }
        }
        // Enter — execute command
        0x1C => {
            framebuffer::print_byte(b'\n', WHITE);
            // Pass input buffer to shell
            shell::execute(&INPUT_BUF[..INPUT_LEN]);
            INPUT_LEN = 0;
            // Print new prompt
            framebuffer::print_str(b"> ", WHITE);
        }
        // Printable character
        _ => {
            if let Some(c) = scancode_to_char(sc, SHIFT) {
                if INPUT_LEN < INPUT_BUF.len() - 1 {
                    INPUT_BUF[INPUT_LEN] = c;
                    INPUT_LEN += 1;
                    framebuffer::print_byte(c, WHITE);
                }
            }
        }
    }
}
