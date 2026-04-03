mod scancodes;

use crate::drivers::port::{Port, PortRead};
use spinning_top::Spinlock;
use alloc::string::String;

fn port_data()   -> Port<u8>     { Port::new(0x60) }
fn port_status() -> PortRead<u8> { PortRead::new(0x64) }

struct Ps2State {
    input: String,
    shift: bool,
}

static STATE: Spinlock<Ps2State> = Spinlock::new(Ps2State {
    input: String::new(),
    shift: false,
});

pub fn try_read_scancode() -> Option<u8> {
    if port_status().read() & 1 != 0 { Some(port_data().read()) } else { None }
}

pub fn handle_scancode(sc: u8) -> Option<HandleResult> {
    let mut state = STATE.lock();
    match sc {
        0x2A | 0x36 => { state.shift = true;  None }
        0xAA | 0xB6 => { state.shift = false; None }
        0x0E => {
            if state.input.is_empty() { return None; }
            state.input.pop();
            Some(HandleResult::Backspace)
        }
        0x1C => {
            let cmd = state.input.clone();
            state.input.clear();
            Some(HandleResult::Enter(cmd))
        }
        _ => {
            if let Some(c) = scancodes::scancode_to_char(sc, state.shift) {
                if state.input.len() < 255 {
                    state.input.push(c);
                    Some(HandleResult::Char(c))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

pub enum HandleResult {
    Char(char),
    Backspace,
    Enter(String),
}
