// ls, cat, mkdir, touch, rm, cd, pwd, write

use crate::framebuffer::{self, WHITE, GREEN, RED, YELLOW};
use crate::fs;

pub fn ls() {
    let entries = unsafe { fs::list() };
    if entries.is_empty() {
        framebuffer::print_str(b"(empty)\n", WHITE);
        return;
    }
    for (name, is_dir) in entries {
        if is_dir {
            framebuffer::print_str(name.as_bytes(), YELLOW);
            framebuffer::print_str(b"/\n", YELLOW);
        } else {
            framebuffer::print_str(name.as_bytes(), WHITE);
            framebuffer::print_byte(b'\n', WHITE);
        }
    }
}

pub fn pwd() {
    let name = unsafe { fs::cwd_name() };
    framebuffer::print_str(name.as_bytes(), GREEN);
    framebuffer::print_byte(b'\n', WHITE);
}

pub fn cd(args: &[&str]) {
    let dir = args.first().copied().unwrap_or("");
    if dir.is_empty() {
        framebuffer::print_str(b"Usage: cd <dir>\n", RED);
        return;
    }
    if let Err(e) = unsafe { fs::chdir(dir) } {
        framebuffer::print_str(e.as_bytes(), RED);
        framebuffer::print_byte(b'\n', WHITE);
    }
}

pub fn mkdir(args: &[&str]) {
    let name = args.first().copied().unwrap_or("");
    if name.is_empty() {
        framebuffer::print_str(b"Usage: mkdir <dir>\n", RED);
        return;
    }
    match unsafe { fs::create_dir(name) } {
        Ok(_)  => { framebuffer::print_str(b"Created: ", GREEN); framebuffer::print_str(name.as_bytes(), GREEN); framebuffer::print_byte(b'\n', WHITE); }
        Err(e) => { framebuffer::print_str(e.as_bytes(), RED); framebuffer::print_byte(b'\n', WHITE); }
    }
}

pub fn touch(args: &[&str]) {
    let name = args.first().copied().unwrap_or("");
    if name.is_empty() {
        framebuffer::print_str(b"Usage: touch <file>\n", RED);
        return;
    }
    match unsafe { fs::create_file(name) } {
        Ok(_)  => { framebuffer::print_str(b"Created: ", GREEN); framebuffer::print_str(name.as_bytes(), GREEN); framebuffer::print_byte(b'\n', WHITE); }
        Err(e) => { framebuffer::print_str(e.as_bytes(), RED); framebuffer::print_byte(b'\n', WHITE); }
    }
}

pub fn rm(args: &[&str]) {
    let name = args.first().copied().unwrap_or("");
    if name.is_empty() {
        framebuffer::print_str(b"Usage: rm <file>\n", RED);
        return;
    }
    match unsafe { fs::remove(name) } {
        Ok(_)  => { framebuffer::print_str(b"Removed: ", GREEN); framebuffer::print_str(name.as_bytes(), GREEN); framebuffer::print_byte(b'\n', WHITE); }
        Err(e) => { framebuffer::print_str(e.as_bytes(), RED); framebuffer::print_byte(b'\n', WHITE); }
    }
}

pub fn cat(args: &[&str]) {
    let name = args.first().copied().unwrap_or("");
    if name.is_empty() {
        framebuffer::print_str(b"Usage: cat <file>\n", RED);
        return;
    }
    match unsafe { fs::read(name) } {
        Ok(data) => { framebuffer::print_str(data, WHITE); framebuffer::print_byte(b'\n', WHITE); }
        Err(e)   => { framebuffer::print_str(e.as_bytes(), RED); framebuffer::print_byte(b'\n', WHITE); }
    }
}

pub fn write(args: &[&str]) {
    if args.len() < 2 {
        framebuffer::print_str(b"Usage: write <file> <text>\n", RED);
        return;
    }
    let name = args[0];
    let text = args[1..].join(" ");
    match unsafe { fs::write(name, text.as_bytes()) } {
        Ok(_)  => { framebuffer::print_str(b"Written: ", GREEN); framebuffer::print_str(name.as_bytes(), GREEN); framebuffer::print_byte(b'\n', WHITE); }
        Err(e) => { framebuffer::print_str(e.as_bytes(), RED); framebuffer::print_byte(b'\n', WHITE); }
    }
}
