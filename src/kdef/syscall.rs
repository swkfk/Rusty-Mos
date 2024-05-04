#[repr(u8)]
pub enum SyscallNo {
    SysPutchar = 0,
    SysPrintCons,
    SysGetenvid,
    SysYield,
    SysEnvDestroy,
    SysSetTlbModEntry,
    SysMemAlloc,
    SysMemMap,
    SysMemUnmap,
    SysExofork,
    SysSetEnvStatus,
    SysSetTrapframe,
    SysPanic,
    SysIpcTrySend,
    SysIpcRecv,
    SysCgetc,
    SysWriteDev,
    SysReadDev,
}

pub const MAX_SYS_NO: usize = 18;
