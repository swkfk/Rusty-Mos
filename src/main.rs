#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![no_std]
#![no_main]

pub mod kern;

use core::{arch::global_asm, include_str, panic::PanicInfo};

use kern::machine::print_charc;

global_asm!(include_str!("start.S"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_mips_main() {
    print_charc(b'H');
    panic!();
}
