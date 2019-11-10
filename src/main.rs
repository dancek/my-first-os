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
    loop {
        use my1os::serial_print;
        for _ in 0..10000 {}
        serial_print!("-");
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my1os::test_panic_handler(info)
}
