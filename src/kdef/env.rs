//! Define the basic structure used in the env module. As well as their basic
//! constructor use by the static initialization.

use core::ptr::null_mut;

use crate::kern::trap::TrapFrame;
use crate::memory::pmap::Pde;
use crate::utils::linked_list::{LinkList, LinkNode, TailLinkList};

/// The env status enum. Compatible with the C-Like memory structure.
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum EnvStatus {
    #[default]
    /// The env is not used (free).
    Free = 0,
    /// The env is running or to be run.
    Runnable,
    /// The env is blocked.
    NotRunnable,
}

/// The IPC data collected together. Compatible with the C-Like memory
/// structure.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IpcData {
    /// The value passed directly.
    pub value: u32,
    /// The sender's env id.
    pub from_id: u32,
    /// Mark this env's receiving status.
    pub receiving: bool,
    /// The target virtual address.
    pub dstva: u32,
    /// The page permission.
    pub perm: u32,
}

impl IpcData {
    /// Used for the static construction. All members are filled with zero.
    pub const fn const_construct() -> Self {
        IpcData {
            value: 0,
            from_id: 0,
            receiving: false,
            dstva: 0,
            perm: 0,
        }
    }
}

/// The PCB struct. Compatible with the C-Like memory structure and the MOS.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EnvData {
    /// Trap Frame stored in the PCB.
    pub trap_frame: TrapFrame,
    /// Unused placeholder item.
    _place_holder_env_link: [u32; 2],
    /// The env id.
    pub id: u32,
    /// The asid for TLB.
    pub asid: u32,
    /// The env's parent env's id;
    pub parent_id: u32,
    /// The running status of this env.
    pub status: EnvStatus,
    /// The page directory address of this env.
    pub pgdir: *mut Pde,
    /// Unused placeholder item.
    _place_holder_env_sched_link: [u32; 2],
    /// The priority of this env.
    pub priority: u32,
    /// The IPC data collected.
    pub ipc_data: IpcData,
    /// The entry of the tlb mod handler in user space.
    pub user_tlb_mod_entry: u32,
    /// Used in Lab 6. ///
    pub env_runs: u32,
}

impl Default for EnvData {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvData {
    /// Used for the static construction. All members are filled with zero.
    pub const fn new() -> Self {
        Self {
            trap_frame: TrapFrame::const_construct(),
            id: 0,
            asid: 0,
            pgdir: null_mut(),
            parent_id: 0,
            status: EnvStatus::Free,
            priority: 0,
            ipc_data: IpcData::const_construct(),
            user_tlb_mod_entry: 0,
            env_runs: 0,
            _place_holder_env_link: [0, 0],
            _place_holder_env_sched_link: [0, 0],
        }
    }
}

/// The env list used only in the *kernel* mode. For free list.
pub type EnvList = LinkList<*mut EnvData>;
/// The env tailq list used only in the *kernel* mode. For sched list.
pub type EnvTailList = TailLinkList<*mut EnvData>;
/// Env link node.
pub type EnvNode = LinkNode<*mut EnvData>;

impl EnvNode {
    /// Used for the static construction. All members are filled with zero.
    pub const fn const_construct() -> Self {
        Self {
            next: core::ptr::null_mut(),
            prev: core::ptr::null_mut(),
            data: core::ptr::null_mut(),
        }
    }
}

/// The log of [NENV].
pub const LOG2NENV: u8 = 10;
/// The count of the envs.
pub const NENV: usize = 1 << LOG2NENV;

/// Spawn the index of the given `envid`.
#[macro_export]
macro_rules! ENVX {
    ($envid: expr) => {
        $envid & ($crate::kdef::env::NENV - 1)
    };
}
