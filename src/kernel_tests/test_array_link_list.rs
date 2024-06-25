use crate::{kernel_tests::slash_print, println, utils::array_based_list::ArrayLinkedList};

use super::TEST_FN;

TEST_FN!({
    let mut list = ArrayLinkedList::<100>::new();
    assert!(list.empty());
    println!("List create is Okay.");

    for i in 0..100 {
        list.insert_head(i);
    }

    for i in (0..100).rev() {
        assert_eq!(i, list.pop_head().unwrap());
    }

    for i in 50..100 {
        list.insert_head(i);
    }

    for i in 0..50 {
        list.insert_tail(i);
    }

    for i in (50..100).rev() {
        assert_eq!(i, list.pop_head().unwrap());
    }

    for i in 0..50 {
        assert_eq!(i, list.pop_head().unwrap());
    }

    assert_eq!(None, list.pop_head());
    println!("Insert and pop is Okay.");

    list.insert_tail(9);
    assert_eq!(9, list.pop_head().unwrap());
    assert!(!list.contains(9));
    assert!(list.empty());

    list.insert_head(4);
    list.insert_head(0);
    list.insert_head(1);
    list.insert_head(2);

    for _ in 0..100 {
        for _ in 0..=2 {
            let p = list.pop_head().unwrap();
            list.insert_tail(p);
        }
        list.insert_head(5);
        assert_eq!(5, list.pop_head().unwrap());
        assert_eq!(4, list.pop_head().unwrap());
        list.insert_tail(4);
    }

    println!("Frequently insert is Okay.");

    for i in 10..100 {
        if i % 17 == 0 {
            list.insert_head(i);
        } else if i % 13 == 0 {
            list.insert_tail(i);
        }
    }

    while !list.empty() {
        list.pop_head();
    }

    println!("Loop pop is Okay.");

    list.insert_head(2);
    list.insert_tail(5);
    list.remove(5);

    assert_eq!(2, list.pop_head().unwrap());
    assert!(list.empty());

    list.insert_head(2);
    list.insert_head(5);
    list.remove(5);

    assert_eq!(2, list.pop_head().unwrap());
    assert!(list.empty());

    list.insert_head(1);
    list.insert_tail(2);
    list.insert_head(3);

    list.remove(1);
    assert_eq!(3, list.pop_head().unwrap());
    assert_eq!(2, list.pop_head().unwrap());
    assert!(list.empty());

    println!("Remove is Okay.");

    slash_print("Passed!");
    println!();
});
