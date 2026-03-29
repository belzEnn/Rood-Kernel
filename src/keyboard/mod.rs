mod scancodes;
use scancodes::scancode_to_char;
use crate::framebuffer::{self, WHITE, GREEN};

// ─── Порты PS/2 ────────────────────────────────────────────────────────────

const KBD_DATA:   u16 = 0x60;
const KBD_STATUS: u16 = 0x64;

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

// ─── Состояние ─────────────────────────────────────────────────────────────

static mut SHIFT: bool = false;
static mut INPUT_BUF: [u8; 256] = [0u8; 256];
static mut INPUT_LEN: usize = 0;

// ─── Публичный API ─────────────────────────────────────────────────────────

/// Попытаться прочитать скан-код из порта PS/2.
/// Возвращает Some(scancode) если данные готовы, иначе None.
pub unsafe fn try_read_scancode() -> Option<u8> {
    if inb(KBD_STATUS) & 1 != 0 { Some(inb(KBD_DATA)) } else { None }
}

/// Обработать один скан-код: обновить состояние и вывести символ на экран.
pub unsafe fn handle_scancode(sc: u8) {
    match sc {
        // Нажатие Shift
        0x2A | 0x36 => { SHIFT = true; }
        // Отпускание Shift
        0xAA | 0xB6 => { SHIFT = false; }
        // Backspace
        0x0E => {
            if INPUT_LEN > 0 {
                INPUT_LEN -= 1;
                framebuffer::backspace();
            }
        }
        // Enter
        0x1C => {
            framebuffer::print_byte(b'\n', WHITE);
            INPUT_LEN = 0;
            framebuffer::print_str(b"> ", GREEN);
        }
        // Печатный символ
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
