#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

mod allocator;
mod framebuffer;
mod fs;
mod keyboard;
mod shell;
mod drivers;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use framebuffer::{WHITE, YELLOW};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let fb   = boot_info.framebuffer.as_mut().unwrap();
    let info = fb.info();

    unsafe {
        allocator::init();

        framebuffer::init(
            fb.buffer_mut().as_mut_ptr(),
            info.width,
            info.height,
            info.stride,
            info.bytes_per_pixel,
            info.pixel_format,
        );

        fs::init();

        // Boot message
        framebuffer::print_str(b"Rood OS\n", YELLOW);
        framebuffer::print_str(b"Type 'help' for available commands\n\n", WHITE);
        framebuffer::print_str(b"> ", WHITE);

        // Main loop
        loop {
            if let Some(sc) = keyboard::try_read_scancode() {
                keyboard::handle_scancode(sc);
            }
            for _ in 0..10_000u64 {
                core::hint::spin_loop();
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Allocation error handler
#[alloc_error_handler]
fn alloc_error(_layout: alloc::alloc::Layout) -> ! {
    loop {}
}