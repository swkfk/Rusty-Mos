use core::{mem::size_of, ptr};

use crate::{
    debugln,
    kdef::queue::{LinkList, LinkNode},
    println, ARRAY_PTR, PADDR, ROUND,
};

const PAGE_SIZE: usize = 4096;
const PAGE_SHIFT: usize = 12;

pub type PageList = LinkList<PageData>;
pub type PageNode = LinkNode<PageData>;

#[derive(Clone, Copy)]
pub struct PageData {
    pp_ref: u16,
}

pub fn mips_detect_memory(npage: &mut usize, memsize: usize) {
    *npage = memsize / 4096;
    println!(
        "Memory Size: {} KiB; Page Number: {}.",
        memsize / 1024,
        *npage
    );
}

fn alloc(
    freemem: &mut usize,
    memsize: usize,
    n: usize,
    align: usize,
    clear: bool,
) -> *mut PageNode {
    extern "C" {
        fn end();
    }
    println!("Externed end = 0x{:x} bytes", end as usize);

    if *freemem == 0 {
        *freemem = end as usize;
    }

    *freemem = ROUND!(*freemem; align);
    let alloced_mem = *freemem;
    *freemem += n;

    assert!(PADDR!(*freemem) < memsize, "Out of memory for pages");

    if clear {
        unsafe {
            ptr::write_bytes(alloced_mem as *mut u8, 0, n);
        }
    }

    alloced_mem as *mut PageNode
}

pub fn mips_vm_init(pages: &mut *mut PageNode, freemem: &mut usize, npage: usize, memsize: usize) {
    *pages = alloc(
        freemem,
        memsize,
        npage * size_of::<PageNode>(),
        PAGE_SIZE,
        true,
    );
    println!("Pages are to the memeory 0x{:x}", freemem);
    debugln!("> pmap.rs: mips vm init success");
}

pub fn page_init(pages: &mut *mut PageNode, freemem: &mut usize, npage: usize) -> PageList {
    let mut page_free_list = PageList::new();

    *freemem = ROUND!(*freemem; PAGE_SIZE);

    let mut page_id = 0;
    while page_id < npage && page_id << PAGE_SHIFT < PADDR!(*freemem) {
        unsafe { ((*ARRAY_PTR!(*pages; page_id, PageNode)).data).pp_ref = 1 };
        page_id += 1;
    }

    while page_id < npage {
        unsafe { ((*ARRAY_PTR!(*pages; page_id, PageNode)).data).pp_ref = 0 };
        unsafe { page_free_list.insert_head(ARRAY_PTR!(*pages; page_id, PageNode)) };
        page_id += 1;
    }

    page_free_list
}
