#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my1os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use my1os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}!", "world");

    my1os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    my1os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    my1os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my1os::test_panic_handler(info)
}
