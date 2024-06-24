//! The syscall number enum definition.

/// Syscall identifier. The number is ordered by enum automatically.
#[repr(u8)]
pub enum SyscallNo {
    /// Print a char into the console.
    SysPutchar = 0,
    /// Print a straight string ends with zero into the console.
    SysPrintCons,
    /// Get the id of the current env.
    SysGetenvid,
    /// Give out the CPU time and re-schedule.
    SysYield,
    /// Destory a env by its id and kill it.
    SysEnvDestroy,
    /// Register the user-space TLB mod handler for the specified env.
    SysSetTlbModEntry,
    /// Allocate memory and map it.
    SysMemAlloc,
    /// Map the virtual address to a specified physical page.
    SysMemMap,
    /// Unmap the address and the page.
    SysMemUnmap,
    /// Allocate a new env and make it child of the current env.
    SysExofork,
    /// Set the env status and move it between the lists.
    SysSetEnvStatus,
    /// Set the trapframe to he specified env.
    SysSetTrapframe,
    /// Do kernel panic.
    SysPanic,
    /// Try to send an ipc data to a env.
    SysIpcTrySend,
    /// Receive a ipc data or wait.
    SysIpcRecv,
    /// Scan a char from the console
    SysCgetc,
    /// Write to a dev.
    SysWriteDev,
    /// Read from a dev.
    SysReadDev,
    /// Create a memory pool.
    SysCreatePool,
    /// Bind a memory pool.
    SysFetchPool,
    /// Try to lock a memory pool.
    SysTryLock,
    /// Unlock a memory pool.
    SysUnlock,
}

/// The count of the syscalls. It should be updated manually.
pub const MAX_SYS_NO: usize = 22;
