#[allow(dead_code)]
static mut COUNT: u32 = 0;
#[allow(dead_code)]
static mut COUNT800: u32 = 0;
#[allow(dead_code)]
static mut COUNT1001: u32 = 0;

#[cfg(ktest_item = "env_run_1")]
pub fn test_loop() {
    use core::ptr::addr_of;

    use crate::{
        kdef::env::EnvNode,
        kern::{
            env::{env_create, PRE_ENV_RUN},
            machine::halt,
            sched::schedule,
        },
        println,
    };
    unsafe {
        PRE_ENV_RUN = |env: *mut EnvNode| {
            if COUNT > 100 {
                println!("{}: Ticks exceeded the limit 100.", COUNT);
                halt();
            }
            println!("% {}: Count = {}", (*env).data.id, COUNT);
            COUNT += 1;
            match (*env).data.id {
                0x800 => COUNT800 += 1,
                0x1001 => COUNT1001 += 1,
                id => {
                    println!("% {}: Unknown env id. Terminated!", id);
                    halt();
                }
            }
            println!(
                "% {}: env0 count: {}, env1 count: {}, ratio: {}%",
                (*env).data.id,
                COUNT800,
                COUNT1001,
                COUNT1001 * 100 / if COUNT800 == 0 { 1 } else { COUNT800 }
            );
        }
    }

    let icode_loop = include_bytes!("bin/loop.b");
    unsafe {
        env_create(addr_of!(*icode_loop) as *const u8, icode_loop.len(), 1).unwrap();
        env_create(addr_of!(*icode_loop) as *const u8, icode_loop.len(), 2).unwrap();
        schedule(false);
    }
}
