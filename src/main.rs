#![no_std] // don't link the standard Rust library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

#[no_mangle] // don't mangle (unique encoded string) the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named '_start' by default
    loop {}
}

// This is called on panic (fatal unrecoverable errors).
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

