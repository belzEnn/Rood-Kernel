#![no_std]
#![no_main]

mod framebuffer;
mod keyboard;
mod shell;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use framebuffer::{GREEN, WHITE, RED, YELLOW};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // Init framebuffer
    let fb   = boot_info.framebuffer.as_mut().unwrap();
    let info = fb.info();

    unsafe {
        framebuffer::init(
            fb.buffer_mut().as_mut_ptr(),
            info.width,
            info.height,
            info.stride,
            info.bytes_per_pixel,
            info.pixel_format,
        );

        // Boot message
        framebuffer::print_str(b"Rood\n", YELLOW);
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
