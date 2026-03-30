// ls, cat, mkdir, touch, rm, cd, pwd, write

use crate::framebuffer::{self, WHITE, GREEN, RED, YELLOW};
use crate::fs;
use super::MAX_ARGS;

// Список файлов / List files
pub unsafe fn ls() {
    let entries = fs::list();
    if entries.is_empty() {
        framebuffer::print_str(b"(empty)\n", WHITE);
        return;
    }
    for (name, is_dir) in entries {
        if is_dir {
            // Директории синим / Dirs in blue
            framebuffer::print_str(name.as_bytes(), YELLOW);
            framebuffer::print_str(b"/\n", YELLOW);
        } else {
            framebuffer::print_str(name.as_bytes(), WHITE);
            framebuffer::print_byte(b'\n', WHITE);
        }
    }
}

// Вывести содержимое файла / Print file contents
pub unsafe fn cat(args: [&[u8]; MAX_ARGS], argc: usize) {
    if argc < 2 {
        framebuffer::print_str(b"Usage: cat <file>\n", RED);
        return;
    }
    let name = core::str::from_utf8(args[1]).unwrap_or("");
    match fs::read(name) {
        Ok(data) => {
            framebuffer::print_str(data, WHITE);
            framebuffer::print_byte(b'\n', WHITE);
        }
        Err(e) => {
            framebuffer::print_str(e.as_bytes(), RED);
            framebuffer::print_byte(b'\n', WHITE);
        }
    }
}

// Создать директорию / Create directory
pub unsafe fn mkdir(args: [&[u8]; MAX_ARGS], argc: usize) {
    if argc < 2 {
        framebuffer::print_str(b"Usage: mkdir <name>\n", RED);
        return;
    }
    let name = core::str::from_utf8(args[1]).unwrap_or("");
    match fs::create_dir(name) {
        Ok(_)  => {
            framebuffer::print_str(b"Created: ", GREEN);
            framebuffer::print_str(args[1], GREEN);
            framebuffer::print_byte(b'\n', WHITE);
        }
        Err(e) => {
            framebuffer::print_str(e.as_bytes(), RED);
            framebuffer::print_byte(b'\n', WHITE);
        }
    }
}

// Создать пустой файл / Create empty file
pub unsafe fn touch(args: [&[u8]; MAX_ARGS], argc: usize) {
    if argc < 2 {
        framebuffer::print_str(b"Usage: touch <file>\n", RED);
        return;
    }
    let name = core::str::from_utf8(args[1]).unwrap_or("");
    match fs::create_file(name) {
        Ok(_)  => {
            framebuffer::print_str(b"Created: ", GREEN);
            framebuffer::print_str(args[1], GREEN);
            framebuffer::print_byte(b'\n', WHITE);
        }
        Err(e) => {
            framebuffer::print_str(e.as_bytes(), RED);
            framebuffer::print_byte(b'\n', WHITE);
        }
    }
}

// Удалить файл / Remove file
pub unsafe fn rm(args: [&[u8]; MAX_ARGS], argc: usize) {
    if argc < 2 {
        framebuffer::print_str(b"Usage: rm <file>\n", RED);
        return;
    }
    let name = core::str::from_utf8(args[1]).unwrap_or("");
    match fs::remove(name) {
        Ok(_)  => {
            framebuffer::print_str(b"Removed: ", GREEN);
            framebuffer::print_str(args[1], GREEN);
            framebuffer::print_byte(b'\n', WHITE);
        }
        Err(e) => {
            framebuffer::print_str(e.as_bytes(), RED);
            framebuffer::print_byte(b'\n', WHITE);
        }
    }
}

// Сменить директорию / Change directory
pub unsafe fn cd(args: [&[u8]; MAX_ARGS], argc: usize) {
    if argc < 2 {
        framebuffer::print_str(b"Usage: cd <dir>\n", RED);
        return;
    }
    let name = core::str::from_utf8(args[1]).unwrap_or("");
    match fs::chdir(name) {
        Ok(_)  => {}
        Err(e) => {
            framebuffer::print_str(e.as_bytes(), RED);
            framebuffer::print_byte(b'\n', WHITE);
        }
    }
}

// Текущая директория / Print working directory
pub unsafe fn pwd() {
    framebuffer::print_str(fs::cwd_name().as_bytes(), GREEN);
    framebuffer::print_byte(b'\n', WHITE);
}

// Записать текст в файл / Write text to file
// Использование / Usage: write <file> <text...>
pub unsafe fn write(args: [&[u8]; MAX_ARGS], argc: usize) {
    if argc < 3 {
        framebuffer::print_str(b"Usage: write <file> <text>\n", RED);
        return;
    }
    let name = core::str::from_utf8(args[1]).unwrap_or("");

    // Собираем текст из аргументов / Collect text from args
    let mut buf = [0u8; 512];
    let mut len = 0;
    for i in 2..argc {
        for &b in args[i] {
            if len < buf.len() { buf[len] = b; len += 1; }
        }
        if i < argc - 1 && len < buf.len() {
            buf[len] = b' '; len += 1;
        }
    }

    match fs::write(name, &buf[..len]) {
        Ok(_)  => {
            framebuffer::print_str(b"Written to: ", GREEN);
            framebuffer::print_str(args[1], GREEN);
            framebuffer::print_byte(b'\n', WHITE);
        }
        Err(e) => {
            framebuffer::print_str(e.as_bytes(), RED);
            framebuffer::print_byte(b'\n', WHITE);
        }
    }
}
