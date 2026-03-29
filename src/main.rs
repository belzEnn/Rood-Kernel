#![no_std]
#![no_main]

mod framebuffer;
mod keyboard;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use framebuffer::{GREEN, WHITE, RED};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // init framebuffer
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

        framebuffer::print_str(b"Rood\n", RED);
        framebuffer::print_str(b"Hello, World!\n\n",   GREEN);
        framebuffer::print_str(b"> ",                  WHITE);

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
