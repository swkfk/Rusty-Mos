use crate::kdef::{env::EnvStatus, error::KError, syscall::MAX_SYS_NO};

use super::{env::CUR_ENV, machine::print_charc, sched::schedule, trap::TrapFrame};

type PureResult = Result<(), KError>;

fn sys_putchar(ch: u8) {
    print_charc(ch);
}

fn sys_print_cons(s: *const u8, num: u32) -> PureResult {
    unimplemented!()
}

fn sys_getenvid() -> u32 {
    unsafe { (*CUR_ENV).data.id }
}

fn sys_yield() -> ! {
    unsafe { schedule(true) }
}

fn sys_env_destroy(envid: u32) -> PureResult {
    unimplemented!()
}

fn sys_set_tlb_mod_entry(envid: u32, func: u32) -> PureResult {
    unimplemented!()
}

fn sys_mem_alloc(envid: u32, va: u32, perm: u32) -> PureResult {
    unimplemented!()
}

fn sys_mem_map(src_id: u32, src_va: u32, dst_id: u32, dst_va: u32, perm: u32) -> PureResult {
    unimplemented!()
}

fn sys_mem_unmap(envid: u32, va: u32) -> PureResult {
    unimplemented!()
}

fn sys_exofork() -> Result<u32, KError> {
    unimplemented!()
}

fn sys_set_env_status(envid: u32, status: EnvStatus) -> PureResult {
    unimplemented!()
}

fn sys_set_trapframe(envid: u32, trapframe: *mut TrapFrame) -> PureResult {
    unimplemented!()
}

fn sys_panic(msg: *const u8) -> ! {
    unimplemented!()
}

fn sys_ipc_try_send(envid: u32, value: u32, src_va: u32, perm: u32) -> PureResult {
    unimplemented!()
}

fn sys_ipc_recv(dst_va: u32) -> PureResult {
    unimplemented!()
}

fn sys_cgetc() -> u8 {
    unimplemented!()
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
