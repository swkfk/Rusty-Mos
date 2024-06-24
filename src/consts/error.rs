//! Define the Error Code in mos.
//!
//! Include both the kernel error ([KError]) and the user error([UError]).

/// Error Codes Enum only for **the Kernel**
#[repr(i8)]
#[derive(Debug)]
pub enum KError {
    /// Unspecified or unknown problem
    Unspecified = 1,
    /// The environment does not exist or otherwise cannot be used in requestd action
    BadEnv,
    /// The parameter is invalid
    Invalid,
    /// Run out of memory
    NoMem,
    /// Invalid syscall number
    NoSys,
    /// The environment maximum count exceeded
    NoFreeEnv,
    /// Attempt to send to env that is not recving
    IpcNotRecv,
    /// Invalid memory pool id
    PoolNotFound = 14,
    /// An env try to bind to a pool twice
    PoolDoubleBind,
    /// The env is not binded to the pool
    PoolNotBind,
    /// Unlock a lock without being locked
    NoLock,
    /// Unlock a lock which was locked by another env
    LockByOthers,
}

impl From<KError> for u32 {
    fn from(val: KError) -> Self {
        -(val as i32) as u32
    }
}

/// Error Codes Enum only for **the User's File System**
pub enum UError {
    /// No free space left on disk
    NoDisk = 8,
    /// The maximum count of opened file exceeded
    MaxOpen,
    /// File or block not found
    NotFound,
    /// Bad path
    BadPath,
    /// File already exisits
    FileExists,
    /// File is not a valid executable
    NotExec,
}
