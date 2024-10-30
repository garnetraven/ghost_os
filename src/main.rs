#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This is called on panic (fatal unrecoverable error).
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// entry point function for runtime
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
