#![no_std] // baremetal, we cannot link the std
#![no_main] // not using normal entry point - crt0 into rustruntime

use core::panic::PanicInfo;

// fn main() {} // no main - no runtime calls it ...

static HELLO: &[u8] = b"Hello World!";

// our own os entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // because we are not using crt0, we define _start that is directly invoked by the os
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // char byte
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color byte, cyan
        }        
    }

    loop {}
}

#[panic_handler] // we need a panic handler, since no std linked, responsible for showing file, line, msg
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
// stack unwinding is disabled in cargo.toml, we just abort
