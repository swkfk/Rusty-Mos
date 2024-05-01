#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![no_std]
#![no_main]

use core::{arch::global_asm, include_str, panic::PanicInfo};

use rusty_mos::{
    debugln,
    kern::{
        env::env_init,
        machine::halt,
        pmap::{mips_detect_memory, mips_vm_init, page_init},
    },
    println, CALL_TEST,
};

global_asm!(include_str!("kasm/include/inc.S"));
global_asm!(include_str!("kasm/tlb.S"));
global_asm!(include_str!("start.S"));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\x1b[31mKernel Panic!");
    println!("{}\x1b[0m", info);
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

    let memsize = ram_low_size as usize;
    let mut freemem: usize = 0;

    mips_detect_memory(memsize);
    mips_vm_init(&mut freemem, memsize);

    page_init(&mut freemem);

    CALL_TEST!(test_linklist; ());
    CALL_TEST!(test_page; ());
    CALL_TEST!(test_page_strong; ());
    CALL_TEST!(test_tlb_refill; ());

    env_init();

    CALL_TEST!(test_tailq; ());
    CALL_TEST!(test_envs; ());
    CALL_TEST!(test_envid2env; ());

    halt();
}
