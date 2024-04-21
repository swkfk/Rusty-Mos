use core::{mem::size_of, ptr};

use crate::{
    debugln,
    kdef::{
        error::KError,
        mmu::{PTE_C_CACHEABLE, PTE_V},
        queue::{LinkList, LinkNode},
    },
    pa2page, page2kva, page2pa, println, ARRAY_PTR, KADDR, PADDR, PDX, PTE_ADDR, PTX, ROUND,
};

use super::tlbex::tlb_invalidate;

const PAGE_SIZE: usize = 4096;
const PAGE_SHIFT: usize = 12;

pub type PageList = LinkList<PageData>;
pub type PageNode = LinkNode<PageData>;
pub type Pde = u32;
pub type Pte = u32;

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

pub fn page_alloc(
    page_free_list: &mut PageList,
    pages: &*mut PageNode,
    // npage: usize,
) -> Result<*mut PageNode, KError> {
    match unsafe { page_free_list.pop_head() } {
        None => Err(KError::NoMem),
        Some(pp) => unsafe {
            ptr::write_bytes(page2kva!(pp, *pages; PageNode) as *mut u8, 0, PAGE_SIZE);
            Ok(pp)
        },
    }
}

pub fn page_free(page_free_list: &mut PageList, page: &mut *mut PageNode) {
    assert_eq!(0, unsafe { **page }.data.pp_ref);
    unsafe { page_free_list.insert_head(*page) };
}

pub fn page_decref(page_free_list: &mut PageList, page: &mut *mut PageNode) {
    assert!(unsafe { **page }.data.pp_ref > 0);
    unsafe { **page }.data.pp_ref -= 1;
    if unsafe { **page }.data.pp_ref == 0 {
        page_free(page_free_list, page);
    }
}

pub fn pgdir_walk(
    pgdir: *mut Pde,
    va: usize,
    create: bool,
    page_free_list: &mut PageList,
    pages: &*mut PageNode,
) -> Result<*mut Pte, KError> {
    let pgdir_entryp = (pgdir as u32 + (PDX!(va) * size_of::<Pde>()) as u32) as *mut Pte;
    if 0 == PTE_V & unsafe { *(pgdir_entryp as *const Pte) } {
        // Not Valid!
        if create {
            let pp = page_alloc(page_free_list, pages)?;
            unsafe {
                ptr::write(
                    pgdir_entryp,
                    (PTE_ADDR!(page2pa!(pp, *pages; PageNode)) as Pte | PTE_C_CACHEABLE | PTE_V),
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
    page_free_list: &mut PageList,
    pages: &*mut PageNode,
) -> Result<(), KError> {
    if let Ok(pte) = pgdir_walk(pgdir, va, false, page_free_list, pages) {
        if !pte.is_null() && unsafe { *pte & PTE_V } != 0 {
            if pp as usize == pa2page!(unsafe { *pte }, *pages; PageNode) {
                page_remove(pgdir, va, asid, page_free_list, pages);
            } else {
                tlb_invalidate(asid, va);
                ptr::write(
                    pte,
                    page2pa!(pp, *pages; PageNode) as Pte | perm | PTE_C_CACHEABLE | PTE_V,
                )
            }
        }
    }

    tlb_invalidate(asid, va);
    let pte = pgdir_walk(pgdir, va, true, page_free_list, pages)?;
    ptr::write(
        pte,
        page2pa!(pp, *pages; PageNode) as Pte | perm | PTE_C_CACHEABLE | PTE_V,
    );
    (*pp).data.pp_ref += 1;

    Ok(())
}

pub fn page_lookup(
    pgdir: *mut Pde,
    va: usize,
    page_free_list: &mut PageList,
    pages: &*mut PageNode,
) -> Option<(*mut PageNode, *mut Pte)> {
    if let Ok(pte) = pgdir_walk(pgdir, va, false, page_free_list, pages) {
        if pte.is_null() || unsafe { *pte & PTE_V == 0 } {
            None
        } else {
            let pp = pa2page!(unsafe { *pte }, *pages; PageNode) as *mut PageNode;
            Some((pp, pte))
        }
    } else {
        None
    }
}

pub fn page_remove(
    pgdir: *mut Pde,
    va: usize,
    asid: u32,
    page_free_list: &mut PageList,
    pages: &*mut PageNode,
) {
    if let Some((mut pp, pte)) = page_lookup(pgdir, va, page_free_list, pages) {
        page_decref(page_free_list, &mut pp);
        unsafe { ptr::write(pte, 0) };
        tlb_invalidate(asid, va);
    }
}
