#[cfg(ktest_item = "env_run_2")]
pub fn test_qsort() {
    use core::ptr::addr_of;

    use crate::{
        kdef::{env::EnvNode, mmu::KSTACKTOP},
        kern::{
            env::{env_create, env_destory, CUR_ENV, PRE_ENV_RUN},
            machine::halt,
            sched::schedule,
            trap::TrapFrame,
        },
        println,
    };

    unsafe {
        PRE_ENV_RUN = |env: *mut EnvNode| {
            let trapframe = if env == CUR_ENV {
                (KSTACKTOP as *const TrapFrame).sub(1)
            } else {
                addr_of!((*env).data.trap_frame)
            };
            let epc = (*trapframe).cp0_epc;
            if epc == 0x400180 {
                println!(
                    "% {}: Reach the end pc: 0x{:x}, $v0: 0x{:x}",
                    (*env).data.id,
                    epc,
                    (*trapframe).regs[2]
                );
                env_destory(env);
                schedule(false);
            }
        }
    }

    let icode_qsort = include_bytes!("bin/quick_sort.b");
    unsafe {
        env_create(addr_of!(*icode_qsort) as *const u8, icode_qsort.len(), 1).unwrap();
        env_create(addr_of!(*icode_qsort) as *const u8, icode_qsort.len(), 2).unwrap();
        env_create(addr_of!(*icode_qsort) as *const u8, icode_qsort.len(), 3).unwrap();
        schedule(false);
    }
}
