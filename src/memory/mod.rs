//! Memory management. Including the page-memory model and the TLB related
//! handlers. A global allocator will be provided as well.

pub mod buddy_allocator;
pub mod marcos;
pub mod pmap;
pub mod regions;
pub mod shared_pool;
pub mod tlbex;
