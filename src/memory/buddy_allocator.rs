use core::{
    alloc::GlobalAlloc,
    cmp::{max, min},
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

struct BuddyInner<const CCOUNT: usize> {
    free_list: [PageList; CCOUNT],
    page_start: *mut PageNode,
}

unsafe impl<const CCOUNT: usize> Send for BuddyInner<CCOUNT> {}

impl<const CCOUNT: usize> Default for BuddyInner<CCOUNT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CCOUNT: usize> BuddyInner<CCOUNT> {
    const fn new() -> Self {
        Self {
            free_list: [PageList::new(); CCOUNT],
            page_start: null_mut(),
        }
    }

    fn init(&mut self, page_start: *mut PageNode, size: usize) {
        self.page_start = page_start;
        let page_count = size / PAGE_SIZE;
        let mut index = 0;
        for i in (0..CCOUNT).rev() {
            while index < page_count {
                unsafe { self.free_list[i].insert_head(page_start.add(index)) };
                index += 1 << i;
            }
        }
    }
}

impl<const CCOUNT: usize> BuddyInner<CCOUNT> {
    fn alloc(&mut self, layout: core::alloc::Layout) -> *mut u8 {
        let page_count = (max(layout.size(), layout.align()) / PAGE_SIZE).next_power_of_two();
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
            let mut page = allocated as *mut PageNode;
            for j in category..=i {
                page = page.wrapping_add(1 << j);
                self.free_list[j].insert_head(page);
            }
            let kva = page2kva!(allocated, *PAGES.borrow(); PageNode);
            debugln!(
                "> ALLOC: allocated: page at 0x{:x}, kva at 0x{:x}",
                allocated as usize,
                kva
            );
            return kva as *mut u8;
        }

        null_mut()
    }

    fn dealloc(&mut self, ptr: *mut u8, layout: core::alloc::Layout) {
        let p = pa2page!(PADDR!(ptr as usize), *PAGES.borrow(); PageNode) as *mut PageNode;
        let mut page_index = unsafe { p.offset_from(self.page_start) } as usize;
        let page_count = (max(layout.size(), layout.align()) / PAGE_SIZE).next_power_of_two();
        let page_count = max(page_count, 1);
        let category = page_count.trailing_zeros() as usize;

        'iter_cate: for i in category..CCOUNT {
            let buddy = page_index ^ (1 << i);
            let mut list = self.free_list[i].head;
            while i != CCOUNT - 1 && !list.is_null() {
                if buddy == unsafe { list.offset_from(self.page_start) } as usize {
                    unsafe { PageList::remove(self.page_start.add(buddy)) };
                    page_index = min(page_index, buddy);
                    continue 'iter_cate;
                }
                unsafe { list = (*list).next };
            }
            // Not find or reach the end
            unsafe { self.free_list[i].insert_head(self.page_start.add(page_index)) };
            break;
        }
    }
}

pub struct BuddyAllocator<const CCOUNT: usize>(SyncImplRef<BuddyInner<CCOUNT>>);

impl<const CCOUNT: usize> Default for BuddyAllocator<CCOUNT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CCOUNT: usize> BuddyAllocator<CCOUNT> {
    pub const fn new() -> Self {
        Self(SyncImplRef::new(BuddyInner::<CCOUNT>::new()))
    }

    /// # Safety
    ///
    pub unsafe fn init(&self, page_start: *mut PageNode, size: usize) {
        self.0.borrow_mut().init(page_start, size)
    }
}

unsafe impl<const CCOUNT: usize> GlobalAlloc for BuddyAllocator<CCOUNT> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.0.borrow_mut().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.0.borrow_mut().dealloc(ptr, layout)
    }
}
