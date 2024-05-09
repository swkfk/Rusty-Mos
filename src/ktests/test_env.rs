#[cfg(ktest_item = "env")]
pub fn test_tailq() {
    use core::ptr::addr_of_mut;

    use crate::{
        kdef::queue::{LinkNode, TailLinkList},
        println,
    };

    let mut node1 = LinkNode::new(123);
    let mut node2 = LinkNode::new(234);
    let mut node3 = LinkNode::new(345);
    let mut node4 = LinkNode::new(456);
    let mut node5 = LinkNode::new(567);
    let mut node6 = LinkNode::new(678);

    let mut q1 = TailLinkList::<u32>::new();
    q1.enable();
    unsafe {
        q1.insert_head(addr_of_mut!(node1));
        q1.insert_head(addr_of_mut!(node2));
        q1.insert_head(addr_of_mut!(node3));
        q1.insert_head(addr_of_mut!(node4));
        assert_eq!((*q1.pop_head().unwrap()).data, 456);
        assert_eq!((*q1.pop_head().unwrap()).data, 345);
        assert_eq!((*q1.pop_head().unwrap()).data, 234);
        // Remain node1 only
        q1.insert_head(addr_of_mut!(node3));
        assert_eq!((*q1.pop_head().unwrap()).data, 345);
        assert_eq!((*q1.pop_head().unwrap()).data, 123);
        assert!(q1.empty());
    }
    println!("Insert head ana Pop head test over.");

    let mut q2 = TailLinkList::<u32>::new();
    q2.enable();
    unsafe {
        assert!(q2.pop_head().is_none());
        q2.insert_tail(addr_of_mut!(node5));
        q2.insert_tail(addr_of_mut!(node6));
        q2.insert_head(addr_of_mut!(node4));
        assert_eq!((*q2.pop_head().unwrap()).data, 456);
        assert_eq!((*q2.pop_head().unwrap()).data, 567);
        assert_eq!((*q2.pop_head().unwrap()).data, 678);
        assert!(q2.empty());
    }
    println!("Insert tail test over.");

    let mut q3 = TailLinkList::<u32>::new();
    q3.enable();
    unsafe {
        q3.insert_head(addr_of_mut!(node3));
        q3.remove(addr_of_mut!(node3));
        assert!(q3.empty());
        q3.insert_tail(addr_of_mut!(node3));
        q3.insert_tail(addr_of_mut!(node4));
        q3.remove(addr_of_mut!(node4));
        q3.insert_head(addr_of_mut!(node5));
        q3.remove(addr_of_mut!(node3));
        assert_eq!((*q3.pop_head().unwrap()).data, 567);
        assert!(q3.empty());
        q3.insert_head(addr_of_mut!(node4));
        q3.insert_tail(addr_of_mut!(node5));
        q3.insert_tail(addr_of_mut!(node6));
        q3.remove(addr_of_mut!(node5));
        assert_eq!((*q3.pop_head().unwrap()).data, 456);
        assert_eq!((*q3.pop_head().unwrap()).data, 678);
    }
    println!("Remove test over.");
}

#[cfg(ktest_item = "env")]
pub fn test_envs() {
    use core::{mem::size_of, ptr::addr_of};

    use crate::{
        kdef::{
            env::{EnvList, EnvNode, NENV},
            error::KError,
            mmu::{PAGE_SIZE, UENVS, UPAGES, UTOP},
        },
        kern::{
            env::{env_alloc, env_free, BASE_PGDIR, ENVS_DATA, ENV_FREE_LIST, ENV_SCHE_LIST},
            pmap::{PageNode, NPAGE, PAGES},
        },
        println, va2pa, PADDR, PDX,
    };

    let pe0;
    let pe1;
    let pe2;
    unsafe {
        pe0 = env_alloc(0).unwrap();
        pe1 = env_alloc(0).unwrap();
        pe2 = env_alloc(0).unwrap();
    }

    assert!(!pe0.is_null());
    assert!(!pe1.is_null() && pe0 != pe1);
    assert!(!pe2.is_null() && pe0 != pe2 && pe1 != pe2);

    unsafe {
        let zfl = ENV_FREE_LIST;
        ENV_FREE_LIST = EnvList::new();

        if let Err(KError::NoFreeEnv) = env_alloc(0) {
        } else {
            unreachable!()
        };

        ENV_FREE_LIST = zfl;
    }

    unsafe {
        println!("pe0: {}", (*(*pe0).data).id);
        println!("pe1: {}", (*(*pe1).data).id);
        println!("pe2: {}", (*(*pe2).data).id);

        assert_eq!(2048, (*(*pe0).data).id);
        assert_eq!(4097, (*(*pe1).data).id);
        assert_eq!(6146, (*(*pe2).data).id);
    }

    println!("env_init test over.");

    unsafe {
        for page_addr in (0..NPAGE * size_of::<PageNode>()).step_by(PAGE_SIZE) {
            assert_eq!(
                PADDR!(PAGES as usize) + page_addr,
                va2pa!(BASE_PGDIR, UPAGES + page_addr)
            );
        }

        for page_addr in (0..NENV * size_of::<EnvNode>()).step_by(PAGE_SIZE) {
            assert_eq!(
                PADDR!(addr_of!(ENVS_DATA) as usize) + page_addr,
                va2pa!(BASE_PGDIR, UENVS + page_addr)
            );
        }

        println!("pe1->env_pgdir 0x{:x}\n", (*(*pe1).data).pgdir as usize);

        assert_eq!(
            *BASE_PGDIR.add(PDX!(UTOP)),
            *(*(*pe2).data).pgdir.add(PDX!(UTOP))
        );

        assert_eq!(0, *(*(*pe2).data).pgdir.add(PDX!(UTOP) - 1));
    }

    println!("env_setup_vm test over.");

    println!("pe2`s sp register 0x{:x}\n", unsafe {
        (*(*pe2).data).trap_frame.regs[29]
    });

    unsafe {
        ENV_SCHE_LIST.insert_tail(pe0);
        ENV_SCHE_LIST.insert_tail(pe1);
        ENV_SCHE_LIST.insert_tail(pe2);

        println!("insert over");

        env_free(pe0);
        env_free(pe1);
        env_free(pe2);
    }
}

#[cfg(ktest_item = "env")]
pub fn test_envid2env() {
    use crate::{
        kdef::{env::EnvStatus, error::KError},
        kern::env::{env_alloc, envid2env, CUR_ENV},
    };

    let pe0;
    let pe2;
    unsafe {
        pe0 = env_alloc(0).unwrap();
        pe2 = env_alloc(0).unwrap();
        (*(*pe2).data).status = EnvStatus::Free;
        if let Err(KError::BadEnv) = envid2env((*(*pe2).data).id, false) {
        } else {
            unreachable!()
        }

        (*(*pe2).data).status = EnvStatus::Runnable;
        assert_eq!(
            (*(*pe2).data).id,
            (*(*envid2env((*(*pe2).data).id, false).unwrap()).data).id
        );

        CUR_ENV = pe0;
        assert!(envid2env((*(*pe2).data).id, true).is_err());
    }
}

#[cfg(ktest_item = "env")]
unsafe fn mem_eq(a: *const u8, b: *const u8, size: usize) {
    for i in 0..size {
        assert_eq!(
            *(a.add(i)),
            *(b.add(i)),
            "Non Eq 0x{:x}@0x{:x} <-> 0x{:x}@0x{:x}",
            *(a.add(i)),
            (a.add(i)) as usize,
            *(b.add(i)),
            (b.add(i)) as usize,
        );
    }
}

#[cfg(ktest_item = "env")]
unsafe fn mem_eqz(a: *const u8, size: usize) {
    for i in 0..size {
        assert_eq!(
            0,
            *(a.add(i)),
            "Non Eqz 0x{:x}@0x{:x} with i={}",
            *(a.add(i)),
            (a.add(i)) as usize,
            i
        );
    }
}

#[cfg(ktest_item = "env")]
unsafe fn seg_check(
    pgdir: *mut crate::kern::pmap::Pde,
    mut va: usize,
    mut std: *const u8,
    mut size: usize,
) {
    use crate::{
        kdef::mmu::PAGE_SIZE, kern::pmap::page_lookup, println, KADDR, PTE_ADDR, ROUNDDOWN,
    };
    use core::cmp::min;

    println!(
        "Segment check: 0x{:x} to 0x{:x} ({}) with std: 0x{:x}",
        va,
        va + size,
        size,
        std as usize
    );
    let off = va - ROUNDDOWN!(va; PAGE_SIZE);
    if off != 0 {
        let n = min(size, PAGE_SIZE - off);
        let (_, pte) = page_lookup(pgdir, va - off).unwrap();
        if std.is_null() {
            mem_eqz((KADDR!(PTE_ADDR!(*pte)) as usize + off) as *const u8, n);
        } else {
            mem_eq(
                (KADDR!(PTE_ADDR!(*pte)) as usize + off) as *const u8,
                std,
                n,
            );
            std = std.add(n);
        }
        va += n;
        size -= n;
    }

    for i in (0..size).step_by(PAGE_SIZE) {
        let n = min(size - i, PAGE_SIZE);
        let (_, pte) = page_lookup(pgdir, va + i).unwrap();
        if std.is_null() {
            mem_eqz(KADDR!(PTE_ADDR!(*pte)) as *const u8, n);
        } else {
            mem_eq(KADDR!(PTE_ADDR!(*pte)) as *const u8, std.add(i), n);
        }
    }
}

#[cfg(ktest_item = "env")]
pub fn test_icode_loader() {
    use crate::kern::env::env_create;
    use core::ptr::{addr_of, null};

    let binary_start = include_bytes!("bin/icode_check.b");
    let icode_check_401030 = include_bytes!("bin/icode_check.b.seg401030");
    let icode_check_402000 = include_bytes!("bin/icode_check.b.seg402000");
    unsafe {
        let e = env_create(addr_of!(*binary_start) as *const u8, binary_start.len(), 1).unwrap();
        seg_check(
            (*(*e).data).pgdir,
            0x401030,
            addr_of!(*icode_check_401030) as *const u8,
            icode_check_401030.len(),
        );
        seg_check(
            (*(*e).data).pgdir,
            0x402000,
            addr_of!(*icode_check_402000) as *const u8,
            icode_check_402000.len(),
        );
        seg_check((*(*e).data).pgdir, 0x402fbc, null(), 4048);
    }
}
