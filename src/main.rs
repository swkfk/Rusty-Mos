#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![no_std]
#![no_main]

use core::{arch::global_asm, include_str, panic::PanicInfo, ptr};

use rusty_mos::{
    debugln,
    kern::{
        machine::halt,
        pmap::{mips_detect_memory, mips_vm_init, Page},
    },
    print, println,
};

global_asm!(include_str!("start.S"));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("Kernel Panic!");
    println!("{}", info);
    halt();
}

#[no_mangle]
pub extern "C" fn rust_mips_init(
    _argc: u32,
    _argv: *const *const u8,
    _penv: *const *const u8,
    ram_low_size: u32,
) {
    debugln!("> main.rs: rust_mips_init() has been called");

    println!("Rusty Mos, By kai_Ker");
    println!("Transplanted From the C-Edition Mos of BUAA OS Course");

    println!("Ram low size={}", ram_low_size);
    println!();

    let mut npage: usize = 0;
    let memsize = ram_low_size as usize;
    let mut freemem: usize = 0;
    let mut pages: *mut Page = ptr::null_mut();

    mips_detect_memory(&mut npage, memsize);
    mips_vm_init(&mut pages, &mut freemem, npage, memsize);

    halt();
}
