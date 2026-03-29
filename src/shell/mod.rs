// Модуль оболочки / Shell module

mod builtin;
mod system;
mod display;

use crate::framebuffer::{self, WHITE, RED};

// Max number of arguments
pub const MAX_ARGS: usize = 8;

// Split input into command and arguments
pub fn parse(input: &[u8]) -> (&[u8], [&[u8]; MAX_ARGS], usize) {
    let mut parts: [&[u8]; MAX_ARGS] = [b""; MAX_ARGS];
    let mut count = 0;
    let mut start = 0;
    let mut in_word = false;

    for i in 0..input.len() {
        let is_space = input[i] == b' ';
        if !is_space && !in_word {
            // Word start
            start = i;
            in_word = true;
        } else if is_space && in_word {
            // Word end
            if count < MAX_ARGS {
                parts[count] = &input[start..i];
                count += 1;
            }
            in_word = false;
        }
    }
    // Last word
    if in_word && count < MAX_ARGS {
        parts[count] = &input[start..];
        count += 1;
    }

    let cmd = if count > 0 { parts[0] } else { b"" };
    (cmd, parts, count)
}

// Execute a command
pub unsafe fn execute(input: &[u8]) {
    // Skip empty input
    if input.is_empty() || input.iter().all(|&b| b == b' ') {
        return;
    }

    let (cmd, args, argc) = parse(input);

    match cmd {
        // Builtin
        b"help"     => builtin::help(),
        b"clear"    => builtin::clear(),
        b"echo"     => builtin::echo(args, argc),

        // System
        b"uname"    => system::uname(),

        // Display
        b"color"    => display::color(args, argc),

        // Unknown command
        _ => {
            framebuffer::print_str(b"Unknown command: ", RED);
            framebuffer::print_str(cmd, RED);
            framebuffer::print_byte(b'\n', WHITE);
            framebuffer::print_str(b"Type 'help' for available commands\n", WHITE);
        }
    }
}
