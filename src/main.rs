#![no_std] // don't link the standard Rust library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle] // don't mangle (unique encoded string) the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named '_start' by default
    println!("Ghost OS{}", "!");
    
    panic!("Some panic message");

    loop {}
}

// This is called on panic (fatal unrecoverable errors).
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

