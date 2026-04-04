#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod allocator;
mod framebuffer;
mod fs;
mod shell;
mod drivers;
mod interrupts;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use framebuffer::{WHITE, YELLOW};
use drivers::input::ps2::{self, HandleResult};
use drivers::dfisk::ata;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let fb   = boot_info.framebuffer.as_mut().unwrap();
    let info = fb.info();

    unsafe {
        allocator::init();
        framebuffer::init(
            fb.buffer_mut().as_mut_ptr(),
            info.width, info.height,
            info.stride, info.bytes_per_pixel,
            info.pixel_format,
        );
        interrupts::init();
        fs::init();
    }

    // Check disk
    if !ata::detect_drive(0xA0) && !ata::detect_drive(0xB0) {
        framebuffer::print_str(b"\n[FATAL] No disk detected!\n", framebuffer::RED);
        framebuffer::print_str(b"Rood OS requires a disk.\n", WHITE);
        loop { unsafe { core::arch::asm!("hlt"); } }
    }

    framebuffer::print_str(b"Rood OS\n", YELLOW);
    framebuffer::print_str(b"Type 'help' for commands\n\n", WHITE);
    framebuffer::print_str(b"> ", WHITE);

    // Main loop
    loop {
        if let Some(sc) = ps2::try_read_scancode() {
            match ps2::handle_scancode(sc) {
                Some(HandleResult::Char(c)) => {
                    // Print character
                    let mut buf = [0u8; 4];
                    let s = c.encode_utf8(&mut buf);
                    framebuffer::print_str(s.as_bytes(), WHITE);
                }
                Some(HandleResult::Backspace) => {
                    framebuffer::backspace();
                }
                Some(HandleResult::Enter(cmd)) => {
                    framebuffer::print_byte(b'\n', WHITE);
                    shell::execute(&cmd);
                    framebuffer::print_str(b"> ", WHITE);
                }
                None => {}
            }
        }
        for _ in 0..10_000u64 {
            core::hint::spin_loop();
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

#[alloc_error_handler]
fn alloc_error(_layout: alloc::alloc::Layout) -> ! { loop {} }
