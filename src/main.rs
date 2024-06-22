#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![no_std]
#![no_main]

use core::{arch::global_asm, include_str, panic::PanicInfo, ptr::addr_of};

use rusty_mos::{
    debugln,
    kern::{
        env::{env_create, env_init},
        machine::halt,
        sched::schedule,
    },
    memory::pmap::{mips_detect_memory, mips_vm_init, page_init},
    println,
};

global_asm!(include_str!("kasm/include/inc.S"));
global_asm!(include_str!("kasm/tlb.S"));
global_asm!(include_str!("start.S"));
global_asm!(include_str!("kasm/entry.S"));
global_asm!(include_str!("kasm/genex.S"));
global_asm!(include_str!("kasm/kclock.S"));
global_asm!(include_str!("kasm/env.S"));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\x1b[31mKernel Panic!");
    println!("{}\x1b[0m", info);
    halt();
}

macro_rules! ENV_CREATE {
    ($icode:expr, $prio:expr) => {
        let b = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/target/user/", $icode));
        unsafe { env_create(addr_of!(*b) as *const u8, b.len(), 1) };
    };
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

    let memsize = ram_low_size as usize;
    let mut freemem: usize = 0;

    mips_detect_memory(memsize);
    mips_vm_init(&mut freemem, memsize);

    page_init(&mut freemem);

    env_init();

    ENV_CREATE!("icode.b", 1);
    ENV_CREATE!("serv.b", 1);

    schedule(false);
}
