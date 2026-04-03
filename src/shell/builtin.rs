// help, clear, echo

use crate::framebuffer::{self, WHITE, YELLOW};

pub fn help() {
    framebuffer::print_str(b"=== Builtin ===\n", YELLOW);
    framebuffer::print_str(b"  help               - this message\n", WHITE);
    framebuffer::print_str(b"  clear              - clear screen\n", WHITE);
    framebuffer::print_str(b"  echo <text>        - print text\n", WHITE);
    framebuffer::print_str(b"=== System ===\n", YELLOW);
    framebuffer::print_str(b"  uname              - system info\n", WHITE);
    framebuffer::print_str(b"  reboot             - reboot\n", WHITE);
    framebuffer::print_str(b"  shutdown           - power off\n", WHITE);
    framebuffer::print_str(b"  diskinfo [-v]      - disk info\n", WHITE);
    framebuffer::print_str(b"=== Display ===\n", YELLOW);
    framebuffer::print_str(b"  color <r> <g> <b> [text]\n", WHITE);
    framebuffer::print_str(b"=== Filesystem ===\n", YELLOW);
    framebuffer::print_str(b"  ls  pwd  cd  mkdir  touch  rm  cat  write\n", WHITE);
}

pub fn clear() {
    crate::framebuffer::clear();
}

pub fn echo(args: &[&str]) {
    let text = args.join(" ");
    framebuffer::print_str(text.as_bytes(), WHITE);
    framebuffer::print_byte(b'\n', WHITE);
}
