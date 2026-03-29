mod font;
use font::FONT;
use bootloader_api::info::PixelFormat;

// Character size 
pub const CHAR_W: usize = 8;
pub const CHAR_H: usize = 8;

//RGB
pub const WHITE: (u8, u8, u8) = (220, 220, 220);
pub const BLACK: (u8, u8, u8) = (0,   0,   0);
pub const GREEN: (u8, u8, u8) = (80,  220, 80);
pub const RED:   (u8, u8, u8) = (255, 0,   0);
pub const YELLOW: (u8, u8, u8) = (255, 220, 0);

// Framebuffer state
static mut FB_PTR:    *mut u8     = core::ptr::null_mut();
static mut FB_WIDTH:  usize       = 0;
static mut FB_HEIGHT: usize       = 0;
static mut FB_STRIDE: usize       = 0; // bytes per row 
static mut FB_BPP:    usize       = 0; // bytes per pixel 
static mut FB_FORMAT: PixelFormat = PixelFormat::Bgr;

static mut CURSOR_COL: usize = 0;
static mut CURSOR_ROW: usize = 0;

// Init
pub unsafe fn init(
    ptr: *mut u8,
    width: usize,
    height: usize,
    stride: usize,
    bpp: usize,
    format: PixelFormat,
) {
    FB_PTR    = ptr;
    FB_WIDTH  = width;
    FB_HEIGHT = height;
    FB_STRIDE = stride * bpp;
    FB_BPP    = bpp;
    FB_FORMAT = format;

    // Fill with black / black background
    core::ptr::write_bytes(FB_PTR, 0, FB_HEIGHT * FB_STRIDE);
}

// Number of text columns and rows
unsafe fn cols() -> usize { FB_WIDTH  / CHAR_W }
unsafe fn rows() -> usize { FB_HEIGHT / CHAR_H }

// Write a single pixel
unsafe fn put_pixel(x: usize, y: usize, r: u8, g: u8, b: u8) {
    if x >= FB_WIDTH || y >= FB_HEIGHT { return; }
    let offset = y * FB_STRIDE + x * FB_BPP;
    let ptr = FB_PTR.add(offset);
    match FB_FORMAT {
        PixelFormat::Rgb => { ptr.write_volatile(r); ptr.add(1).write_volatile(g); ptr.add(2).write_volatile(b); }
        PixelFormat::Bgr => { ptr.write_volatile(b); ptr.add(1).write_volatile(g); ptr.add(2).write_volatile(r); }
        _                => { ptr.write_volatile(r); ptr.add(1).write_volatile(g); ptr.add(2).write_volatile(b); }
    }
}

unsafe fn draw_char(col: usize, row: usize, ch: u8, color: (u8, u8, u8)) {
    let idx = if ch >= 32 && ch < 128 { (ch - 32) as usize } else { 0 };
    let bitmap = &FONT[idx];
    let px = col * CHAR_W;
    let py = row * CHAR_H;
    for (y, &bits) in bitmap.iter().enumerate() {
        for x in 0..8usize {
            if bits & (0x01 << x) != 0 {
                put_pixel(px + x, py + y, color.0, color.1, color.2);
            } else {
                put_pixel(px + x, py + y, 0, 0, 0);
            }
        }
    }
}

// Scroll screen up by one line
unsafe fn scroll() {
    let row_bytes = CHAR_H * FB_STRIDE;
    let total     = rows() * row_bytes;
    core::ptr::copy(FB_PTR.add(row_bytes), FB_PTR, total - row_bytes);
    // Clear last row / Очистить последнюю строку
    let last = (rows() - 1) * CHAR_H;
    for y in last..last + CHAR_H {
        for x in 0..FB_WIDTH { put_pixel(x, y, 0, 0, 0); }
    }
}

// API

// Print a single byte with color
pub unsafe fn print_byte(ch: u8, color: (u8, u8, u8)) {
    if ch == b'\n' {
        CURSOR_COL = 0;
        CURSOR_ROW += 1;
    } else {
        draw_char(CURSOR_COL, CURSOR_ROW, ch, color);
        CURSOR_COL += 1;
        if CURSOR_COL >= cols() {
            CURSOR_COL = 0;
            CURSOR_ROW += 1;
        }
    }
    if CURSOR_ROW >= rows() {
        scroll();
        CURSOR_ROW = rows() - 1;
    }
}

// Print a byte string with color
pub unsafe fn print_str(s: &[u8], color: (u8, u8, u8)) {
    for &b in s { print_byte(b, color); }
}

// Clear screen and reset cursor
pub unsafe fn clear() {
    core::ptr::write_bytes(FB_PTR, 0, FB_HEIGHT * FB_STRIDE);
    CURSOR_COL = 0;
    CURSOR_ROW = 0;
}

// backspace
pub unsafe fn backspace() {
    if CURSOR_COL > 0 {
        CURSOR_COL -= 1;
        draw_char(CURSOR_COL, CURSOR_ROW, b' ', BLACK);
    }
}