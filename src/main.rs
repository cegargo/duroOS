#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::vga::print_characters;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("duroOS\n\n\n");
    println!("Made with Rust. Super+Q or Ctrl+Alt+Q to exit.\n\n");
    print_characters();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
