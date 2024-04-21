#[cfg(ktest_item = "memory")]
pub fn test_page(
    page_free_list: &mut crate::kern::pmap::PageList,
    pages: &mut *mut crate::kern::pmap::PageNode,
    _freemem: &mut usize,
    _npage: usize,
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
