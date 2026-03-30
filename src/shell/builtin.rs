// help, clear, echo

use crate::framebuffer::{self, WHITE, GREEN, YELLOW};
use super::MAX_ARGS;

// List all commands
pub unsafe fn help() {
    framebuffer::print_str(b"=== Builtin ===\n", YELLOW);
    framebuffer::print_str(b"  help               - show this message\n", WHITE);
    framebuffer::print_str(b"  clear              - clear the screen\n", WHITE);
    framebuffer::print_str(b"  echo <text>        - print text\n", WHITE);
    framebuffer::print_str(b"=== System ===\n", YELLOW);
    framebuffer::print_str(b"  uname              - system info\n", WHITE);
    framebuffer::print_str(b"  reboot             - reboot\n", WHITE);
    framebuffer::print_str(b"  shutdown           - power off\n", WHITE);
    framebuffer::print_str(b"=== Filesystem ===\n", YELLOW);
    framebuffer::print_str(b"  ls                - list files\n", WHITE);
    framebuffer::print_str(b"  pwd               - current directory\n", WHITE);
    framebuffer::print_str(b"  cd <dir>          - change directory\n", WHITE);
    framebuffer::print_str(b"  mkdir <dir>       - create directory\n", WHITE);
    framebuffer::print_str(b"  touch <file>      - create empty file\n", WHITE);
    framebuffer::print_str(b"  rm <file>         - remove file\n", WHITE);
    framebuffer::print_str(b"  cat <file>        - print file contents\n", WHITE);
    framebuffer::print_str(b"  write <f> <text>  - write text to file\n", WHITE);
    framebuffer::print_str(b"=== Display ===\n", YELLOW);
    framebuffer::print_str(b"  color <r> <g> <b> [text] - colored text\n", WHITE);
}

// Clear the screen
pub unsafe fn clear() {
    framebuffer::clear();
}

// Print arguments
pub unsafe fn echo(args: [&[u8]; MAX_ARGS], argc: usize) {
    // Skip first arg
    for i in 1..argc {
        framebuffer::print_str(args[i], WHITE);
        if i < argc - 1 {
            framebuffer::print_byte(b' ', WHITE);
        }
    }
    framebuffer::print_byte(b'\n', WHITE);
}
