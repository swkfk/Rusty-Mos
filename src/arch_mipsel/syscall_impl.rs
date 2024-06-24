//! Syscall implementations.

use core::{
    fmt::{Display, Write},
    mem::{self, size_of},
    sync::atomic::Ordering::SeqCst,
};

use crate::{
    consts::error::KError,
    debugln,
    memory::{
        pmap::{page_alloc, page_insert, page_lookup, page_remove, PageNode, PAGES},
        regions::{KSTACKTOP, PAGE_SIZE, PTE_V, UTEMP, UTOP},
        shared_pool::MEMORY_POOL,
    },
    process::envs::{
        env_alloc, env_destory, envid2env, EnvStatus, CUR_ENV_IDX, ENVS_DATA, ENV_SCHE_LIST,
    },
    utils::io::{ioread_into_va, iowrite_from_va},
};

use super::{
    machine::{print_charc, scan_charc},
    syscall::MAX_SYS_NO,
    trap::TrapFrame,
};

use crate::process::scheduler::schedule;

/// SYSNO: 0, just show a character on the console.
///
/// For this implementation, it will call the [print_charc] function directly.
fn sys_putchar(ch: u8) {
    print_charc(ch);
}

/// SYSNO: 1, show one `num`-lengthed string on the console.
///
/// It will call the [print_charc] function one by one to put each characters.
///
/// The string *SHALL* be under the [UTOP] wholy.
///
/// # Failure
///
/// This syscall will return a negated-[KError::Invalid] if any of the string
/// is above the [UTOP].
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

/// SYSNO: 2, get the [EnvData::id](crate::process::envs::EnvData::id) or the
/// **pid**. (a.k.a in Linux) of the *current* env.
fn sys_getenvid() -> u32 {
    ENVS_DATA.borrow().0[CUR_ENV_IDX.load(SeqCst)].id
}

/// SYSNO: 3, give out the CPU, re-[schedule]. *NO-RETURN*
fn sys_yield() -> ! {
    schedule(true)
}

/// SYSNO: 4, destory a specified env with the
/// [id](crate::process::envs::EnvData::id) of `envid`.
///
/// Only the *current* env or the *child* of the current env can be destoried.
///
/// # Failure
///
/// This syscall will return a negated-[KError] returned by [envid2env].
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

/// SYSNO: 5, set the specified env's
/// [user_tlb_mod_entry](crate::process::envs::EnvData::user_tlb_mod_entry).
///
/// Only the *current* env's or the *child* of the current env's entry can be set.
///
/// # Failure
///
/// This syscall will return a negated-[KError] returned by [envid2env].
fn sys_set_tlb_mod_entry(envid: u32, func: u32) -> u32 {
    let e = envid2env(envid, true);
    if let Err(e) = e {
        return e.into();
    }
    let e = e.unwrap();
    ENVS_DATA.borrow_mut().0[e].user_tlb_mod_entry = func;
    0
}

/// SYSNO: 6, alloc a page and map it to `va`.
///
/// The `perm` is used when insert the page [alloced](page_alloc) into the
/// env's page table.
///
/// Only the *current* env or the *child* of the current env can be allocated.
///
/// If `va` is already mapped, that original page is sliently unmapped.
///
/// # Failure
///
/// This syscall will return a negated-[KError] returned by [envid2env],
/// [page_alloc] and [page_insert].
///
/// For example, the `envid` is not allowed or the pages are ran out.
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

/// SYSNO: 7, map a page from one env (`src_id`) to another (`dst_id`).
///
/// Get the page with the virtual address (`src_va`) in the env (`src_id`), and
/// map it into the virtual address (`dst_va`) in the env (`dst_id`).
///
/// Both the source env and the destination env *shall* be the *current* env or
/// the *child* of the current env.
///
/// See Also: [sys_mem_unmap]
///
/// # Failure
///
/// This syscall will return a negated-[KError] returned by [envid2env],
/// [page_lookup] and [page_insert].
///
/// This syscall will return a negated-[KError::Invalid] if any of the virtual
/// address is above the [UTOP] or below the [UTEMP].
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

/// SYSNO: 8, cancel the map of the specified virtual address for the specified
/// env.
///
/// The env *shall* be the *current* env or the *child* of the current env.
///
/// # Failure
///
/// This syscall will return a negated-[KError] returned by [envid2env] and
/// [page_remove].
///
/// This syscall will return a negated-[KError::Invalid] if the virtual address
/// is above the [UTOP] or below the [UTEMP].
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

/// SYSNO: 9, fork a new env which will be the *child* of the *current* env.
///
/// For the child, the priority will be same with its parent and the trap frame
/// also. The ruturn value will be set to *zero* to mark the child.
///
/// # Failure
///
/// This syscall will return a negated-[KError] returned by [env_alloc].
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

/// SYSNO: 10, set the run status of the specified env.
///
/// The `status` can only be [EnvStatus::Runnable] or [EnvStatus::NotRunnable].
/// The env will be added to or removed from the [ENV_SCHE_LIST] to be or be
/// not scheduled.
///
/// The env *shall* be the *current* env or the *child* of the current env.
///
/// # Failure
///
/// This syscall will return a negated-[KError::Invalid] if the `status` is
/// not allowed.
///
/// This syscall will return a negated-[KError] returned by [envid2env].
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

/// SYSNO: 11, set the trapframe of the specified env.
///
/// The env *shall* be the *current* env or the *child* of the current env.
///
/// The raw pointer `trapframe` *shall* be in the \[[UTEMP], [UTOP]) range.
///
/// # Failure
///
/// This syscall will return a negated-[KError::Invalid] if the `trapframe` is
/// not out of the required range.
///
/// This syscall will return a negated-[KError] returned by [envid2env].
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

/// A wrapper for the `*const u8`.
///
/// This struct is used to formatted-print the C-Style string. (A.k.a
/// *zero-terminated* string).
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

/// SYSNO: 12, trigger the kernel's panic with the given C-Style string.
///
/// # Panic
///
/// This syscall will *panic* in any situation.
fn sys_panic(msg: *const u8) -> ! {
    panic!("{}", CLikeStr(msg));
}

/// SYSNO: 13, Try to send a ipc-message to the target env.
///
/// The message will be a value together with a page if the `src_va` is *not*
/// zero. And the page will be inserted into the `envid`'s `dst_va`. The
/// value will be written to the target env's PCB.
///
/// # Failure
///
/// This syscall will return a negated-[KError::Invalid] if the `src_va` is
/// not zero and is not in the \[[UTEMP, UTOP]) range.
///
/// This syscall will return a negated-[KError::IpcNotRecv] if the `envid` is
/// not ready for receiving.
///
/// This syscall will return a negated-[KError] returned by [envid2env],
/// [page_lookup] and [page_insert].
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

/// SYSNO: 14, wait for a ipc-message.
///
/// The message will be a value together with a page if the `dst_va` is not
/// *zero*.
///
/// The current env will be *blocked*.
///
/// # Failure
///
/// This syscall will return a negated-[KError::Invalid] if the `dst_va` is
/// not zero and is not in the \[[UTEMP, UTOP]) range.
///
/// This syscall will not return if everything goes smooth.
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

/// SYSNO: 15, read a char from the console via [scan_charc].
///
/// **ATTENTION**! Kernel does busy waiting here and all the envs will be
/// blocked.
fn sys_cgetc() -> u8 {
    loop {
        let ch = scan_charc();
        if ch != 0 {
            return ch;
        }
    }
}

/// SYSNO: 16, write data at `va` into `pa` with the length of `len`.
///
/// The `len` can only be in `1`, `2` or `4`.
///
/// All the valid devices and their physical address ranges are as follows:
///
/// |  device  | start address | length |
/// |:--------:|:-------------:|:------:|
/// | console  |  0x180003f8   |  0x20  |
/// | IDE disk |  0x180001f0   |  0x8   |
///
/// See Also: [sys_read_dev]
///
/// # Failure
///
/// This syscall will return a negated-[KError::Invalid] if the `va` is out of
/// the \[[UTEMP, UTOP]) range, or the `pa` is invalid or the `len` is invalid.

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

/// SYSNO: 17, read data from `pa` into `va` with the length of `len`.
///
/// The `len` can only be in `1`, `2` or `4`.
///
/// The `pa` is the same as that in [sys_write_dev].
///
/// See Also: [sys_write_dev]
///
/// # Failure
///
/// This syscall will return a negated-[KError::Invalid] if the `va` is out of
/// the \[[UTEMP, UTOP]) range, or the `pa` is invalid or the `len` is invalid.
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

fn sys_create_shared_pool(va: u32, len: u32, perm: u32) -> u32 {
    let va = va as usize;
    let len = len as usize;
    if va.checked_add(len).is_none() || va < UTEMP || va + len > UTOP {
        return KError::Invalid.into();
    }
    if va & (PAGE_SIZE - 1) != 0 {
        return KError::Invalid.into();
    }
    let page_count = len.div_ceil(PAGE_SIZE);
    let pool_id = MEMORY_POOL
        .borrow_mut()
        .crate_pool(ENVS_DATA.borrow().0[CUR_ENV_IDX.load(SeqCst)].id as usize);

    for i in 0..page_count {
        let pp = page_alloc();
        if let Err(e) = pp {
            return e.into();
        }
        let pp = pp.unwrap();

        if let Err(r) = MEMORY_POOL.borrow_mut().insert_page(
            pool_id,
            (pp as usize - *PAGES.borrow() as usize) / mem::size_of::<PageNode>(),
        ) {
            return r.into();
        }

        if let Err(r) = page_insert(
            ENVS_DATA.borrow().0[CUR_ENV_IDX.load(SeqCst)].pgdir,
            va + i * PAGE_SIZE,
            ENVS_DATA.borrow().0[CUR_ENV_IDX.load(SeqCst)].asid,
            perm,
            pp,
        ) {
            return r.into();
        }
    }

    if let Err(r) = MEMORY_POOL
        .borrow_mut()
        .bind(pool_id, CUR_ENV_IDX.load(SeqCst))
    {
        return r.into();
    }

    pool_id as u32
}

fn sys_bind_shared_pool(va: u32, id: u32, perm: u32) -> u32 {
    let va = va as usize;
    match MEMORY_POOL.borrow_mut().bind(
        id as usize,
        ENVS_DATA.borrow().0[CUR_ENV_IDX.load(SeqCst)].id as usize,
    ) {
        Err(e) => e.into(),
        Ok(pages) => {
            for (i, page) in pages.iter().enumerate() {
                let pp = PAGES.borrow().wrapping_add(*page);
                if let Err(r) = page_insert(
                    ENVS_DATA.borrow().0[CUR_ENV_IDX.load(SeqCst)].pgdir,
                    va + i * PAGE_SIZE,
                    ENVS_DATA.borrow().0[CUR_ENV_IDX.load(SeqCst)].asid,
                    perm,
                    pp,
                ) {
                    return r.into();
                }
            }
            0
        }
    }
}

/// Just a type used in the [SYSCALL_TABLE]. A *holder*.
type SyscallRawPtr = *const ();
/// The real syscall function type.
///
/// It takes 5 32-bit arguments and return a 32-bit integer. Normally, a return
/// value of a negative number marks something bad happened.
///
/// **ATTENTION**! Some of the functions are *no-return*.
type SyscallFn = fn(u32, u32, u32, u32, u32) -> u32;

/// Syscall function table. Indexed with the syscall number.
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
    sys_create_shared_pool as SyscallRawPtr,
    sys_bind_shared_pool as SyscallRawPtr,
];

/// Get the syscall number and all the five arguments. Invoke the syscall.
///
/// The fourth and fifth argument is loaded from the stack. And the return
/// value will be stored into the `$v0`.
///
/// If the syscall number is invalid (out of the range([MAX_SYS_NO])), the
/// return value will be *set* as negated-[KError::NoSys];
#[no_mangle]
pub fn do_syscall(trapframe: *mut TrapFrame) {
    let tf_p = trapframe; // deceits
    let tf = unsafe { *tf_p };
    let sysno = tf.regs[4];
    if !(0..MAX_SYS_NO as u32).contains(&sysno) {
        unsafe { (*tf_p).regs[2] = KError::NoSys.into() }
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
