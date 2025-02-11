use core::arch::global_asm;

global_asm!(include_str!("asm/include/inc.S"));

global_asm!(include_str!("start.S"));
global_asm!(include_str!("asm/tlb.S"));
global_asm!(include_str!("asm/entry.S"));
global_asm!(include_str!("asm/genex.S"));
global_asm!(include_str!("asm/kclock.S"));
global_asm!(include_str!("asm/env.S"));
