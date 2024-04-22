#[cfg(ktest_item = "memory")]
pub fn test_page(
    page_free_list: &mut crate::kern::pmap::PageList,
    pages: &mut *mut crate::kern::pmap::PageNode,
) {
    use crate::kdef::mmu::{PTE_C_CACHEABLE, PTE_V};
    use crate::kern::pmap::{page_alloc, page_free, page_insert, PageList, PageNode};
    use crate::{kern::pmap::Pde, page2kva};

    let pp = page_alloc(page_free_list, pages).unwrap();
    let boot_pgdir = page2kva!(pp, *pages; PageNode) as *mut Pde;

    let mut pp0 = page_alloc(page_free_list, pages).unwrap();
    let pp1 = page_alloc(page_free_list, pages).unwrap();
    let mut pp2 = page_alloc(page_free_list, pages).unwrap();
    assert!(!pp0.is_null());
    assert!(!pp1.is_null() && pp1 != pp0);
    assert!(!pp2.is_null() && pp2 != pp0);

    crate::println!(
        "Normal page_alloc test over. boot_pgdir = 0x{:x}",
        boot_pgdir as usize
    );

    let mut zfl = PageList::new();
    assert!(page_alloc(&mut zfl, pages).is_err());
    assert!(unsafe { page_insert(boot_pgdir, 0, 0, 0, pp1, &mut zfl, pages).is_err() });
    page_free(&mut zfl, &mut pp0);
    assert!(unsafe { page_insert(boot_pgdir, 0, 0, 0, pp1, &mut zfl, pages).is_ok() });

    assert_eq!(PTE_C_CACHEABLE | PTE_V, unsafe {
        core::ptr::read(boot_pgdir) & 0xFFF
    });

    assert_eq!(crate::page2pa!(pp0, *pages; PageNode) as u32, unsafe {
        core::ptr::read(boot_pgdir) & !0xFFF
    });

    assert_eq!(PTE_C_CACHEABLE | PTE_V, unsafe {
        core::ptr::read(crate::page2kva!(pp0, *pages; PageNode) as *mut u32) & 0xFFF
    });

    crate::println!("Empty page_free_list test over.");

    let temp = crate::page2kva!(pp2, *pages; PageNode) as *mut u32;
    unsafe {
        *temp = 1000;
    }
    page_free(page_free_list, &mut pp2);
    assert_eq!(1000, unsafe { *temp });

    pp0 = page_alloc(page_free_list, pages).unwrap();
    assert!(!pp0.is_null());

    let temp2 = crate::page2kva!(pp2, *pages; PageNode) as *mut u32;
    assert_eq!(temp, temp2);
    assert_eq!(0, unsafe { *temp2 });

    crate::println!("Directely assignment test over.");
}

#[cfg(ktest_item = "memory")]
pub fn test_page_strong(
    page_free_list: &mut crate::kern::pmap::PageList,
    pages: &mut *mut crate::kern::pmap::PageNode,
) {
    use crate::{
        kdef::{
            error::KError,
            mmu::{PAGE_SIZE, PDMAP},
        },
        kern::{
            pmap::{
                page_alloc, page_free, page_insert, page_remove, PageList, PageNode, Pde, CUR_PGDIR,
            },
            tlbex::tlb_init_global_vars,
        },
        pa2page, page2kva, page2pa, println, va2pa, PADDR, PTE_ADDR,
    };
    use core::ptr;

    let pp = page_alloc(page_free_list, pages).unwrap();
    let boot_pgdir = page2kva!(pp, *pages; PageNode) as *mut Pde;
    unsafe { CUR_PGDIR = boot_pgdir };

    let mut pp0 = page_alloc(page_free_list, pages).unwrap();
    let mut pp1 = page_alloc(page_free_list, pages).unwrap();
    let mut pp2 = page_alloc(page_free_list, pages).unwrap();
    let mut pp3 = page_alloc(page_free_list, pages).unwrap();
    let mut pp4 = page_alloc(page_free_list, pages).unwrap();

    assert!(!pp0.is_null());
    assert!(!pp1.is_null() && pp1 != pp0);
    assert!(!pp2.is_null() && pp2 != pp0 && pp2 != pp1);
    assert!(!pp3.is_null() && pp3 != pp0 && pp3 != pp2 && pp3 != pp1);
    assert!(!pp4.is_null() && pp4 != pp0 && pp4 != pp3 && pp4 != pp2 && pp4 != pp1);

    let mut zfl = PageList::new();
    tlb_init_global_vars(&mut zfl, pages);

    unsafe { assert!(page_insert(boot_pgdir, 0, 0, 0, pp1, &mut zfl, pages).is_err()) };
    if let Err(KError::NoMem) = page_alloc(&mut zfl, pages) {
    } else {
        unreachable!()
    }

    page_free(&mut zfl, &mut pp0);
    unsafe {
        assert!(page_insert(boot_pgdir, 0, 0, 0, pp1, &mut zfl, pages).is_ok());
        assert!(page_insert(boot_pgdir, PAGE_SIZE, 0, 0, pp2, &mut zfl, pages).is_ok());
        assert!(page_insert(boot_pgdir, PAGE_SIZE * 2, 0, 0, pp3, &mut zfl, pages).is_ok());
        assert_eq!(
            page2pa!(pp0, *pages; PageNode),
            PTE_ADDR!((*boot_pgdir) as usize)
        );

        println!("va2pa(boot_pgdir, 0x0) is 0x{:x}.", va2pa!(boot_pgdir, 0x0));
        println!("page2pa(pp1) is 0x{:x}.", page2pa!(pp1, *pages; PageNode));
    }

    unsafe {
        assert_eq!(va2pa!(boot_pgdir, 0x0), page2pa!(pp1, *pages; PageNode));
        assert_eq!((*pp1).data.pp_ref, 1);
        assert_eq!(
            va2pa!(boot_pgdir, PAGE_SIZE),
            page2pa!(pp2, *pages; PageNode)
        );
        assert_eq!((*pp2).data.pp_ref, 1);
        assert_eq!(
            va2pa!(boot_pgdir, PAGE_SIZE * 2),
            page2pa!(pp3, *pages; PageNode)
        );
        assert_eq!((*pp3).data.pp_ref, 1);
    }

    println!("Start Page Insert.");

    unsafe {
        assert!(page_insert(boot_pgdir, PAGE_SIZE, 0, 0, pp2, &mut zfl, pages).is_ok());
        assert_eq!(
            va2pa!(boot_pgdir, PAGE_SIZE),
            page2pa!(pp2, *pages; PageNode)
        );
        assert_eq!((*pp2).data.pp_ref, 1);
    }

    unsafe {
        assert!(page_insert(boot_pgdir, PDMAP, 0, 0, pp0, &mut zfl, pages).is_err());
        page_remove(boot_pgdir, 0, 0, &mut zfl, pages);
        assert_eq!(!0, va2pa!(boot_pgdir, 0x0));
        assert!(page_insert(boot_pgdir, PDMAP, 0, 0, pp0, &mut zfl, pages).is_ok());
    }

    unsafe {
        assert!(page_insert(boot_pgdir, PAGE_SIZE * 2, 0, 0, pp2, &mut zfl, pages).is_ok());
        assert_eq!(
            va2pa!(boot_pgdir, PAGE_SIZE),
            page2pa!(pp2, *pages; PageNode)
        );
        assert_eq!(
            va2pa!(boot_pgdir, PAGE_SIZE * 2),
            page2pa!(pp2, *pages; PageNode)
        );
        assert_eq!((*pp2).data.pp_ref, 2);
        assert_eq!((*pp3).data.pp_ref, 0);

        assert!(page_insert(boot_pgdir, PAGE_SIZE + PDMAP, 0, 0, pp2, &mut zfl, pages).is_ok());
        assert_eq!((*pp2).data.pp_ref, 3);
    }

    println!("Page Insert test over.");

    let pp = page_alloc(&mut zfl, pages).unwrap();
    assert_eq!(pp, pp3);
    page_remove(boot_pgdir, PAGE_SIZE, 0, &mut zfl, pages);
    unsafe {
        assert_eq!(
            va2pa!(boot_pgdir, PAGE_SIZE * 2),
            page2pa!(pp2, *pages; PageNode)
        );
        assert_eq!((*pp2).data.pp_ref, 2);
        assert_eq!((*pp3).data.pp_ref, 0);
    }

    page_remove(boot_pgdir, PAGE_SIZE * 2, 0, &mut zfl, pages);
    unsafe {
        assert_eq!(va2pa!(boot_pgdir, 0x0), !0);
        assert_eq!(va2pa!(boot_pgdir, PAGE_SIZE), !0);
        assert_eq!(va2pa!(boot_pgdir, PAGE_SIZE * 2), !0);
        assert_eq!((*pp2).data.pp_ref, 1);
        assert_eq!((*pp3).data.pp_ref, 0);
    }

    page_remove(boot_pgdir, PAGE_SIZE + PDMAP, 0, &mut zfl, pages);
    unsafe {
        assert_eq!(va2pa!(boot_pgdir, 0x0), !0);
        assert_eq!(va2pa!(boot_pgdir, PAGE_SIZE), !0);
        assert_eq!(va2pa!(boot_pgdir, PAGE_SIZE * 2), !0);
        assert_eq!(va2pa!(boot_pgdir, PAGE_SIZE + PDMAP), !0);
        assert_eq!((*pp2).data.pp_ref, 0);
    }

    let pp = page_alloc(&mut zfl, pages).unwrap();
    assert_eq!(pp, pp2);

    assert!(page_alloc(&mut zfl, pages).is_err());
    unsafe {
        assert_eq!(
            page2pa!(pp0, *pages; PageNode),
            PTE_ADDR!(*boot_pgdir as usize)
        );
        assert_eq!(
            page2pa!(pp1, *pages; PageNode),
            PTE_ADDR!((*(boot_pgdir as *mut [u32; 2]))[1] as usize)
        );
    }

    unsafe {
        ptr::write(boot_pgdir as *mut [u32; 2], [0; 2]);
        assert_eq!((*pp0).data.pp_ref, 2);
        assert_eq!((*pp1).data.pp_ref, 1);
        (*pp0).data.pp_ref = 0;
        (*pp1).data.pp_ref = 0;
    }

    tlb_init_global_vars(page_free_list, pages);
    page_free(page_free_list, &mut pp0);
    page_free(page_free_list, &mut pp1);
    page_free(page_free_list, &mut pp2);
    page_free(page_free_list, &mut pp3);
    page_free(page_free_list, &mut pp4);
    page_free(
        page_free_list,
        &mut (pa2page!(PADDR!(boot_pgdir as usize), *pages; PageNode) as *mut PageNode),
    );
}

#[cfg(ktest_item = "memory")]
pub fn test_tlb_refill(
    page_free_list: &mut crate::kern::pmap::PageList,
    pages: &mut *mut crate::kern::pmap::PageNode,
) {
    use crate::{
        kdef::mmu::PAGE_SIZE,
        kern::{
            pmap::{
                page_alloc, page_free, page_insert, page_lookup, PageList, PageNode, Pde, CUR_PGDIR,
            },
            tlbex::{_do_tlb_refill, tlb_init_global_vars},
        },
        page2kva, page2pa, println, va2pa,
    };
    use core::arch::asm;

    let pp0 = page_alloc(page_free_list, pages).unwrap();
    let boot_pddir = page2kva!(pp0, *pages; PageNode) as *mut Pde;
    unsafe { CUR_PGDIR = boot_pddir };

    let mut pp0 = page_alloc(page_free_list, pages).unwrap();
    let pp1 = page_alloc(page_free_list, pages).unwrap();
    let pp2 = page_alloc(page_free_list, pages).unwrap();
    let mut pp3 = page_alloc(page_free_list, pages).unwrap();
    let mut pp4 = page_alloc(page_free_list, pages).unwrap();

    let mut fl = PageList::new();
    tlb_init_global_vars(&mut fl, pages);

    page_free(&mut fl, &mut pp0);
    unsafe {
        page_insert(boot_pddir, 0, 0, 0, pp1, &mut fl, pages).unwrap();
        page_insert(boot_pddir, PAGE_SIZE, 0, 0, pp2, &mut fl, pages).unwrap();
    }

    println!("TLB-Refill check begin.");

    let entrys: &mut [u32; 2] = &mut [0; 2];
    _do_tlb_refill(entrys, PAGE_SIZE, 0);
    let (_, walk_pte) = page_lookup(boot_pddir, PAGE_SIZE, &mut fl, pages).unwrap();

    unsafe {
        println!("  entrys: {}, {}", entrys[0], entrys[1]);
        println!("  Left  Arm: {}", entrys[0] == (*walk_pte >> 6));
        println!("  Right Arm: {}", entrys[1] == (*walk_pte >> 6));
        assert!((entrys[0] == (*walk_pte >> 6)) ^ (entrys[1] == (*walk_pte >> 6)));
        assert_eq!(
            page2pa!(pp2, *pages; PageNode),
            va2pa!(boot_pddir, PAGE_SIZE)
        );
    };

    println!("Test #1 Passed.");

    page_free(&mut fl, &mut pp4);
    page_free(&mut fl, &mut pp3);

    assert!(page_lookup(boot_pddir, 0x00400000, &mut fl, pages).is_none());
    _do_tlb_refill(entrys, 0x00400000, 0);
    let (pp, _) = page_lookup(boot_pddir, 0x00400000, &mut fl, pages).unwrap();
    assert!(!pp.is_null());
    unsafe {
        assert_eq!(
            page2pa!(pp3, *pages; PageNode),
            va2pa!(boot_pddir, 0x00400000)
        );
    }

    println!("Test #2 Passed.");

    let mut index = -1_i32;
    let badva = 0x00400000_u32;
    let entryhi = badva & 0xffffe000_u32;
    let entrylo: u32;
    unsafe {
        asm!("mtc0 {}, $10" , in(reg) entryhi);
        extern "C" {
            fn do_tlb_refill_call(non_used: u32, va: u32, entryhi: u32);
        }
        do_tlb_refill_call(0, badva, entryhi);

        asm!("mtc0 {}, $0" , in(reg) index);
        asm!("tlbp");
        asm!("nop");
        asm!("mfc0 {}, $0" , out(reg) index);
        assert!(index >= 0);
        asm!("tlbr");
        asm!("mfc0 {}, $2" , out(reg) entrylo);
        assert!((entrylo == entrys[0]) ^ (entrylo == entrys[1]));
    }
    println!("Test #3 Passed.");
}

#[cfg(ktest_item = "memory")]
pub fn test_linklist() {
    use crate::println;
    use core::ptr::addr_of_mut;
    type LinkList = crate::kdef::queue::LinkList<u32>;
    type LinkNode = crate::kdef::queue::LinkNode<u32>;

    let mut ll = LinkList::new();
    assert!(ll.empty());

    let mut node1 = LinkNode::new(1);

    unsafe { ll.insert_head(addr_of_mut!(node1)) };
    assert!(!ll.empty());

    unsafe { LinkList::remove(addr_of_mut!(node1)) };
    assert!(ll.empty());

    println!("Single node test over.");

    let mut node2 = LinkNode::new(2);
    let mut node3 = LinkNode::new(3);
    let mut node4 = LinkNode::new(4);
    unsafe {
        ll.insert_head(addr_of_mut!(node3));
        ll.insert_head(addr_of_mut!(node4));
        ll.insert_head(addr_of_mut!(node2));

        assert_eq!(ll.pop_head().unwrap(), addr_of_mut!(node2));
        assert_eq!(ll.pop_head().unwrap(), addr_of_mut!(node4));
        assert_eq!(ll.pop_head().unwrap(), addr_of_mut!(node3));
    }

    assert!(ll.empty());

    println!("Multy nodes test over.");

    unsafe {
        ll.insert_head(addr_of_mut!(node3));
        ll.insert_head(addr_of_mut!(node4));
        ll.insert_head(addr_of_mut!(node2));
        LinkList::remove(addr_of_mut!(node4));
        assert_eq!(ll.pop_head().unwrap(), addr_of_mut!(node2));
    }

    assert!(!ll.empty());

    println!(
        "Prev: 0x{:x}, Next: 0x{:x}",
        node3.prev as usize, node3.next as usize
    );
    println!("Head: 0x{:x}", ll.head as usize);
    unsafe { LinkList::remove(addr_of_mut!(node3)) };
    assert!(ll.empty());

    println!("Remove test over.");
}
