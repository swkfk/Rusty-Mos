use core::{
    fmt::{Display, Write},
    mem::size_of,
    sync::atomic::Ordering::SeqCst,
};

use crate::{
    debugln,
    kdef::{
        env::EnvStatus,
        error::KError,
        mmu::{KSTACKTOP, PTE_V, UTEMP, UTOP},
        syscall::MAX_SYS_NO,
    },
    kern::env::env_destory,
    memory::pmap::{page_alloc, page_insert, page_lookup, page_remove},
};

use super::{
    env::{env_alloc, envid2env, CUR_ENV_IDX, ENVS_DATA, ENV_SCHE_LIST},
    io::{ioread_into_va, iowrite_from_va},
    machine::{print_charc, scan_charc},
    sched::schedule,
    trap::TrapFrame,
};

// type PureResult = Result<(), KError>;

fn sys_putchar(ch: u8) {
    print_charc(ch);
}

fn sys_print_cons(s: *const u8, num: u32) -> u32 {
    let num = num as usize;
    if s as usize + num > UTOP || s as usize > UTOP || s as usize > s as usize + num {
        return KError::Invalid.into();
    }
    for i in 0..num {
        let s = s.wrapping_add(i);
        print_charc(unsafe { *s });
    }
    0
}

fn sys_getenvid() -> u32 {
    ENVS_DATA.borrow().0[CUR_ENV_IDX.load(SeqCst)].id
}

fn sys_yield() -> ! {
    schedule(true)
}

fn sys_env_destroy(envid: u32) -> u32 {
    let e = envid2env(envid, true);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();
    debugln!(
        "% {}: Destorying {}",
        ENVS_DATA.borrow().0[CUR_ENV_IDX.load(SeqCst)].id,
        ENVS_DATA.borrow().0[e].id
    );
    env_destory(e);
    0
}

fn sys_set_tlb_mod_entry(envid: u32, func: u32) -> u32 {
    let e = envid2env(envid, true);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();
    ENVS_DATA.borrow_mut().0[e].user_tlb_mod_entry = func;
    0
}

fn sys_mem_alloc(envid: u32, va: u32, perm: u32) -> u32 {
    let va = va as usize;
    if !(UTEMP..UTOP).contains(&va) {
        return KError::Invalid.into();
    }
    let e = envid2env(envid, true);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();
    let pp = page_alloc();
    if let Err(e) = pp {
        return e.into();
    }
    let pp = pp.unwrap();
    let r = page_insert(
        ENVS_DATA.borrow().0[e].pgdir,
        va,
        ENVS_DATA.borrow().0[e].asid,
        perm,
        pp,
    );
    if let Err(e) = r {
        return e.into();
    }
    0
}

fn sys_mem_map(src_id: u32, src_va: u32, dst_id: u32, dst_va: u32, perm: u32) -> u32 {
    let src_va = src_va as usize;
    let dst_va = dst_va as usize;
    if !(UTEMP..UTOP).contains(&src_va) || !(UTEMP..UTOP).contains(&dst_va) {
        return KError::Invalid.into();
    }

    let src_env = envid2env(src_id, true);
    if let Err(e) = src_env {
        return e.into();
    }
    let src_env = src_env.unwrap();
    let dst_env = envid2env(dst_id, true);
    if let Err(e) = dst_env {
        return e.into();
    }
    let dst_env = dst_env.unwrap();

    let r = page_lookup(ENVS_DATA.borrow().0[src_env].pgdir, src_va).ok_or(KError::Invalid);
    if let Err(e) = r {
        return e.into();
    }
    let (pp, _) = r.unwrap();

    let r = page_insert(
        ENVS_DATA.borrow().0[dst_env].pgdir,
        dst_va,
        ENVS_DATA.borrow().0[dst_env].asid,
        perm,
        pp,
    );

    if let Err(e) = r {
        e.into()
    } else {
        0
    }
}

fn sys_mem_unmap(envid: u32, va: u32) -> u32 {
    let va = va as usize;
    if !(UTEMP..UTOP).contains(&va) {
        return KError::Invalid.into();
    }
    let e = envid2env(envid, true);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();
    page_remove(
        ENVS_DATA.borrow().0[e].pgdir,
        va,
        ENVS_DATA.borrow().0[e].asid,
    );
    0
}

fn sys_exofork() -> u32 {
    let env_data = ENVS_DATA.borrow();
    let id = env_data.0[CUR_ENV_IDX.load(SeqCst)].id;
    drop(env_data);

    let e = env_alloc(id);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();

    let mut env_data = ENVS_DATA.borrow_mut();
    env_data.0[e].trap_frame = unsafe { *((KSTACKTOP as *mut TrapFrame).sub(1)) };
    env_data.0[e].trap_frame.regs[2] = 0;
    env_data.0[e].status = EnvStatus::NotRunnable;
    env_data.0[e].priority = env_data.0[CUR_ENV_IDX.load(SeqCst)].priority;
    env_data.0[e].id
}

fn sys_set_env_status(envid: u32, status: EnvStatus) -> u32 {
    if status != EnvStatus::NotRunnable && status != EnvStatus::Runnable {
        return KError::Invalid.into();
    }
    let e = envid2env(envid, true);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();

    if ENVS_DATA.borrow().0[e].status == status {
        return 0;
    }

    if status == EnvStatus::Runnable {
        ENV_SCHE_LIST.borrow_mut().insert_tail(e);
    } else {
        ENV_SCHE_LIST.borrow_mut().remove(e);
    }

    ENVS_DATA.borrow_mut().0[e].status = status;
    0
}

fn sys_set_trapframe(envid: u32, trapframe: *mut TrapFrame) -> u32 {
    let len = size_of::<TrapFrame>();
    let va = trapframe as usize;
    if va.checked_add(len).is_none() || va < UTEMP || va + len > UTOP {
        return KError::Invalid.into();
    }
    let env = envid2env(envid, true);
    if let Err(e) = env {
        return e.into();
    }
    let env = env.unwrap();

    if env == CUR_ENV_IDX.load(SeqCst) {
        unsafe { ((KSTACKTOP as *mut TrapFrame).sub(1)).write(*trapframe) };
        unsafe { (*trapframe).regs[2] }
    } else {
        ENVS_DATA.borrow_mut().0[env].trap_frame = unsafe { *trapframe };
        0
    }
}

struct CLikeStr(*const u8);

impl Display for CLikeStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for i in 0.. {
            let b = unsafe { (self.0.add(i)).read_volatile() };
            if b == b'\0' {
                break;
            }
            f.write_char(b as char)?;
        }
        Ok(())
    }
}

fn sys_panic(msg: *const u8) -> ! {
    panic!("{}", CLikeStr(msg));
}

fn sys_ipc_try_send(envid: u32, value: u32, src_va: u32, perm: u32) -> u32 {
    let src_va = src_va as usize;
    if src_va != 0 && !(UTEMP..UTOP).contains(&src_va) {
        return KError::Invalid.into();
    }

    let e = envid2env(envid, false);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();

    let mut env_data = ENVS_DATA.borrow_mut();
    if !env_data.0[e].ipc_data.receiving {
        return KError::IpcNotRecv.into();
    }

    env_data.0[e].ipc_data.value = value;
    env_data.0[e].ipc_data.from_id = env_data.0[CUR_ENV_IDX.load(SeqCst)].id;
    env_data.0[e].ipc_data.perm = perm | PTE_V;
    env_data.0[e].ipc_data.receiving = false;

    env_data.0[e].status = EnvStatus::Runnable;
    ENV_SCHE_LIST.borrow_mut().insert_tail(e);

    if src_va != 0 {
        let r =
            page_lookup(env_data.0[CUR_ENV_IDX.load(SeqCst)].pgdir, src_va).ok_or(KError::Invalid);
        if let Err(e) = r {
            return e.into();
        }
        let (p, _) = r.unwrap();
        if let Err(e) = page_insert(
            env_data.0[e].pgdir,
            env_data.0[e].ipc_data.dstva as usize,
            env_data.0[e].asid,
            perm,
            p,
        ) {
            return e.into();
        }
    }

    0
}

fn sys_ipc_recv(dst_va: u32) -> u32 {
    let dst_va = dst_va as usize;
    if dst_va != 0 && !(UTEMP..UTOP).contains(&dst_va) {
        return KError::Invalid.into();
    }

    let cur_env_idx = CUR_ENV_IDX.load(SeqCst);

    ENVS_DATA.borrow_mut().0[cur_env_idx].ipc_data.receiving = true;
    ENVS_DATA.borrow_mut().0[cur_env_idx].ipc_data.dstva = dst_va as u32;
    ENVS_DATA.borrow_mut().0[cur_env_idx].status = EnvStatus::NotRunnable;
    ENV_SCHE_LIST.borrow_mut().remove(cur_env_idx);

    let ktf = (KSTACKTOP as *mut TrapFrame).wrapping_sub(1);
    unsafe { (*ktf).regs[2] = 0 };
    schedule(true);
}

fn sys_cgetc() -> u8 {
    loop {
        let ch = scan_charc();
        if ch != 0 {
            return ch;
        }
    }
}

fn sys_write_dev(va: u32, pa: u32, len: u32) -> u32 {
    let va = va as usize;
    let pa = pa as usize;
    let len = len as usize;
    if va.checked_add(len).is_none() || va < UTEMP || va + len > UTOP {
        return KError::Invalid.into();
    }
    if !(0x180003f8 <= pa && pa + len <= 0x180003f8 + 0x20
        || 0x180001f0 <= pa && pa + len <= 0x180001f0 + 0x8)
    {
        return KError::Invalid.into();
    }
    match len {
        1 => iowrite_from_va::<u8>(pa, va),
        2 => iowrite_from_va::<u16>(pa, va),
        4 => iowrite_from_va::<u32>(pa, va),
        _ => return KError::Invalid.into(),
    }
    0
}

fn sys_read_dev(va: u32, pa: u32, len: u32) -> u32 {
    let va = va as usize;
    let pa = pa as usize;
    let len = len as usize;
    if va.checked_add(len).is_none() || va < UTEMP || va + len > UTOP {
        return KError::Invalid.into();
    }
    if !(0x180003f8 <= pa && pa + len <= 0x180003f8 + 0x20
        || 0x180001f0 <= pa && pa + len <= 0x180001f0 + 0x8)
    {
        return KError::Invalid.into();
    }
    match len {
        1 => ioread_into_va::<u8>(pa, va),
        2 => ioread_into_va::<u16>(pa, va),
        4 => ioread_into_va::<u32>(pa, va),
        _ => return KError::Invalid.into(),
    }
    0
}

type SyscallRawPtr = *const ();
type SyscallFn = fn(u32, u32, u32, u32, u32) -> u32;

pub const SYSCALL_TABLE: [SyscallRawPtr; MAX_SYS_NO] = [
    sys_putchar as SyscallRawPtr,
    sys_print_cons as SyscallRawPtr,
    sys_getenvid as SyscallRawPtr,
    sys_yield as SyscallRawPtr,
    sys_env_destroy as SyscallRawPtr,
    sys_set_tlb_mod_entry as SyscallRawPtr,
    sys_mem_alloc as SyscallRawPtr,
    sys_mem_map as SyscallRawPtr,
    sys_mem_unmap as SyscallRawPtr,
    sys_exofork as SyscallRawPtr,
    sys_set_env_status as SyscallRawPtr,
    sys_set_trapframe as SyscallRawPtr,
    sys_panic as SyscallRawPtr,
    sys_ipc_try_send as SyscallRawPtr,
    sys_ipc_recv as SyscallRawPtr,
    sys_cgetc as SyscallRawPtr,
    sys_write_dev as SyscallRawPtr,
    sys_read_dev as SyscallRawPtr,
];

/// # Safety
///
#[no_mangle]
pub fn do_syscall(trapframe: *mut TrapFrame) {
    let tf_p = trapframe; // deceits
    let tf = unsafe { *tf_p };
    let sysno = tf.regs[4];
    if !(0..MAX_SYS_NO as u32).contains(&sysno) {
        unsafe { (*tf_p).regs[2] = KError::NoSys as u32 }
        return;
    }
    unsafe { (*tf_p).cp0_epc += size_of::<u32>() as u32 }

    let func =
        unsafe { core::mem::transmute::<SyscallRawPtr, SyscallFn>(SYSCALL_TABLE[sysno as usize]) };
    let arg1 = tf.regs[5];
    let arg2 = tf.regs[6];
    let arg3 = tf.regs[7];
    let addr = (tf.regs[29] as *const u32).wrapping_add(4);
    let arg4 = unsafe { addr.read() };
    let addr = (tf.regs[29] as *const u32).wrapping_add(5);
    let arg5 = unsafe { addr.read() };

    unsafe { (*tf_p).regs[2] = func(arg1, arg2, arg3, arg4, arg5) }
}
