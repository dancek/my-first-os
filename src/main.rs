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

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

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
