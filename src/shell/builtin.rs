// help, clear, echo

use crate::framebuffer::{self, WHITE, GREEN, YELLOW};
use super::MAX_ARGS;

// List all commands
pub unsafe fn help() {
    framebuffer::print_str(b"Available command:\n", GREEN);
    framebuffer::print_str(b"  help - show this message\n", WHITE);
    framebuffer::print_str(b"  clear - clear the screen\n", WHITE);
    framebuffer::print_str(b"  echo <text> - print text\n", WHITE);
    framebuffer::print_str(b"  uname - system info\n", WHITE);
    framebuffer::print_str(b"  color <r> <g> <b> - set text color (0-255)\n", WHITE);
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
