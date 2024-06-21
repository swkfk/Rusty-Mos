use core::{
    fmt::{Display, Write},
    mem::size_of,
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
    env::{env_alloc, envid2env, CUR_ENV, ENV_SCHE_LIST},
    io::{ioread_into_va, iowrite_from_va},
    machine::{print_charc, scan_charc},
    sched::schedule,
    trap::TrapFrame,
};

// type PureResult = Result<(), KError>;

fn sys_putchar(ch: u8) {
    print_charc(ch);
}

unsafe fn sys_print_cons(s: *const u8, num: u32) -> u32 {
    let num = num as usize;
    if s as usize + num > UTOP || s as usize > UTOP || s as usize > s as usize + num {
        return KError::Invalid.into();
    }
    for i in 0..num {
        print_charc(*(s.add(i)));
    }
    0
}

fn sys_getenvid() -> u32 {
    unsafe { (*(*CUR_ENV).data).id }
}

fn sys_yield() -> ! {
    unsafe { schedule(true) }
}

unsafe fn sys_env_destroy(envid: u32) -> u32 {
    let e = envid2env(envid, true);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();
    debugln!(
        "% {}: Destorying {}",
        (*(*CUR_ENV).data).id,
        (*(*e).data).id
    );
    env_destory(e);
    0
}

unsafe fn sys_set_tlb_mod_entry(envid: u32, func: u32) -> u32 {
    let e = envid2env(envid, true);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();
    (*(*e).data).user_tlb_mod_entry = func;
    0
}

unsafe fn sys_mem_alloc(envid: u32, va: u32, perm: u32) -> u32 {
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
    if let Err(e) = page_insert((*(*e).data).pgdir, va, (*(*e).data).asid, perm, pp) {
        return e.into();
    }
    0
}

unsafe fn sys_mem_map(src_id: u32, src_va: u32, dst_id: u32, dst_va: u32, perm: u32) -> u32 {
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

    let r = page_lookup((*(*src_env).data).pgdir, src_va).ok_or(KError::Invalid);
    if let Err(e) = r {
        return e.into();
    }
    let (pp, _) = r.unwrap();

    if let Err(e) = page_insert(
        (*(*dst_env).data).pgdir,
        dst_va,
        (*(*dst_env).data).asid,
        perm,
        pp,
    ) {
        e.into()
    } else {
        0
    }
}

unsafe fn sys_mem_unmap(envid: u32, va: u32) -> u32 {
    let va = va as usize;
    if !(UTEMP..UTOP).contains(&va) {
        return KError::Invalid.into();
    }
    let e = envid2env(envid, true);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();
    page_remove((*(*e).data).pgdir, va, (*(*e).data).asid);
    0
}

unsafe fn sys_exofork() -> u32 {
    let e = env_alloc((*(*CUR_ENV).data).id);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();
    (*(*e).data).trap_frame = *((KSTACKTOP as *mut TrapFrame).sub(1));
    (*(*e).data).trap_frame.regs[2] = 0;
    (*(*e).data).status = EnvStatus::NotRunnable;
    (*(*e).data).priority = (*(*CUR_ENV).data).priority;
    (*(*e).data).id
}

unsafe fn sys_set_env_status(envid: u32, status: EnvStatus) -> u32 {
    if status != EnvStatus::NotRunnable && status != EnvStatus::Runnable {
        return KError::Invalid.into();
    }
    let e = envid2env(envid, true);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();

    if (*(*e).data).status == status {
        return 0;
    }

    if status == EnvStatus::Runnable {
        ENV_SCHE_LIST.insert_tail(e);
    } else {
        ENV_SCHE_LIST.remove(e);
    }

    (*(*e).data).status = status;
    0
}

unsafe fn sys_set_trapframe(envid: u32, trapframe: *mut TrapFrame) -> u32 {
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

    if env == CUR_ENV {
        ((KSTACKTOP as *mut TrapFrame).sub(1)).write(*trapframe);
        (*trapframe).regs[2]
    } else {
        (*(*env).data).trap_frame = *trapframe;
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

unsafe fn sys_ipc_try_send(envid: u32, value: u32, src_va: u32, perm: u32) -> u32 {
    let src_va = src_va as usize;
    if src_va != 0 && !(UTEMP..UTOP).contains(&src_va) {
        return KError::Invalid.into();
    }

    let e = envid2env(envid, false);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();
    if !(*(*e).data).ipc_data.receiving {
        return KError::IpcNotRecv.into();
    }

    (*(*e).data).ipc_data.value = value;
    (*(*e).data).ipc_data.from_id = (*(*CUR_ENV).data).id;
    (*(*e).data).ipc_data.perm = perm | PTE_V;
    (*(*e).data).ipc_data.receiving = false;

    (*(*e).data).status = EnvStatus::Runnable;
    ENV_SCHE_LIST.insert_tail(e);

    if src_va != 0 {
        let r = page_lookup((*(*CUR_ENV).data).pgdir, src_va).ok_or(KError::Invalid);
        if let Err(e) = r {
            return e.into();
        }
        let (p, _) = r.unwrap();
        if let Err(e) = page_insert(
            (*(*e).data).pgdir,
            (*(*e).data).ipc_data.dstva as usize,
            (*(*e).data).asid,
            perm,
            p,
        ) {
            return e.into();
        }
    }

    0
}

unsafe fn sys_ipc_recv(dst_va: u32) -> u32 {
    let dst_va = dst_va as usize;
    if dst_va != 0 && !(UTEMP..UTOP).contains(&dst_va) {
        return KError::Invalid.into();
    }

    (*(*CUR_ENV).data).ipc_data.receiving = true;
    (*(*CUR_ENV).data).ipc_data.dstva = dst_va as u32;
    (*(*CUR_ENV).data).status = EnvStatus::NotRunnable;
    ENV_SCHE_LIST.remove(CUR_ENV);

    (*(KSTACKTOP as *mut TrapFrame).sub(1)).regs[2] = 0;
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
pub unsafe fn do_syscall(trapframe: *mut TrapFrame) {
    let sysno = (*trapframe).regs[4];
    if !(0..MAX_SYS_NO as u32).contains(&sysno) {
        (*trapframe).regs[2] = KError::NoSys as u32;
    }
    (*trapframe).cp0_epc += size_of::<u32>() as u32;

    let func = core::mem::transmute::<SyscallRawPtr, SyscallFn>(SYSCALL_TABLE[sysno as usize]);
    let arg1 = (*trapframe).regs[5];
    let arg2 = (*trapframe).regs[6];
    let arg3 = (*trapframe).regs[7];
    let arg4 = ((*trapframe).regs[29] as *const u32).add(4).read();
    let arg5 = ((*trapframe).regs[29] as *const u32).add(5).read();

    (*trapframe).regs[2] = func(arg1, arg2, arg3, arg4, arg5);
}
