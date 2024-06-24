use core::{arch::asm, panic::PanicInfo, sync::atomic::Ordering::SeqCst};

use crate::{
    memory::pmap::CUR_PGDIR,
    print, println,
    process::envs::{CUR_ENV_IDX, ENVS_DATA},
};

use super::machine::halt;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\x1b[31mKernel Panic!");
    let mut x: u32;
    unsafe { asm!("move {}, $29", out(reg) x) };
    print!("  \x1b[31m$sp:    \x1b[32m0x{:08x}", x);
    unsafe { asm!("move {}, $31", out(reg) x) };
    print!("  \x1b[31m$ra:  \x1b[32m0x{:08x}", x);
    unsafe { asm!("mfc0 {}, $12", out(reg) x) };
    print!("  \x1b[31mStatus:  \x1b[32m0x{:08x}\n", x);
    unsafe { asm!("mfc0 {}, $13", out(reg) x) };
    print!("  \x1b[31mCause:  \x1b[32m0x{:08x}", x);
    unsafe { asm!("mfc0 {}, $14", out(reg) x) };
    print!("  \x1b[31mEPC:  \x1b[32m0x{:08x}", x);
    unsafe { asm!("mfc0 {}, $8", out(reg) x) };
    print!("  \x1b[31mBadAddr: \x1b[32m0x{:08x}\n", x);
    print!("  \x1b[31mCurrent Env:\n");
    let env_index = CUR_ENV_IDX.load(SeqCst);
    let env = &ENVS_DATA.borrow().0[env_index];
    print!("    \x1b[31mIndex: \x1b[32m{}", env_index);
    print!("  \x1b[31mId: \x1b[32m{} (0x{:x})", env.id, env.id);
    print!(
        "  \x1b[31mParent: \x1b[32m{} (0x{:x})\n",
        env.parent_id, env.parent_id
    );
    println!(
        "  \x1b[31mCurrent Page Directory: \x1b[32m0x{:x}",
        *CUR_PGDIR.borrow() as usize
    );
    println!("\x1b[31m{}\x1b[0m", info);
    halt();
}
