use crate::kernel_tests::slash_print;
use crate::println;

use super::TEST_FN;

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

TEST_FN!({
    let mut v1 = vec![12; 100]; // get one page
    v1[12] = 21;
    assert_eq!(v1[12], 21);
    assert_eq!(v1[21], 12);

    let mut v2 = vec![33; 1500]; // get two page
    v2[1499] = 34;
    assert_eq!(v2[1499], 34);

    let mut v3 = vec![42; 1000]; // get one page
    v3[0] = 44;
    assert_eq!(v3[0], 44);

    drop(v1);
    drop(v3);
    drop(v2);

    println!("Basic alloc and dealloc is Okay.");
    // Now, the system is the origin situation. We have 64 8-page items.

    let mut v = Vec::with_capacity(66); // 1
    for i in 0..63 {
        v.push(vec![3_u8; 4096 * 7 + 16]); // 63 * 8
        assert_eq!(3, v[i][10000]);
    }
    v.push(vec![1_u8; 4096 * 3 + 16]); // 4
    v.push(vec![1_u8; 4096 + 16]); // 2
    v.push(vec![1_u8; 16]); // 1

    // Now we used up all the memories
    drop(v); // Restore everything

    println!("Huge alloc and dealloc is Okay.");

    let mut v = Vec::with_capacity(510); // 2 pages
    for i in 0..510 {
        v.push(vec![2_u8; 4000]);
        assert_eq!(2, v[i][3000]);
    }

    println!("Tiny alloc is Okay.");

    let mut current_index = 1;
    for i in (1..510).rev() {
        v.remove(current_index);
        current_index = (current_index + 7) % i;
    }

    drop(v);

    println!("Tiny dealloc is Okay.");

    let mut v = Vec::with_capacity(66); // 1
    for i in 0..63 {
        v.push(vec![1_u8; 4096 * 7 + 16]); // 63 * 8
        assert_eq!(1, v[i][10000]);
    }
    v.push(vec![1_u8; 4096 * 3 + 16]); // 4
    v.push(vec![1_u8; 4096 + 16]); // 2
    v.push(vec![1_u8; 16]); // 1

    println!("Check merge is Okay.");

    drop(v);

    slash_print("Passed!");
    println!();
});
