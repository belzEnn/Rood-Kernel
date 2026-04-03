// color

use crate::framebuffer::{self, WHITE, RED};

fn parse_u8(s: &str) -> Option<u8> {
    let n: u16 = s.chars().try_fold(0u16, |acc, c| {
        let d = c.to_digit(10)? as u16;
        Some(acc * 10 + d)
    })?;
    if n <= 255 { Some(n as u8) } else { None }
}

pub fn color(args: &[&str]) {
    if args.len() < 3 {
        framebuffer::print_str(b"Usage: color <r> <g> <b> [text]\n", WHITE);
        return;
    }
    match (parse_u8(args[0]), parse_u8(args[1]), parse_u8(args[2])) {
        (Some(r), Some(g), Some(b)) => {
            let color = (r, g, b);
            if args.len() > 3 {
                let text = args[3..].join(" ");
                framebuffer::print_str(text.as_bytes(), color);
            } else {
                framebuffer::print_str(b"Color sample", color);
            }
            framebuffer::print_byte(b'\n', WHITE);
        }
        _ => { framebuffer::print_str(b"Invalid values (0-255)\n", RED); }
    }
}
