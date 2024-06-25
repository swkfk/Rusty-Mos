use crate::{kernel_tests::slash_print, memory::shared_pool::MemoryPool, println};

use super::TEST_FN;

TEST_FN!({
    let mut memory_pool = MemoryPool::new();
    let id = memory_pool.crate_pool(1);
    memory_pool.bind(id, 1).unwrap();
    assert!(memory_pool.insert_page(id - 1, 1).is_err());
    for page in 1..100 {
        memory_pool.insert_page(id, page).unwrap();
    }
    memory_pool.fork_bind(2, 1); // 2 forked from 1
    memory_pool.fork_bind(3, 2); // 3 forked from 2

    memory_pool.destory_env(1);

    memory_pool.insert_page(id, 101).unwrap();
    memory_pool.destory_env(3);
    memory_pool.destory_env(2);

    // The id is now deleted
    assert!(memory_pool.insert_page(id, 102).is_err());

    println!("Delete pool auto is Okay.");

    let id = memory_pool.crate_pool(1);
    memory_pool.bind(id, 1).unwrap();
    assert!(memory_pool.bind(id, 1).is_err());

    println!("Double bind block is Okay.");

    for page in 1..1000 {
        memory_pool.insert_page(id, page).unwrap();
    }

    memory_pool.destory_env(1);
    assert!(memory_pool.insert_page(id, 10200).is_err());
    assert!(memory_pool.bind(id, 2).is_err());

    println!("Non-exist pool check is Okay.");

    let id = memory_pool.crate_pool(1);
    memory_pool.bind(id, 1).unwrap();
    assert!(memory_pool.lock(id, 2).is_err());
    assert!(memory_pool.lock(id, 1).unwrap()); // Lock is Okay.
    assert!(!memory_pool.lock(id, 1).unwrap());

    memory_pool.bind(id, 2).unwrap();
    assert!(!memory_pool.lock(id, 2).unwrap());
    memory_pool.unlock(id, 1).unwrap();
    assert!(memory_pool.lock(id, 2).unwrap());

    println!("Pool lock & unlock is Okay.");

    assert!(!memory_pool.lock(id, 1).unwrap());
    memory_pool.destory_env(2); // Destory the lock
    assert!(memory_pool.lock(id, 1).unwrap());

    println!("Lock destory is Okay.");

    slash_print("Passed!");
    println!();
});
