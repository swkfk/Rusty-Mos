/// Error Codes Enum
/// ## Error Types
/// - Error#1 (`Unspecified`) to Error#7 (`IpcNotRecv`) is for the *operating system*
/// - Error#8 (`NoDisk`) to Error#13 (`NotExec`) is for the *file system*
/// ## Visibility
/// Error Codes for the *file system* is only seen in user-level
#[repr(i8)]
pub enum Error {
    Unspecified = 1,
    BadEnv,
    Invalid,
    NoMem,
    NoSys,
    NoFreeEnv,
    IpcNotRecv,
    NoDisk,
    MaxOpen,
    NotFound,
    BadPath,
    FileExists,
    NotExec,
}
