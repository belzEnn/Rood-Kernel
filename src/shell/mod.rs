// Shell

mod builtin;
mod system;
mod display;
mod files;

use crate::framebuffer::{self, RED, WHITE};

pub fn execute(input: &str) {
    let input = input.trim();
    if input.is_empty() { return; }

    let mut parts = input.splitn(9, ' ');
    let cmd = parts.next().unwrap_or("");
    let argv: alloc::vec::Vec<&str> = parts.filter(|s| !s.is_empty()).collect();

    match cmd {
        "help"     => builtin::help(),
        "clear"    => builtin::clear(),
        "echo"     => builtin::echo(&argv),
        "uname"    => system::uname(),
        "reboot"   => system::reboot(),
        "shutdown" => system::shutdown(),
        "diskinfo" => system::diskinfo(&argv),
        "color"    => display::color(&argv),
        "ls"       => files::ls(),
        "pwd"      => files::pwd(),
        "cd"       => files::cd(&argv),
        "mkdir"    => files::mkdir(&argv),
        "touch"    => files::touch(&argv),
        "rm"       => files::rm(&argv),
        "cat"      => files::cat(&argv),
        "write"    => files::write(&argv),
        _ => {
            framebuffer::print_str(b"Unknown command: ", RED);
            framebuffer::print_str(cmd.as_bytes(), RED);
            framebuffer::print_byte(b'\n', WHITE);
        }
    }
}
