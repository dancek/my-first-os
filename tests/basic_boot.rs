#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my1os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use my1os::{println, tests};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my1os::test_panic_handler(info)
}

tests! {
    test_println {
        println!("test_println output");
    }
}
