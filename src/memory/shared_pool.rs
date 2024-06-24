extern crate alloc;

use core::sync::atomic::Ordering::SeqCst;
use core::sync::atomic::{AtomicBool, AtomicUsize};

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::consts::error::KError;
use crate::process::envs::{LOG2NENV, NENV};
use crate::utils::sync_ref_cell::SyncImplRef;

struct MemoryPoolEntry {
    pages: Vec<usize>,
    envs: Vec<usize>,
    reference: usize,
    write_lock: AtomicBool,
}

impl Default for MemoryPoolEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryPoolEntry {
    pub const fn new() -> Self {
        Self {
            reference: 0,
            pages: Vec::new(),
            envs: Vec::new(),
            write_lock: AtomicBool::new(false),
        }
    }

    pub fn deref(&mut self, envid: usize) -> Result<bool, KError> {
        if !self.envs.contains(&envid) {
            Err(KError::PoolNotBind)
        } else {
            self.reference -= 1;
            Ok(self.reference == 0)
        }
    }

    pub fn try_lock(&mut self) -> bool {
        self.write_lock.swap(true, SeqCst)
    }
}

pub struct MemoryPool {
    pools: BTreeMap<usize, MemoryPoolEntry>,
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryPool {
    pub const fn new() -> Self {
        Self {
            pools: BTreeMap::new(),
        }
    }

    pub fn crate_pool(&mut self, envid: usize) -> usize {
        let id = mkpoolid(envid);
        self.pools.insert(id, MemoryPoolEntry::new());
        id
    }

    pub fn insert_page(&mut self, poolid: usize, pageid: usize) -> Result<(), KError> {
        match self.pools.get_mut(&poolid) {
            None => Err(KError::PoolNotFound),
            Some(pool) => {
                pool.pages.push(pageid);
                Ok(())
            }
        }
    }

    pub fn bind(&mut self, poolid: usize, envid: usize) -> Result<&Vec<usize>, KError> {
        match self.pools.get_mut(&poolid) {
            None => Err(KError::PoolNotFound),
            Some(pool) => {
                if pool.envs.contains(&envid) {
                    Err(KError::PoolDoubleBind)
                } else {
                    pool.envs.push(envid);
                    Ok(&pool.pages)
                }
            }
        }
    }

    pub fn unbind(&mut self, poolid: usize, envid: usize) -> Result<(), KError> {
        match self.pools.get_mut(&poolid) {
            None => Err(KError::PoolNotFound),
            Some(pool) => {
                if pool.deref(envid)? {
                    self.pools.remove(&poolid);
                }
                Ok(())
            }
        }
    }

    pub fn try_lock(&mut self, poolid: usize) -> Result<bool, KError> {
        match self.pools.get_mut(&poolid) {
            None => Err(KError::PoolNotFound),
            Some(pool) => Ok(pool.try_lock()),
        }
    }
}

pub static MEMORY_POOL: SyncImplRef<MemoryPool> = SyncImplRef::new(MemoryPool::new());

static POOL_I: AtomicUsize = AtomicUsize::new(1);

fn mkpoolid(envid: usize) -> usize {
    POOL_I.fetch_add(1, SeqCst) << (1 + LOG2NENV) | (envid * (NENV - 1))
}
