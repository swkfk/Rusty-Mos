//! Global allocator use the buddy system.
//!
//! The *category* count can be specified via a const generic. The smallest
//! alloc unit is one page while the largest is (2 ^ (category - 1)) pages.
//!
//! This allocator can be used after the page-memory manager is initilized.

use core::{
    alloc::GlobalAlloc,
    cmp::{max, min},
    mem,
    ptr::null_mut,
};

use crate::{
    debugln,
    memory::pmap::{PageList, PageNode, PAGES},
    memory::regions::PAGE_SIZE,
    pa2page, page2kva,
    utils::sync_ref_cell::SyncImplRef,
    PADDR,
};

/// The buddy system core data structure.
struct BuddyInner<const CCOUNT: usize> {
    /// Free page lists. Only the first page will be in the list.
    ///
    /// The `i`-th category holds (2^i) pages each.
    free_list: [PageList; CCOUNT],
    /// The start page address used in the system.
    page_start: *mut PageNode,
}

/// For it can be used globally.
unsafe impl<const CCOUNT: usize> Send for BuddyInner<CCOUNT> {}

impl<const CCOUNT: usize> Default for BuddyInner<CCOUNT> {
    /// Default constructions.
    fn default() -> Self {
        Self::new()
    }
}

impl<const CCOUNT: usize> BuddyInner<CCOUNT> {
    /// Create a brand-new buddy contents.
    const fn new() -> Self {
        Self {
            free_list: [PageList::new(); CCOUNT],
            page_start: null_mut(),
        }
    }

    /// Initialize the free list. The pages will be inserted into the topest
    /// category and the remained into one-level-lower category. As to the
    /// category *zero*.
    ///
    /// Only after this function invoked, the system can be used.
    fn init(&mut self, page_start: *mut PageNode, size: usize) {
        self.page_start = page_start;
        let page_count = size / PAGE_SIZE;
        let mut index = 0;
        for i in (0..CCOUNT).rev() {
            while index < page_count {
                self.free_list[i].insert_head(page_start.wrapping_add(index));
                index += 1 << i;
            }
        }
    }
}

impl<const CCOUNT: usize> BuddyInner<CCOUNT> {
    /// Buddy Alloc.
    ///
    /// This method will search the specified category's free list first. If
    /// found, the page will be returned.
    ///
    /// Otherwise, this method will search the higher category, until a free
    /// item was found. In this situation, the pages will be splitted and
    /// inserted into the lower category's free list one by one.
    ///
    /// If no pages can be found, a *null* pointer will be returned.
    fn alloc(&mut self, layout: core::alloc::Layout) -> *mut u8 {
        let page_count =
            (max(layout.size(), layout.align()).div_ceil(PAGE_SIZE)).next_power_of_two();
        let page_count = max(page_count, 1);
        let category = page_count.trailing_zeros() as usize;
        debugln!(
            "> ALLOC: alloc {} bytes with align {}, page needed is {} while the category is {}",
            layout.size(),
            layout.align(),
            page_count,
            category
        );

        for i in category..CCOUNT {
            if self.free_list[i].empty() {
                continue;
            }
            let allocated = self.free_list[i].pop_head().unwrap();
            let page = allocated as *mut PageNode;
            for j in category..i {
                self.free_list[j].insert_head(page.wrapping_add(1 << j));
            }
            let kva = page2kva!(allocated, *PAGES.borrow(); PageNode);
            debugln!(
                "> ALLOC: allocated: page at 0x{:x}, kva at 0x{:x}, index: {}",
                allocated as usize,
                kva,
                (allocated as usize - self.page_start as usize) / mem::size_of::<PageNode>()
            );
            return kva as *mut u8;
        }

        null_mut()
    }

    /// Buddy Dealloc.
    ///
    /// The page will be inserted into its category's free list if no buddy is
    /// found.
    ///
    /// Otherwise, the buddy will be removed from the free list. They will
    /// be merged into a larger item and be inserted into one-level-higher
    /// category.
    ///
    /// The process above will be performed recursively unless reached the
    /// toppest category.
    fn dealloc(&mut self, ptr: *mut u8, layout: core::alloc::Layout) {
        let p = pa2page!(PADDR!(ptr as usize), *PAGES.borrow(); PageNode);
        let mut page_index = (p - self.page_start as usize) / mem::size_of::<PageNode>();
        let page_count =
            (max(layout.size(), layout.align()).div_ceil(PAGE_SIZE)).next_power_of_two();
        let page_count = max(page_count, 1);
        let category = page_count.trailing_zeros() as usize;
        debugln!(
            "> FREE: dealloc {} bytes with align {}, at 0x{:x}, index: {}",
            layout.size(),
            layout.align(),
            ptr as usize,
            page_index
        );

        'iter_cate: for i in category..CCOUNT {
            let buddy = page_index ^ (1 << i);
            let mut list = self.free_list[i].head;
            while i != CCOUNT - 1 && !list.is_null() {
                if buddy == (list as usize - self.page_start as usize) / mem::size_of::<PageNode>()
                {
                    PageList::remove(self.page_start.wrapping_add(buddy));
                    debugln!("> FREE: remove page {}", buddy);
                    page_index = min(page_index, buddy);
                    continue 'iter_cate;
                }
                unsafe { list = (*list).next };
            }
            // Not find or reach the end
            debugln!("> FREE: insert page {} into category {}", page_index, i);
            self.free_list[i].insert_head(self.page_start.wrapping_add(page_index));
            break;
        }
    }
}

/// The *real* allocator provided to the Rust. A wrapper.
pub struct BuddyAllocator<const CCOUNT: usize>(SyncImplRef<BuddyInner<CCOUNT>>);

impl<const CCOUNT: usize> Default for BuddyAllocator<CCOUNT> {
    /// Default constructions.
    fn default() -> Self {
        Self::new()
    }
}

impl<const CCOUNT: usize> BuddyAllocator<CCOUNT> {
    /// Just build a inner struct.
    pub const fn new() -> Self {
        Self(SyncImplRef::new(BuddyInner::<CCOUNT>::new()))
    }

    /// Just initialize the inner struct.
    pub fn init(&self, page_start: *mut PageNode, size: usize) {
        self.0.borrow_mut().init(page_start, size)
    }
}

unsafe impl<const CCOUNT: usize> GlobalAlloc for BuddyAllocator<CCOUNT> {
    /// Do alloc.
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.0.borrow_mut().alloc(layout)
    }

    /// Do dealloc.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.0.borrow_mut().dealloc(ptr, layout)
    }
}
