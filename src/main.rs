#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![no_std]
#![no_main]

use core::{arch::global_asm, include_str};

#[cfg(mos_test)]
use rusty_mos::kernel_tests::unit_test;

use rusty_mos::{
    arch_mipsel::trap::set_exc_base,
    debugln,
    memory::pmap::{mips_detect_memory, mips_vm_init, page_init},
    println,
    process::{envs::env_init, scheduler::schedule},
};

global_asm!(include_str!("arch_mipsel/asm/include/inc.S"));
global_asm!(include_str!("arch_mipsel/asm/tlb.S"));
global_asm!(include_str!("start.S"));
global_asm!(include_str!("arch_mipsel/asm/entry.S"));
global_asm!(include_str!("arch_mipsel/asm/genex.S"));
global_asm!(include_str!("arch_mipsel/asm/kclock.S"));
global_asm!(include_str!("arch_mipsel/asm/env.S"));

#[cfg(mos_build)]
use core::ptr::addr_of;
#[cfg(mos_build)]
use rusty_mos::process::envs::env_create;

/// Read the elf from '/target/user/{icode}' and load it. The priority can be
/// set.
#[cfg(mos_build)]
macro_rules! ENV_CREATE {
    ($icode:expr, $prio:expr) => {
        let b = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/target/user/", $icode));
        env_create(addr_of!(*b) as *const u8, b.len(), 1);
    };
}

/// Start function in rust. Call by the asm function `_start`.
#[no_mangle]
pub extern "C" fn rust_mips_init(
    _argc: u32,
    _argv: *const *const u8,
    _penv: *const *const u8,
    ram_low_size: u32,
) {
    extern "C" {
        fn exc_handler();
    }
    set_exc_base(exc_handler as usize as u32);

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

    #[cfg(mos_test)]
    unit_test();

    #[cfg(mos_build)]
    ENV_CREATE!("icode.b", 1);
    #[cfg(mos_build)]
    ENV_CREATE!("serv.b", 1);

    schedule(false);
}
