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

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
// On Windows:
#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    main();
}