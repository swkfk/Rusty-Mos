#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![no_std]
#![no_main]

use core::{arch::global_asm, include_str, panic::PanicInfo};

use rusty_mos::{kern::machine::halt, println};

global_asm!(include_str!("start.S"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_mips_main() {
    println!("Rusty Mos, By kai_Ker");
    println!("Transplanted From the C-Edition Mos of BUAA OS Course");
    println!("> main.rs: rust_mips_main() has been called");
    println!("Simple string format: {}", "Compiler will done it!");
    println!("Simple integar format: {}", 123);
    println!();
    halt();
}
