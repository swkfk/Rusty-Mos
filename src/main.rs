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
pub extern "C" fn rust_mips_init(
    _argc: u32,
    _argv: *const *const u8,
    _penv: *const *const u8,
    ram_low_size: u32,
) {
    println!("> main.rs: rust_mips_init() has been called");
    println!("> main.rs: the arg ram_low_size is {}", ram_low_size);
    println!("Rusty Mos, By kai_Ker");
    println!("Transplanted From the C-Edition Mos of BUAA OS Course");
    println!("Simple string format: {}", "Compiler will done it!");
    println!("Simple integar format: {}", 123);
    println!();
    halt();
}
