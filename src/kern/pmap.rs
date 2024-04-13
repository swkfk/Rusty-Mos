use core::{mem::size_of, ptr};

use crate::{debugln, println, PADDR, ROUND};

const PAGE_SIZE: usize = 4096;
static mut MEM_SIZE: usize = 0;
static mut NPAGE: usize = 0;
static mut FREEMEM: usize = 0;
static mut PAGES: *mut Page = core::ptr::null_mut();

struct Page {
    _pp_ref: u16,
}

pub fn mips_detect_memory(memsize: u32) {
    unsafe { MEM_SIZE = memsize as usize };
    unsafe { NPAGE = memsize as usize / 4096 };
    println!(
        "Memory Size: {} KiB; Page Number: {}.",
        memsize / 1024,
        unsafe { NPAGE }
    );
}

fn alloc(n: usize, align: usize, clear: bool) -> *mut Page {
    extern "C" {
        fn end();
    }
    println!("Externed end = 0x{:x} bytes", end as usize);

    let alloced_mem;

    unsafe {
        if FREEMEM == 0 {
            FREEMEM = end as usize;
        }

        FREEMEM = ROUND!(FREEMEM; align);
        alloced_mem = FREEMEM;
        FREEMEM += n;

        assert!(PADDR!(FREEMEM) < MEM_SIZE, "Out of memory for pages");

        if clear {
            ptr::write_bytes(alloced_mem as *mut u8, 0, n);
        }
    }

    alloced_mem as *mut Page
}

pub fn mips_vm_init() {
    unsafe { PAGES = alloc(NPAGE * size_of::<Page>(), PAGE_SIZE, true) };
    println!("Pages are to the memeory 0x{:x}", unsafe { FREEMEM });
    debugln!("> pmap.rs: mips vm init success");
}
