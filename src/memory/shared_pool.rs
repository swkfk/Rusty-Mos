//! Support the shared momeory pool. We can share more than one pages between
//! envs.

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

/// Memory Pool Entry. Contains the pages it shared and the envs attatched to
/// it. The entry also contains a lock field.
struct MemoryPoolEntry {
    /// List of page *indexes* shared by this pool.
    pages: Vec<usize>,
    /// List of envs bind to this pool.
    envs: Vec<usize>,
    /// The reference count of this pool. Destroy the pool is the reference
    /// goes *zero*.
    reference: usize,
    /// Write lock, store the env-id which locked it. Or *zero* means unlocked.
    write_lock: AtomicUsize,
}

impl Default for MemoryPoolEntry {
    /// Default comstructions.
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryPoolEntry {
    /// Create a new pool.
    pub const fn new() -> Self {
        Self {
            reference: 0,
            pages: Vec::new(),
            envs: Vec::new(),
            write_lock: AtomicUsize::new(0),
        }
    }

    /// Decrease the reference of the pool. If the `reference` is minused to
    /// *zero*, an `Ok(true)` will be returned.
    fn deref(&mut self, envid: usize) -> Result<bool, KError> {
        if !self.envs.contains(&envid) {
            Err(KError::PoolNotBind)
        } else {
            self.reference -= 1;
            let _ = self.write_lock.compare_exchange(envid, 0, SeqCst, SeqCst);
            Ok(self.reference == 0)
        }
    }

    /// Try to lock the pool. If locked successfully, the return value is
    /// `true`.
    pub fn lock(&mut self, envid: usize) -> bool {
        self.write_lock.compare_exchange(0, envid, SeqCst, SeqCst) == Ok(0)
    }

    /// Unlock the pool. Only the env who locked it can unlock it.
    ///
    /// # Return
    ///
    /// If unlocked successfully, an `Ok(())` is returned.
    ///
    /// Otherwise, a [KError] with a `Err` wrapper is returned:
    /// - `Err(KError::NoLock)`: The pool was not locked.
    /// - `Err(KError::LockByOthers)`: The pool was locked by another env.
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

/// Memory pool manager. Support every operation that is needed to perform.
pub struct MemoryPool {
    /// The memory pools. Map from pool_id to [MemoryPoolEntry].
    pools: BTreeMap<usize, MemoryPoolEntry>,
    envs: BTreeMap<usize, Vec<usize>>,
}

impl Default for MemoryPool {
    /// Default constructions
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryPool {
    /// Create a new pool manager.
    pub const fn new() -> Self {
        Self {
            pools: BTreeMap::new(),
            envs: BTreeMap::new(),
        }
    }

    /// Get a new pool id and create a new pool entry.
    pub fn crate_pool(&mut self, envid: usize) -> usize {
        let id = mkpoolid(envid);
        self.pools.insert(id, MemoryPoolEntry::new());
        id
    }

    /// Insert a page into a pool. If the pool is not found, a
    /// `Err(KError::PoolNotFound)` will be returned.
    pub fn insert_page(&mut self, poolid: usize, pageid: usize) -> Result<(), KError> {
        match self.pools.get_mut(&poolid) {
            None => Err(KError::PoolNotFound),
            Some(pool) => {
                pool.pages.push(pageid);
                Ok(())
            }
        }
    }

    /// Bind the forked child env into the parent env's pools.
    ///
    /// Since when forked, the parent's pages will be dupped to the child's, so
    /// we need to fork the pools meanwhile. The `reference` of each pool will
    /// be increased.
    pub fn fork_bind(&mut self, child_id: usize, envid: usize) {
        debugln!("> POOL: forked pools from {} to {}...", envid, child_id);
        match self.envs.get_mut(&envid) {
            None => (),
            Some(v) => {
                let v = v.clone();
                for pool in v.iter() {
                    self.pools.get_mut(pool).unwrap().reference += 1;
                    self.pools.get_mut(pool).unwrap().envs.push(child_id);
                }
                self.envs.insert(child_id, v);
            }
        }
    }

    /// Bind bind an env to a pool. If the pool does not exist or the env has
    /// been bind to the pool, This method will fail.
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

    /// Unbind an env from a pool. If unbind successfully, the pool will get a
    /// decrease-reference.
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

    /// Unbind all the pools bind to the env. Called by
    /// [env_free](crate::process::envs::env_free).
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

    /// Lock the pool with `poolid`, and the locker will be `envid`.
    pub fn lock(&mut self, poolid: usize, envid: usize) -> Result<bool, KError> {
        match self.pools.get_mut(&poolid) {
            None => Err(KError::PoolNotFound),
            Some(pool) => {
                if !pool.envs.contains(&envid) {
                    Err(KError::PoolNotBind)
                } else {
                    Ok(pool.lock(envid))
                }
            }
        }
    }

    /// Unlock the pool with `poolid`, and the locker will be `envid`.
    pub fn unlock(&mut self, poolid: usize, envid: usize) -> Result<(), KError> {
        match self.pools.get_mut(&poolid) {
            None => Err(KError::PoolNotFound),
            Some(pool) => pool.unlock(envid),
        }
    }
}

/// Global shared memory pool.
pub static MEMORY_POOL: SyncImplRef<MemoryPool> = SyncImplRef::new(MemoryPool::new());

/// Used to spawn the pool id. Increase one-by-one when doing [mkpoolid].
static POOL_I: AtomicUsize = AtomicUsize::new(1);

/// Spawn the unique id of a new pool.
fn mkpoolid(envid: usize) -> usize {
    (POOL_I.fetch_add(1, SeqCst) << (1 + LOG2NENV)) | (envid & (NENV - 1))
}
