use core::ptr;

pub struct LinkList<T: Copy> {
    head: *mut LinkNode<T>,
}

#[derive(Clone, Copy)]
pub struct LinkNode<T: Copy> {
    next: *mut LinkNode<T>,
    prev: *mut *mut LinkNode<T>,
    pub data: T,
}

impl<T: Copy> Default for LinkList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy> LinkList<T> {
    pub const fn new() -> LinkList<T> {
        LinkList {
            head: ptr::null_mut(),
        }
    }

    pub fn empty(&self) -> bool {
        self.head.is_null()
    }

    /// # Safety
    ///
    pub unsafe fn insert_head(&mut self, item: *mut LinkNode<T>) {
        if !self.empty() {
            *item = *self.head as LinkNode<T>;
        }
        self.head = item;
    }

    pub fn pop_head(&mut self) -> Option<*mut LinkNode<T>> {
        match self.empty() {
            true => None,
            false => {
                let item = self.head;
                self.head = unsafe { (*item).next };
                Some(item)
            }
        }
    }

    /// # Safety
    ///
    pub unsafe fn remove(item: *mut LinkNode<T>) {
        let item = *item;
        if !item.next.is_null() {
            (*(item.next)).prev = item.prev;
        }
        *(item.prev) = item.next;
    }
}

impl<T: Copy> LinkNode<T> {
    pub const fn new(data: T) -> LinkNode<T> {
        LinkNode {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            data,
        }
    }
}
