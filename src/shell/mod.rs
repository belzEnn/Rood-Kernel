// Shell module

mod builtin;
mod system;
mod display;
mod files;

use crate::framebuffer::{self, WHITE, RED};

pub const MAX_ARGS: usize = 8;

pub fn parse(input: &[u8]) -> (&[u8], [&[u8]; MAX_ARGS], usize) {
    let mut parts: [&[u8]; MAX_ARGS] = [b""; MAX_ARGS];
    let mut count = 0;
    let mut start = 0;
    let mut in_word = false;

    for i in 0..input.len() {
        let is_space = input[i] == b' ';
        if !is_space && !in_word {
            start = i;
            in_word = true;
        } else if is_space && in_word {
            if count < MAX_ARGS {
                parts[count] = &input[start..i];
                count += 1;
            }
            in_word = false;
        }
    }
    if in_word && count < MAX_ARGS {
        parts[count] = &input[start..];
        count += 1;
    }

    let cmd = if count > 0 { parts[0] } else { b"" };
    (cmd, parts, count)
}

pub unsafe fn execute(input: &[u8]) {
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
        b"reboot"   => system::reboot(),
        b"shutdown" => system::shutdown(),
        // Display
        b"color"    => display::color(args, argc),
        // Filesystem
        b"ls"       => files::ls(),
        b"cat"      => files::cat(args, argc),
        b"mkdir"    => files::mkdir(args, argc),
        b"touch"    => files::touch(args, argc),
        b"rm"       => files::rm(args, argc),
        b"cd"       => files::cd(args, argc),
        b"pwd"      => files::pwd(),
        b"write"    => files::write(args, argc),
        // Unknown
        _ => {
            framebuffer::print_str(b"Unknown command: ", RED);
            framebuffer::print_str(cmd, RED);
            framebuffer::print_byte(b'\n', WHITE);
            framebuffer::print_str(b"Type 'help' for available commands\n", WHITE);
        }
    }
}