use x86_64::structures::idt::InterruptStackFrame;
use crate::framebuffer::{self, RED, WHITE, YELLOW};

// Print exception info
fn print_exception(name: &str, frame: &InterruptStackFrame) {
    framebuffer::print_str(b"\n[EXCEPTION] ", RED);
    framebuffer::print_str(name.as_bytes(), RED);
    framebuffer::print_byte(b'\n', WHITE);

    framebuffer::print_str(b"IP:  0x", YELLOW);
    print_hex_u64(frame.instruction_pointer.as_u64());
    framebuffer::print_byte(b'\n', WHITE);

    framebuffer::print_str(b"CS:  0x", YELLOW);
    print_hex_u64(frame.code_segment.0 as u64);
    framebuffer::print_byte(b'\n', WHITE);

    framebuffer::print_str(b"SS:  0x", YELLOW);
    print_hex_u64(frame.stack_segment.0 as u64);
    framebuffer::print_byte(b'\n', WHITE);
}

// Print u64 as hex
fn print_hex_u64(val: u64) {
    let hex = b"0123456789ABCDEF";
    for i in (0..16).rev() {
        let nibble = ((val >> (i * 4)) & 0xF) as usize;
        framebuffer::print_byte(hex[nibble], YELLOW);
    }
}

// #0 Division by zero
pub extern "x86-interrupt" fn divide_error(frame: InterruptStackFrame) {
    print_exception("Divide Error (#DE)", &frame);
    loop { unsafe { core::arch::asm!("hlt"); } }
}

// #1 Отладка / Debug
pub extern "x86-interrupt" fn debug(frame: InterruptStackFrame) {
    print_exception("Debug (#DB)", &frame);
    loop { unsafe { core::arch::asm!("hlt"); } }
}

// #3 Breakpoint
pub extern "x86-interrupt" fn breakpoint(frame: InterruptStackFrame) {
    framebuffer::print_str(b"\n[BREAKPOINT] hit\n", YELLOW);
}

// #6 Invalid opcode
pub extern "x86-interrupt" fn invalid_opcode(frame: InterruptStackFrame) {
    print_exception("Invalid Opcode (#UD)", &frame);
    loop { unsafe { core::arch::asm!("hlt"); } }
}

// #8 Double fault
pub extern "x86-interrupt" fn double_fault(
    frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    print_exception("Double Fault (#DF)", &frame);
    loop { unsafe { core::arch::asm!("hlt"); } }
}

// #13 General protection fault
pub extern "x86-interrupt" fn general_protection(
    frame: InterruptStackFrame,
    error_code: u64,
) {
    print_exception("General Protection Fault (#GP)", &frame);
    framebuffer::print_str(b"Error: 0x", RED);
    print_hex_u64(error_code);
    framebuffer::print_byte(b'\n', WHITE);
    loop { unsafe { core::arch::asm!("hlt"); } }
}

// #14  Page fault
pub extern "x86-interrupt" fn page_fault(
    frame: InterruptStackFrame,
    error_code: x86_64::structures::idt::PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    print_exception("Page Fault (#PF)", &frame);
    framebuffer::print_str(b"Addr: 0x", RED);
    print_hex_u64(Cr2::read_raw());
    framebuffer::print_byte(b'\n', WHITE);
    framebuffer::print_str(b"Code: 0x", RED);
    print_hex_u64(error_code.bits());
    framebuffer::print_byte(b'\n', WHITE);
    loop { unsafe { core::arch::asm!("hlt"); } }
}
