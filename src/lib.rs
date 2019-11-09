#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

use core::panic::PanicInfo;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    let count = tests.len();
    if count > 0 {
        serial_println!("\nRunning {} tests", count);
        for (i, test) in tests.iter().enumerate() {
            serial_print!("{:>3}/{:<3}", i+1, count);
            test();
        }
        serial_println!();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("    Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// test suite helper
#[macro_export]
macro_rules! tests {
    {$($name:ident $body:block)*} => {
        $(
            #[cfg(test)]
            #[test_case]
            fn $name() {
                $crate::serial_print!("{:60} ", stringify!($name));
                $body
                $crate::serial_println!("[ok]");
            }
        )*
    };
}
