use core::{
    fmt::{Display, Write},
    mem::size_of,
};

use crate::{
    kdef::{
        env::EnvStatus,
        error::KError,
        mmu::{KSTACKTOP, PTE_V, UTEMP, UTOP},
        syscall::MAX_SYS_NO,
    },
    kern::env::env_destory,
    println,
};

use super::{
    env::{env_alloc, envid2env, CUR_ENV, ENV_SCHE_LIST},
    machine::{print_charc, scan_charc},
    pmap::{page_alloc, page_insert, page_lookup, page_remove},
    sched::schedule,
    trap::TrapFrame,
};

type PureResult = Result<(), KError>;

fn sys_putchar(ch: u8) {
    print_charc(ch);
}

unsafe fn sys_print_cons(s: *const u8, num: u32) -> PureResult {
    let num = num as usize;
    if s as usize + num > UTOP || s as usize > UTOP || s as usize > s as usize + num {
        return Err(KError::Invalid);
    }
    for i in 0..num {
        print_charc(*(s.add(i)));
    }
    Ok(())
}

fn sys_getenvid() -> u32 {
    unsafe { (*CUR_ENV).data.id }
}

fn sys_yield() -> ! {
    unsafe { schedule(true) }
}

unsafe fn sys_env_destroy(envid: u32) -> PureResult {
    let e = envid2env(envid, true)?;
    println!("% {}: Destorying {}", (*CUR_ENV).data.id, (*e).data.id);
    env_destory(e);
    Ok(())
}

unsafe fn sys_set_tlb_mod_entry(envid: u32, func: u32) -> PureResult {
    let e = envid2env(envid, true)?;
    (*e).data.user_tlb_mod_entry = func;
    Ok(())
}

unsafe fn sys_mem_alloc(envid: u32, va: u32, perm: u32) -> PureResult {
    let va = va as usize;
    if !(UTEMP..UTOP).contains(&va) {
        return Err(KError::Invalid);
    }
    let e = envid2env(envid, true)?;
    let pp = page_alloc()?;
    page_insert((*e).data.pgdir, va, (*e).data.asid, perm, pp)
}

unsafe fn sys_mem_map(src_id: u32, src_va: u32, dst_id: u32, dst_va: u32, perm: u32) -> PureResult {
    let src_va = src_va as usize;
    let dst_va = dst_va as usize;
    if !(UTEMP..UTOP).contains(&src_va) || !(UTEMP..UTOP).contains(&dst_va) {
        return Err(KError::Invalid);
    }

    let src_env = envid2env(src_id, true)?;
    let dst_env = envid2env(dst_id, true)?;
    let (pp, _) = page_lookup((*src_env).data.pgdir, src_va).ok_or(KError::Invalid)?;

    page_insert(
        (*dst_env).data.pgdir,
        dst_va,
        (*dst_env).data.asid,
        perm,
        pp,
    )
}

unsafe fn sys_mem_unmap(envid: u32, va: u32) -> PureResult {
    let va = va as usize;
    if !(UTEMP..UTOP).contains(&va) {
        return Err(KError::Invalid);
    }
    let e = envid2env(envid, true)?;
    page_remove((*e).data.pgdir, va, (*e).data.asid);
    Ok(())
}

unsafe fn sys_exofork() -> Result<u32, KError> {
    let e = env_alloc((*CUR_ENV).data.id)?;
    (*e).data.trap_frame = *((KSTACKTOP as *mut TrapFrame).sub(1));
    (*e).data.trap_frame.regs[2] = 0;
    (*e).data.status = EnvStatus::NotRunnable;
    (*e).data.priority = (*CUR_ENV).data.priority;
    Ok((*e).data.id)
}

unsafe fn sys_set_env_status(envid: u32, status: EnvStatus) -> PureResult {
    if status != EnvStatus::NotRunnable && status != EnvStatus::Runnable {
        return Err(KError::Invalid);
    }
    let e = envid2env(envid, true)?;

    if (*e).data.status == status {
        return Ok(());
    }

    if status == EnvStatus::Runnable {
        ENV_SCHE_LIST.insert_tail(e);
    } else {
        ENV_SCHE_LIST.remove(e);
    }

    (*e).data.status = status;
    Ok(())
}

unsafe fn sys_set_trapframe(envid: u32, trapframe: *mut TrapFrame) -> Result<u32, KError> {
    let len = size_of::<TrapFrame>();
    let va = trapframe as usize;
    if va.checked_add(len).is_none() || va < UTEMP || va + len > UTOP {
        return Err(KError::Invalid);
    }
    let env = envid2env(envid, true)?;

    if env == CUR_ENV {
        ((KSTACKTOP as *mut TrapFrame).sub(1)).write(*trapframe);
        Ok((*trapframe).regs[2])
    } else {
        (*env).data.trap_frame = *trapframe;
        Ok(0)
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

unsafe fn sys_ipc_try_send(envid: u32, value: u32, src_va: u32, perm: u32) -> PureResult {
    let src_va = src_va as usize;
    if src_va != 0 && !(UTEMP..UTOP).contains(&src_va) {
        return Err(KError::Invalid);
    }

    let e = envid2env(envid, false)?;
    if !(*e).data.ipc_data.receiving {
        return Err(KError::IpcNotRecv);
    }

    (*e).data.ipc_data.value = value;
    (*e).data.ipc_data.from_id = (*CUR_ENV).data.id;
    (*e).data.ipc_data.perm = perm | PTE_V;
    (*e).data.ipc_data.receiving = false;

    (*e).data.status = EnvStatus::Runnable;
    ENV_SCHE_LIST.insert_head(e);

    if src_va != 0 {
        let (p, _) = page_lookup((*CUR_ENV).data.pgdir, src_va).ok_or(KError::Invalid)?;
        page_insert(
            (*e).data.pgdir,
            (*e).data.ipc_data.dstva as usize,
            (*e).data.asid,
            perm,
            p,
        )
    } else {
        Ok(())
    }
}

unsafe fn sys_ipc_recv(dst_va: u32) -> PureResult {
    let dst_va = dst_va as usize;
    if dst_va != 0 && !(UTEMP..UTOP).contains(&dst_va) {
        return Err(KError::Invalid);
    }

    (*CUR_ENV).data.ipc_data.receiving = true;
    (*CUR_ENV).data.ipc_data.dstva = dst_va as u32;
    (*CUR_ENV).data.status = EnvStatus::NotRunnable;
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

fn sys_write_dev(_va: u32, _pa: u32, _len: u32) -> PureResult {
    unimplemented!()
}

fn sys_read_dev(_va: u32, _pa: u32, _len: u32) -> PureResult {
    unimplemented!()
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
pub unsafe fn do_syscall(_trapframe: *mut TrapFrame) {
    core::mem::transmute::<SyscallRawPtr, SyscallFn>(SYSCALL_TABLE[0])(0, 0, 0, 0, 0);
}
