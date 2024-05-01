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
            env::{env_alloc, env_free, BASE_PGDIR, ENVS, ENV_FREE_LIST, ENV_SCHE_LIST},
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
        println!("pe0: {}", (*pe0).data.id);
        println!("pe1: {}", (*pe1).data.id);
        println!("pe2: {}", (*pe2).data.id);

        assert_eq!(2048, (*pe0).data.id);
        assert_eq!(4097, (*pe1).data.id);
        assert_eq!(6146, (*pe2).data.id);
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
                PADDR!(addr_of!(ENVS) as usize) + page_addr,
                va2pa!(BASE_PGDIR, UENVS + page_addr)
            );
        }

        println!("pe1->env_pgdir 0x{:x}\n", (*pe1).data.pgdir as usize);

        assert_eq!(
            *BASE_PGDIR.add(PDX!(UTOP)),
            *(*pe2).data.pgdir.add(PDX!(UTOP))
        );

        assert_eq!(0, *(*pe2).data.pgdir.add(PDX!(UTOP) - 1));
    }

    println!("env_setup_vm test over.");

    println!("pe2`s sp register 0x{:x}\n", unsafe {
        (*pe2).data.trap_frame.regs[29]
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
        (*pe2).data.status = EnvStatus::Free;
        if let Err(KError::BadEnv) = envid2env((*pe2).data.id, false) {
        } else {
            unreachable!()
        }

        (*pe2).data.status = EnvStatus::Runnable;
        assert_eq!(
            (*pe2).data.id,
            (*envid2env((*pe2).data.id, false).unwrap()).data.id
        );

        CUR_ENV = pe0;
        assert!(envid2env((*pe2).data.id, true).is_err());
    }
}
