use core::{mem::size_of, ptr};

use crate::{debugln, println, PADDR, ROUND};

const PAGE_SIZE: usize = 4096;

pub struct Page {
    _pp_ref: u16,
}

pub fn mips_detect_memory(npage: &mut usize, memsize: usize) {
    *npage = memsize / 4096;
    println!(
        "Memory Size: {} KiB; Page Number: {}.",
        memsize / 1024,
        *npage
    );
}

fn alloc(freemem: &mut usize, memsize: usize, n: usize, align: usize, clear: bool) -> *mut Page {
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

    alloced_mem as *mut Page
}

pub fn mips_vm_init(pages: &mut *mut Page, freemem: &mut usize, npage: usize, memsize: usize) {
    *pages = alloc(freemem, memsize, npage * size_of::<Page>(), PAGE_SIZE, true);
    println!("Pages are to the memeory 0x{:x}", freemem);
    debugln!("> pmap.rs: mips vm init success");
}
