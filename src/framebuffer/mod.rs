mod font;
use font::FONT;
use bootloader_api::info::PixelFormat;
use spinning_top::Spinlock;

pub const CHAR_W: usize = 8;
pub const CHAR_H: usize = 8;

pub const WHITE:  (u8, u8, u8) = (220, 220, 220);
pub const BLACK:  (u8, u8, u8) = (0,   0,   0);
pub const GREEN:  (u8, u8, u8) = (80,  220, 80);
pub const RED:    (u8, u8, u8) = (255, 0,   0);
pub const YELLOW: (u8, u8, u8) = (255, 220, 0);

struct Framebuffer {
    ptr:    *mut u8,
    width:  usize,
    height: usize,
    stride: usize,
    bpp:    usize,
    format: PixelFormat,
    col:    usize,
    row:    usize,
}

unsafe impl Send for Framebuffer {}
unsafe impl Sync for Framebuffer {}

impl Framebuffer {
    const fn new() -> Self {
        Self {
            ptr:    core::ptr::null_mut(),
            width:  0, height: 0,
            stride: 0, bpp: 0,
            format: PixelFormat::Bgr,
            col: 0, row: 0,
        }
    }

    fn cols(&self) -> usize { self.width  / CHAR_W }
    fn rows(&self) -> usize { self.height / CHAR_H }

    fn put_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x >= self.width || y >= self.height { return; }
        let offset = y * self.stride + x * self.bpp;
        unsafe {
            let ptr = self.ptr.add(offset);
            match self.format {
                PixelFormat::Rgb => { ptr.write_volatile(r); ptr.add(1).write_volatile(g); ptr.add(2).write_volatile(b); }
                PixelFormat::Bgr => { ptr.write_volatile(b); ptr.add(1).write_volatile(g); ptr.add(2).write_volatile(r); }
                _                => { ptr.write_volatile(r); ptr.add(1).write_volatile(g); ptr.add(2).write_volatile(b); }
            }
        }
    }

    fn draw_char(&mut self, col: usize, row: usize, ch: u8, color: (u8, u8, u8)) {
        let idx = if ch >= 32 && ch < 128 { (ch - 32) as usize } else { 0 };
        let bitmap = &FONT[idx];
        let px = col * CHAR_W;
        let py = row * CHAR_H;
        for (y, &bits) in bitmap.iter().enumerate() {
            for x in 0..8usize {
                if bits & (0x01 << x) != 0 {
                    self.put_pixel(px + x, py + y, color.0, color.1, color.2);
                } else {
                    self.put_pixel(px + x, py + y, 0, 0, 0);
                }
            }
        }
    }

    fn scroll(&mut self) {
        let row_bytes = CHAR_H * self.stride;
        let total = self.rows() * row_bytes;
        unsafe { core::ptr::copy(self.ptr.add(row_bytes), self.ptr, total - row_bytes); }
        let last = (self.rows() - 1) * CHAR_H;
        for y in last..last + CHAR_H {
            for x in 0..self.width { self.put_pixel(x, y, 0, 0, 0); }
        }
    }

    fn print_byte_inner(&mut self, ch: u8, color: (u8, u8, u8)) {
        if ch == b'\n' {
            self.col = 0;
            self.row += 1;
        } else {
            let (col, row) = (self.col, self.row);
            self.draw_char(col, row, ch, color);
            self.col += 1;
            if self.col >= self.cols() { self.col = 0; self.row += 1; }
        }
        if self.row >= self.rows() {
            self.scroll();
            self.row = self.rows() - 1;
        }
    }

    fn clear_inner(&mut self) {
        unsafe { core::ptr::write_bytes(self.ptr, 0, self.height * self.stride); }
        self.col = 0;
        self.row = 0;
    }

    fn backspace_inner(&mut self) {
        if self.col > 0 {
            self.col -= 1;
            let (col, row) = (self.col, self.row);
            self.draw_char(col, row, b' ', BLACK);
        }
    }
}

static FB: Spinlock<Framebuffer> = Spinlock::new(Framebuffer::new());

pub fn init(ptr: *mut u8, width: usize, height: usize,
            stride: usize, bpp: usize, format: PixelFormat) {
    let mut fb = FB.lock();
    fb.ptr    = ptr;
    fb.width  = width;
    fb.height = height;
    fb.stride = stride * bpp;
    fb.bpp    = bpp;
    fb.format = format;
    fb.clear_inner();
}

pub fn print_byte(ch: u8, color: (u8, u8, u8)) {
    FB.lock().print_byte_inner(ch, color);
}

pub fn print_str(s: &[u8], color: (u8, u8, u8)) {
    let mut fb = FB.lock();
    for &b in s { fb.print_byte_inner(b, color); }
}

pub fn clear() { FB.lock().clear_inner(); }

pub fn backspace() { FB.lock().backspace_inner(); }
