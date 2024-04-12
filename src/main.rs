#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![no_std]
#![no_main]

use core::{arch::global_asm, include_str, panic::PanicInfo};

use rusty_mos::{kern::machine::halt, println, printnumln};

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
    printnumln!("> main.rs: the arg ram_low_size is", ram_low_size as i32);
    println!("Rusty Mos, By kai_Ker");
    println!("Transplanted From the C-Edition Mos of BUAA OS Course");
    println!("Simple string format: {}", "Compiler will done it!");
    println!("Simple integer format: {}", 123);
    println!();
    printnumln!("With no integer");
    printnumln!("With an integer:", 1024);
    printnumln!("With an integer:", 1029; 16);
    printnumln!("With an integer:", 1029; 8);
    printnumln!("With an integer:", 1029; 2);
    printnumln!("With an integer:", 1029; 7);
    printnumln!("With a zero:", 0);
    printnumln!("With a negetive:", -123);
    printnumln!("With a lot of numbers:", 123, 321, 666; 16);
    println!();
    halt();
}
