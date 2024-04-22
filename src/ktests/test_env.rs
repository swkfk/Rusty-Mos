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
