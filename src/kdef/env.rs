use core::ptr::null_mut;

use crate::kern::{pmap::Pde, trap::TrapFrame};

use super::queue::{LinkList, LinkNode, TailLinkList};

#[repr(i8)]
#[derive(Debug, Clone, Copy, Default)]
pub enum EnvStatus {
    #[default]
    Free = 0,
    Runnable,
    NotRunnable,
}

#[derive(Clone, Copy)]
pub struct IpcData {
    pub value: u32,
    pub from_id: u32,
    pub receiving: bool,
    pub dstva: u32,
    pub perm: u32,
}

impl IpcData {
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

#[derive(Clone, Copy)]
pub struct EnvData {
    pub trap_frame: TrapFrame,
    pub id: u32,
    pub asid: u32,
    pub pgdir: *mut Pde,
    pub parent_id: u32,
    pub status: EnvStatus,
    pub priority: u32,
    pub ipc_data: IpcData,
    pub user_tlb_mod_entry: u32,
    pub env_runs: u32,
}

impl EnvData {
    pub const fn const_construct() -> Self {
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
        }
    }
}

pub type EnvList = LinkList<EnvData>;
pub type EnvTailList = TailLinkList<EnvData>;
pub type EnvNode = LinkNode<EnvData>;

impl EnvNode {
    pub const fn const_construct() -> Self {
        Self {
            next: core::ptr::null_mut(),
            prev: core::ptr::null_mut(),
            data: EnvData::const_construct(),
        }
    }
}

pub const LOG2NENV: u8 = 10;
pub const NENV: usize = 1 << LOG2NENV;

#[macro_export]
macro_rules! ENVX {
    ($envid: expr) => {
        $envid & ($crate::kdef::env::NENV - 1)
    };
}
