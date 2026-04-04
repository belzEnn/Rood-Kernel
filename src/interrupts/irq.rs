use x86_64::structures::idt::InterruptStackFrame;
use crate::drivers::port::{Port, PortWrite};

// Programmable Interrupt Controller

// Порты PIC / PIC ports
fn pic1_cmd()  -> PortWrite<u8> { PortWrite::new(0x20) }
fn pic1_data() -> Port<u8>      { Port::new(0x21) }
fn pic2_cmd()  -> PortWrite<u8> { PortWrite::new(0xA0) }
fn pic2_data() -> Port<u8>      { Port::new(0xA1) }

// EOI command
const PIC_EOI: u8 = 0x20;

// Vector offsets
pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = 40;

// Initialize PIC
pub fn init_pic() {
    // Start init
    pic1_cmd().write(0x11);
    pic2_cmd().write(0x11);

    // Offsets
    pic1_data().write(PIC1_OFFSET);
    pic2_data().write(PIC2_OFFSET);

    // Cascade
    pic1_data().write(4);
    pic2_data().write(2);

    // 8086 mode
    pic1_data().write(0x01);
    pic2_data().write(0x01);
    
    pic1_data().write(0b11111100); 
    pic2_data().write(0b11111111); 
}

// Send EOI
fn eoi_master() { pic1_cmd().write(PIC_EOI); }
fn eoi_slave()  { pic2_cmd().write(PIC_EOI); pic1_cmd().write(PIC_EOI); }

// IRQ 0 Timer

static mut TICKS: u64 = 0;

pub extern "x86-interrupt" fn timer(_frame: InterruptStackFrame) {
    unsafe { TICKS += 1; }
    eoi_master();
}

pub fn ticks() -> u64 {
    unsafe { TICKS }
}

// IRQ 1 Keyboard

pub extern "x86-interrupt" fn keyboard(_frame: InterruptStackFrame) {
    use crate::drivers::input::ps2::{self, HandleResult};
    use crate::framebuffer::{self, WHITE};

    // Читаем скан-код / Read scancode
    if let Some(sc) = ps2::try_read_scancode() {
        match ps2::handle_scancode(sc) {
            Some(HandleResult::Char(c)) => {
                let mut buf = [0u8; 4];
                let s = c.encode_utf8(&mut buf);
                framebuffer::print_str(s.as_bytes(), WHITE);
            }
            Some(HandleResult::Backspace) => {
                framebuffer::backspace();
            }
            Some(HandleResult::Enter(cmd)) => {
                framebuffer::print_byte(b'\n', WHITE);
                crate::shell::execute(&cmd);
                framebuffer::print_str(b"> ", WHITE);
            }
            None => {}
        }
    }

    eoi_master();
}
