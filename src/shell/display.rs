// color

use crate::framebuffer::{self, WHITE, RED};
use super::MAX_ARGS;

// Parse number from byte string
fn parse_u8(s: &[u8]) -> Option<u8> {
    let mut result: u16 = 0;
    if s.is_empty() { return None; }
    for &b in s {
        if b < b'0' || b > b'9' { return None; }
        result = result * 10 + (b - b'0') as u16;
        if result > 255 { return None; }
    }
    Some(result as u8)
}

// Print text in given color
// Usage: color <r> <g> <b> <text...>
pub unsafe fn color(args: [&[u8]; MAX_ARGS], argc: usize) {
    if argc < 4 {
        framebuffer::print_str(b"Usage: color <r> <g> <b> [text]\n", WHITE);
        framebuffer::print_str(b"Example: color 255 100 0 Hello!\n", WHITE);
        return;
    }

    let r = parse_u8(args[1]);
    let g = parse_u8(args[2]);
    let b = parse_u8(args[3]);

    match (r, g, b) {
        (Some(r), Some(g), Some(b)) => {
            // Print remaining args with given color
            if argc > 4 {
                for i in 4..argc {
                    framebuffer::print_str(args[i], (r, g, b));
                    if i < argc - 1 {
                        framebuffer::print_byte(b' ', (r, g, b));
                    }
                }
            } else {
                // No text — show sample
                framebuffer::print_str(b"Color sample", (r, g, b));
            }
            framebuffer::print_byte(b'\n', WHITE);
        }
        _ => {
            framebuffer::print_str(b"Invalid values. Use 0-255\n", RED);
        }
    }
}
