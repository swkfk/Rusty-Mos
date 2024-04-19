#[cfg(ktest_item = "memory")]
pub fn test_page(
    page_free_list: &mut crate::kern::pmap::PageList,
    pages: &mut *mut crate::kern::pmap::PageNode,
    freemem: &mut usize,
    npage: usize,
) {
    let pp0 = crate::kern::pmap::page_alloc(page_free_list, pages).unwrap();
    let pp1 = crate::kern::pmap::page_alloc(page_free_list, pages).unwrap();
    let pp2 = crate::kern::pmap::page_alloc(page_free_list, pages).unwrap();
    assert!(!pp0.is_null());
    assert!(!pp1.is_null() && pp1 != pp0);
    assert!(!pp2.is_null() && pp2 != pp0);

    let mut zfl = crate::kern::pmap::PageList::new();
    assert!(crate::kern::pmap::page_alloc(&mut zfl, pages).is_err());
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
