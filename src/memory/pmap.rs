//! The page-memory manager model of MOS. Core functions are provided here.

use core::{mem::size_of, ptr};

use crate::utils::linked_list::{LinkList, LinkNode};
use crate::utils::sync_ref_cell::SyncImplRef;
use crate::{
    consts::error::KError,
    debugln,
    memory::regions::{PGSHIFT, PTE_C_CACHEABLE, PTE_V},
    pa2page, page2kva, page2pa, println, KADDR, PADDR, PDX, PTE_ADDR, PTX, ROUND,
};

use super::tlbex::tlb_invalidate;

/// Current env's page directary address.
pub static CUR_PGDIR: SyncImplRef<*mut Pde> = SyncImplRef::new(ptr::null_mut());
/// The kernel array for all pages.
pub static PAGES: SyncImplRef<*mut PageNode> = SyncImplRef::new(core::ptr::null_mut());
/// The list of free pages.
pub static PAGE_FREE_LIST: SyncImplRef<PageList> = SyncImplRef::new(PageList::new());
/// The kernel heap start. Used by the buddy system.
pub static KERN_HEAP: SyncImplRef<*mut PageNode> = SyncImplRef::new(core::ptr::null_mut());
/// The the count of all the pages available.
pub static NPAGE: SyncImplRef<usize> = SyncImplRef::new(0);

/// The page size is 4 KB.
const PAGE_SIZE: usize = 4096;

/// Page list, for the 'free_list'. See Also: [PAGE_FREE_LIST].
pub type PageList = LinkList<PageData>;
/// Node in the page list.
pub type PageNode = LinkNode<PageData>;
/// The page directory entry type alias.
pub type Pde = u32;
/// The page table entry type alias.
pub type Pte = u32;

/// The only page data needed to maintain.
#[derive(Clone, Copy)]
pub struct PageData {
    /// The reference count of the page.
    pub pp_ref: u16,
}

/// Calculate the [NPAGE] and validate it.
pub fn mips_detect_memory(memsize: usize) {
    *NPAGE.borrow_mut() = memsize / 4096;
    println!(
        "Memory Size: {} KiB; Page Number: {}.",
        memsize / 1024,
        *NPAGE.borrow()
    );
}

/// Alloc needed memorys in kernel mode. Only use it before the page-manager
/// is ready to use.
///
/// The `freemem` will be updated to mark the memory used.
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

/// Alloc memories needed for the [PAGES] and reserve 512 * 4KB spaces for the
/// buddy system to alloc.
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
    crate::BUDDY_ALLOCATOR.init(page_start, 512 * PAGE_SIZE);

    println!("Heaps are to the memeory 0x{:x}", freemem);
    debugln!("> pmap.rs: mips vm init success");
}

/// Init all the pages. Mark the pages below the `freemem` as *used*.
pub fn page_init(freemem: &mut usize) {
    let pages = *PAGES.borrow_mut();

    *freemem = ROUND!(*freemem; PAGE_SIZE);

    let mut page_id = 0;
    while page_id < *NPAGE.borrow() && page_id << PGSHIFT < PADDR!(*freemem) {
        unsafe { ((*pages.wrapping_add(page_id)).data).pp_ref = 1 };
        page_id += 1;
    }

    debugln!("> pmap.rs: pages are used for {}", page_id);

    while page_id < *NPAGE.borrow() {
        unsafe { ((*pages.wrapping_add(page_id)).data).pp_ref = 0 };
        (*PAGE_FREE_LIST.borrow_mut()).insert_head(pages.wrapping_add(page_id));
        page_id += 1;
    }
}

/// Alloc a page. Return the page's address or an error if no free pages
/// available.
pub fn page_alloc() -> Result<*mut PageNode, KError> {
    match (*PAGE_FREE_LIST.borrow_mut()).pop_head() {
        None => Err(KError::NoMem),
        Some(pp) => {
            let entry_p = page2kva!(pp, *PAGES.borrow(); PageNode) as *mut u8;
            unsafe { ptr::write_bytes(entry_p, 0, PAGE_SIZE) }
            Ok(pp)
        }
    }
}

/// Free a page. The `pp_ref` *shall* be *zero* before freeing it.
///
/// Otherwise, the kernel will panic.
pub fn page_free(page: &mut *mut PageNode) {
    assert_eq!(0, unsafe { **page }.data.pp_ref);
    (*PAGE_FREE_LIST.borrow_mut()).insert_head(*page);
}

/// Decrease the page's `pp_ref`. If all reference is removed, the page will
/// be freed.
pub fn page_decref(page: &mut *mut PageNode) {
    assert!(unsafe { **page }.data.pp_ref > 0);
    unsafe { (**page).data.pp_ref -= 1 };
    if unsafe { **page }.data.pp_ref == 0 {
        page_free(page);
    }
}

/// Walk the current page table to find the virtual address `va`'s page table
/// entry ([Pte]).
///
/// If the pte is not found or is invalid, the parameter `create` determines
/// whether to create a new entry or just return an error.
///
/// # Return
///
/// Return the pte address found with a `Ok` wrapper or the [KError] with an
/// `Err` wrapper if failed.
pub fn pgdir_walk(pgdir: *mut Pde, va: usize, create: bool) -> Result<*mut Pte, KError> {
    let pgdir_entryp = (pgdir as u32 + (PDX!(va) * size_of::<Pde>()) as u32) as *mut Pte;
    if 0 == PTE_V & unsafe { *(pgdir_entryp as *const Pte) } {
        // Not Valid!
        if create {
            let pp = page_alloc()?;
            let entry = (PTE_ADDR!(page2pa!(pp, *PAGES.borrow(); PageNode)) as Pte
                | PTE_C_CACHEABLE
                | PTE_V);
            unsafe {
                ptr::write(pgdir_entryp, entry);
                (*pp).data.pp_ref = 1;
            }
        } else {
            return Ok(ptr::null_mut());
        }
    }
    let entry = unsafe { *pgdir_entryp };
    Ok((KADDR!(PTE_ADDR!(entry)) + (PTX!(va) * size_of::<Pte>()) as u32) as *mut Pte)
}

/// Map the physical `page` to the virtual address `va`. The permission bits
/// will be set to `perm | PTE_C_CACHEABLE | PTE_V`.
///
/// If there is already a page mapped at `va`, [page_remove] will be invoked
/// to unmap it.
pub fn page_insert(
    pgdir: *mut Pde,
    va: usize,
    asid: u32,
    perm: u32,
    page: *mut PageNode,
) -> Result<(), KError> {
    let pp = page; // deceits
    if let Ok(pte) = pgdir_walk(pgdir, va, false) {
        if !pte.is_null() && unsafe { *pte & PTE_V } != 0 {
            if pp as usize != pa2page!(unsafe { *pte }, *PAGES.borrow(); PageNode) {
                page_remove(pgdir, va, asid);
            } else {
                tlb_invalidate(asid, va);
                let entry =
                    page2pa!(pp, *PAGES.borrow(); PageNode) as Pte | perm | PTE_C_CACHEABLE | PTE_V;
                unsafe {
                    ptr::write(pte, entry);
                }
                return Ok(());
            }
        }
    }

    tlb_invalidate(asid, va);
    let pte = pgdir_walk(pgdir, va, true)?;
    let entry = page2pa!(pp, *PAGES.borrow(); PageNode) as Pte | perm | PTE_C_CACHEABLE | PTE_V;
    unsafe {
        ptr::write(pte, entry);
        (*pp).data.pp_ref += 1;
    }

    Ok(())
}

/// Look up the Page that virtual address `va` map to. Return the page and the
/// page table entry together if the page is found and is valid.
pub fn page_lookup(pgdir: *mut Pde, va: usize) -> Option<(*mut PageNode, *mut Pte)> {
    if let Ok(pte) = pgdir_walk(pgdir, va, false) {
        if pte.is_null() || unsafe { *pte } & PTE_V == 0 {
            None
        } else {
            let entry = unsafe { *pte };
            let pp = pa2page!( entry , *PAGES.borrow(); PageNode) as *mut PageNode;
            Some((pp, pte))
        }
    } else {
        None
    }
}

/// Unmap the physical page at virtual address `va`.
pub fn page_remove(pgdir: *mut Pde, va: usize, asid: u32) {
    if let Some((mut pp, pte)) = page_lookup(pgdir, va) {
        page_decref(&mut pp);
        unsafe { ptr::write(pte, 0) };
        tlb_invalidate(asid, va);
    }
}
