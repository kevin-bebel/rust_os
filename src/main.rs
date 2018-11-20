// Build with command 
//https://os.phil-opp.com/

/*
 * Since we are creating an Operating system we cannot use anything that rely on another Operating system to function
 * So we are disabling the rust standard Library
*/
#![no_std] 
#![no_main]
use core::panic::PanicInfo;
//The Panic Handler is a function that the compiler executes when a [Panic] occurs
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}