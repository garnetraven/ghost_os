#![no_std] // don't link the standard Rust library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"ghost OS";

#[no_mangle] // don't mangle (unique encoded string) the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named '_start' by default
    
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
            
        }
    }

    loop {}
}

// This is called on panic (fatal unrecoverable errors).
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

