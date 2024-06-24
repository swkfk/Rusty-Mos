extern crate alloc;

use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::SeqCst;

use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;

use crate::consts::error::KError;
use crate::debugln;
use crate::process::envs::{LOG2NENV, NENV};
use crate::utils::sync_ref_cell::SyncImplRef;

struct MemoryPoolEntry {
    pages: Vec<usize>,
    envs: Vec<usize>,
    reference: usize,
    write_lock: AtomicUsize,
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
            write_lock: AtomicUsize::new(0),
        }
    }

    fn deref(&mut self, envid: usize) -> Result<bool, KError> {
        if !self.envs.contains(&envid) {
            Err(KError::PoolNotBind)
        } else {
            self.reference -= 1;
            let _ = self.write_lock.compare_exchange(envid, 0, SeqCst, SeqCst);
            Ok(self.reference == 0)
        }
    }

    pub fn lock(&mut self, envid: usize) -> bool {
        self.write_lock.compare_exchange(0, envid, SeqCst, SeqCst) == Ok(0)
    }

    pub fn unlock(&mut self, envid: usize) -> Result<(), KError> {
        let ret = self.write_lock.compare_exchange(envid, 0, SeqCst, SeqCst);
        if let Err(r) = ret {
            if r == 0 {
                return Err(KError::NoLock);
            } else {
                return Err(KError::LockByOthers);
            }
        }
        Ok(())
    }
}

pub struct MemoryPool {
    pools: BTreeMap<usize, MemoryPoolEntry>,
    envs: BTreeMap<usize, Vec<usize>>,
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
            envs: BTreeMap::new(),
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

    pub fn fork_bind(&mut self, child_id: usize, envid: usize) {
        debugln!("> POOL: forked pools from {} to {}...", envid, child_id);
        match self.envs.get_mut(&envid) {
            None => (),
            Some(v) => {
                let v = v.clone();
                for pool in v.iter() {
                    self.pools.get_mut(pool).unwrap().reference += 1;
                }
                self.envs.insert(child_id, v);
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
                    pool.reference += 1;
                    match self.envs.get_mut(&envid) {
                        None => {
                            debugln!("> POOL: new env {}...", envid);
                            let _ = self.envs.insert(envid, vec![poolid]);
                        }
                        Some(v) => v.push(poolid),
                    }
                    Ok(&pool.pages)
                }
            }
        }
    }

    fn unbind(&mut self, poolid: usize, envid: usize) -> Result<(), KError> {
        match self.pools.get_mut(&poolid) {
            None => Err(KError::PoolNotFound),
            Some(pool) => {
                if pool.deref(envid)? {
                    debugln!("> POOL: pool {} removed", poolid);
                    self.pools.remove(&poolid);
                }
                Ok(())
            }
        }
    }

    pub fn destory_env(&mut self, envid: usize) {
        match self.envs.get_mut(&envid) {
            None => (),
            Some(_) => {
                let v = self.envs.remove(&envid).unwrap();
                for poolid in v {
                    debugln!("> POOL: unbind {} from env {}", poolid, envid);
                    let _ = self.unbind(poolid, envid);
                }
            }
        }
    }

    pub fn lock(&mut self, poolid: usize, envid: usize) -> Result<bool, KError> {
        match self.pools.get_mut(&poolid) {
            None => Err(KError::PoolNotFound),
            Some(pool) => Ok(pool.lock(envid)),
        }
    }

    pub fn unlock(&mut self, poolid: usize, envid: usize) -> Result<(), KError> {
        match self.pools.get_mut(&poolid) {
            None => Err(KError::PoolNotFound),
            Some(pool) => pool.unlock(envid),
        }
    }
}

pub static MEMORY_POOL: SyncImplRef<MemoryPool> = SyncImplRef::new(MemoryPool::new());

static POOL_I: AtomicUsize = AtomicUsize::new(1);

fn mkpoolid(envid: usize) -> usize {
    (POOL_I.fetch_add(1, SeqCst) << (1 + LOG2NENV)) | (envid & (NENV - 1))
}
