// Interrupts module

pub mod exceptions;
pub mod irq;

use x86_64::structures::idt::InterruptDescriptorTable;
use spinning_top::Spinlock;

// IDT

// IDT must live for the entire kernel lifetime
static IDT: Spinlock<InterruptDescriptorTable> = Spinlock::new(
    InterruptDescriptorTable::new()
);

// Initialize IDT
pub fn init() {
    {
        let mut idt = IDT.lock();

        // Исключения CPU / CPU exceptions
        idt.divide_error.set_handler_fn(exceptions::divide_error);
        idt.debug.set_handler_fn(exceptions::debug);
        idt.breakpoint.set_handler_fn(exceptions::breakpoint);
        idt.invalid_opcode.set_handler_fn(exceptions::invalid_opcode);
        idt.double_fault.set_handler_fn(exceptions::double_fault);
        idt.general_protection_fault.set_handler_fn(exceptions::general_protection);
        idt.page_fault.set_handler_fn(exceptions::page_fault);

        // Hardware interrupts
        idt[irq::PIC1_OFFSET as u8].set_handler_fn(irq::timer);
        idt[(irq::PIC1_OFFSET + 1) as u8].set_handler_fn(irq::keyboard);
    }

    // Load IDT
    unsafe {
        IDT.lock().load_unsafe();
    }

    // Initialize PIC
    irq::init_pic();
    // Enable interrupts
    x86_64::instructions::interrupts::enable();
}

// Uptime in seconds (timer ticks / 18.2 Hz)
pub fn uptime_secs() -> u64 {
    irq::ticks() / 18
}
