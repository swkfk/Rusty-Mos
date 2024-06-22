use core::{mem::size_of, ptr};

use crate::utils::linked_list::{LinkList, LinkNode};
use crate::utils::sync_ref_cell::SyncImplRef;
use crate::{
    debugln,
    kdef::{
        error::KError,
        mmu::{PGSHIFT, PTE_C_CACHEABLE, PTE_V},
    },
    pa2page, page2kva, page2pa, println, ARRAY_PTR, KADDR, PADDR, PDX, PTE_ADDR, PTX, ROUND,
};

use super::tlbex::tlb_invalidate;

pub static CUR_PGDIR: SyncImplRef<*mut Pde> = SyncImplRef::new(ptr::null_mut());
pub static PAGES: SyncImplRef<*mut PageNode> = SyncImplRef::new(core::ptr::null_mut());
pub static PAGE_FREE_LIST: SyncImplRef<PageList> = SyncImplRef::new(PageList::new());
pub static KERN_HEAP: SyncImplRef<*mut PageNode> = SyncImplRef::new(core::ptr::null_mut());
pub static NPAGE: SyncImplRef<usize> = SyncImplRef::new(0);

const PAGE_SIZE: usize = 4096;

pub type PageList = LinkList<PageData>;
pub type PageNode = LinkNode<PageData>;
pub type Pde = u32;
pub type Pte = u32;

#[derive(Clone, Copy)]
pub struct PageData {
    pub pp_ref: u16,
}

pub fn mips_detect_memory(memsize: usize) {
    *NPAGE.borrow_mut() = memsize / 4096;
    println!(
        "Memory Size: {} KiB; Page Number: {}.",
        memsize / 1024,
        *NPAGE.borrow()
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

    if *freemem == 0 {
        println!("Externed end = 0x{:x} bytes", end as usize);
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

pub fn mips_vm_init(freemem: &mut usize, memsize: usize) {
    *PAGES.borrow_mut() = alloc(
        freemem,
        memsize,
        *NPAGE.borrow() * size_of::<PageNode>(),
        PAGE_SIZE,
        true,
    );
    println!("Pages are to the memeory 0x{:x}", freemem);
    *KERN_HEAP.borrow_mut() = alloc(freemem, memsize, 512 * PAGE_SIZE, PAGE_SIZE, true);
    let page_start =
        pa2page!(PADDR!(*KERN_HEAP.borrow() as usize), *PAGES.borrow(); PageNode) as *mut PageNode;
    unsafe { crate::BUDDY_ALLOCATOR.init(page_start, 512 * PAGE_SIZE) }

    println!("Heaps are to the memeory 0x{:x}", freemem);
    debugln!("> pmap.rs: mips vm init success");
}

pub fn page_init(freemem: &mut usize) {
    let pages = *PAGES.borrow_mut();

    *freemem = ROUND!(*freemem; PAGE_SIZE);

    let mut page_id = 0;
    while page_id < *NPAGE.borrow() && page_id << PGSHIFT < PADDR!(*freemem) {
        unsafe { ((*ARRAY_PTR!(pages; page_id, PageNode)).data).pp_ref = 1 };
        page_id += 1;
    }

    debugln!("> pmap.rs: pages are used for {}", page_id);

    while page_id < *NPAGE.borrow() {
        unsafe { ((*ARRAY_PTR!(pages; page_id, PageNode)).data).pp_ref = 0 };
        unsafe { (*PAGE_FREE_LIST.borrow_mut()).insert_head(ARRAY_PTR!(pages; page_id, PageNode)) };
        page_id += 1;
    }
}

pub fn page_alloc() -> Result<*mut PageNode, KError> {
    match unsafe { (*PAGE_FREE_LIST.borrow_mut()).pop_head() } {
        None => Err(KError::NoMem),
        Some(pp) => unsafe {
            ptr::write_bytes(
                page2kva!(pp, *PAGES.borrow(); PageNode) as *mut u8,
                0,
                PAGE_SIZE,
            );
            Ok(pp)
        },
    }
}

pub fn page_free(page: &mut *mut PageNode) {
    assert_eq!(0, unsafe { **page }.data.pp_ref);
    unsafe { (*PAGE_FREE_LIST.borrow_mut()).insert_head(*page) };
}

pub fn page_decref(page: &mut *mut PageNode) {
    assert!(unsafe { **page }.data.pp_ref > 0);
    unsafe { (**page).data.pp_ref -= 1 };
    if unsafe { **page }.data.pp_ref == 0 {
        page_free(page);
    }
}

pub fn pgdir_walk(pgdir: *mut Pde, va: usize, create: bool) -> Result<*mut Pte, KError> {
    let pgdir_entryp = (pgdir as u32 + (PDX!(va) * size_of::<Pde>()) as u32) as *mut Pte;
    if 0 == PTE_V & unsafe { *(pgdir_entryp as *const Pte) } {
        // Not Valid!
        if create {
            let pp = page_alloc()?;
            unsafe {
                ptr::write(
                    pgdir_entryp,
                    (PTE_ADDR!(page2pa!(pp, *PAGES.borrow(); PageNode)) as Pte
                        | PTE_C_CACHEABLE
                        | PTE_V),
                );
                (*pp).data.pp_ref = 1;
            }
        } else {
            return Ok(ptr::null_mut());
        }
    }

    unsafe {
        Ok((KADDR!(PTE_ADDR!(*pgdir_entryp)) + (PTX!(va) * size_of::<Pte>()) as u32) as *mut Pte)
    }
}

/// # Safety
///
pub unsafe fn page_insert(
    pgdir: *mut Pde,
    va: usize,
    asid: u32,
    perm: u32,
    pp: *mut PageNode,
) -> Result<(), KError> {
    if let Ok(pte) = pgdir_walk(pgdir, va, false) {
        if !pte.is_null() && unsafe { *pte & PTE_V } != 0 {
            if pp as usize != pa2page!(unsafe { *pte }, *PAGES.borrow(); PageNode) {
                page_remove(pgdir, va, asid);
            } else {
                tlb_invalidate(asid, va);
                ptr::write(
                    pte,
                    page2pa!(pp, *PAGES.borrow(); PageNode) as Pte | perm | PTE_C_CACHEABLE | PTE_V,
                );
                return Ok(());
            }
        }
    }

    tlb_invalidate(asid, va);
    let pte = pgdir_walk(pgdir, va, true)?;
    ptr::write(
        pte,
        page2pa!(pp, *PAGES.borrow(); PageNode) as Pte | perm | PTE_C_CACHEABLE | PTE_V,
    );
    (*pp).data.pp_ref += 1;

    Ok(())
}

pub fn page_lookup(pgdir: *mut Pde, va: usize) -> Option<(*mut PageNode, *mut Pte)> {
    if let Ok(pte) = pgdir_walk(pgdir, va, false) {
        if pte.is_null() || unsafe { *pte & PTE_V == 0 } {
            None
        } else {
            let pp = unsafe { pa2page!( *pte , *PAGES.borrow(); PageNode) as *mut PageNode };
            Some((pp, pte))
        }
    } else {
        None
    }
}

pub fn page_remove(pgdir: *mut Pde, va: usize, asid: u32) {
    if let Some((mut pp, pte)) = page_lookup(pgdir, va) {
        page_decref(&mut pp);
        unsafe { ptr::write(pte, 0) };
        tlb_invalidate(asid, va);
    }
}
