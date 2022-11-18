#![no_std]
#![no_main]

use core::panic::PanicInfo;

static MESSAGE: &[u8] = b"duroOS. Made with Rust (Super+Q to exit.)";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in MESSAGE.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop{}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}