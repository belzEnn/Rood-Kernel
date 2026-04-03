use core::marker::PhantomData;

// Trait for types that can be read/written via port
pub trait PortValue: Copy {
    unsafe fn read_port(port: u16) -> Self;
    unsafe fn write_port(port: u16, val: Self);
}

impl PortValue for u8 {
    unsafe fn read_port(port: u16) -> u8 {
        let v: u8;
        core::arch::asm!("in al, dx", out("al") v, in("dx") port, options(nomem, nostack));
        v
    }
    unsafe fn write_port(port: u16, val: u8) {
        core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack));
    }
}

impl PortValue for u16 {
    unsafe fn read_port(port: u16) -> u16 {
        let v: u16;
        core::arch::asm!("in ax, dx", out("ax") v, in("dx") port, options(nomem, nostack));
        v
    }
    unsafe fn write_port(port: u16, val: u16) {
        core::arch::asm!("out dx, ax", in("dx") port, in("ax") val, options(nomem, nostack));
    }
}

impl PortValue for u32 {
    unsafe fn read_port(port: u16) -> u32 {
        let v: u32;
        core::arch::asm!("in eax, dx", out("eax") v, in("dx") port, options(nomem, nostack));
        v
    }
    unsafe fn write_port(port: u16, val: u32) {
        core::arch::asm!("out dx, eax", in("dx") port, in("eax") val, options(nomem, nostack));
    }
}

//Read-only port
pub struct PortRead<T: PortValue> {
    port: u16,
    _phantom: PhantomData<T>,
}

// Write-only port
pub struct PortWrite<T: PortValue> {
    port: u16,
    _phantom: PhantomData<T>,
}

// Read-write port
pub struct Port<T: PortValue> {
    port: u16,
    _phantom: PhantomData<T>,
}

impl<T: PortValue> PortRead<T> {
    pub const fn new(port: u16) -> Self {
        Self { port, _phantom: PhantomData }
    }
    // Reading is safe
    pub fn read(&self) -> T {
        unsafe { T::read_port(self.port) }
    }
}

impl<T: PortValue> PortWrite<T> {
    pub const fn new(port: u16) -> Self {
        Self { port, _phantom: PhantomData }
    }
    // Writing affects hardware
    pub fn write(&self, val: T) {
        unsafe { T::write_port(self.port, val) }
    }
}

impl<T: PortValue> Port<T> {
    pub const fn new(port: u16) -> Self {
        Self { port, _phantom: PhantomData }
    }
    pub fn read(&self) -> T {
        unsafe { T::read_port(self.port) }
    }
    pub fn write(&self, val: T) {
        unsafe { T::write_port(self.port, val) }
    }
}

unsafe impl<T: PortValue> Sync for Port<T> {}
unsafe impl<T: PortValue> Sync for PortRead<T> {}
unsafe impl<T: PortValue> Sync for PortWrite<T> {}