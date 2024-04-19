use core::ptr;

pub struct LinkList<T: Copy> {
    pub head: *mut LinkNode<T>,
}

#[derive(Clone, Copy)]
pub struct LinkNode<T: Copy> {
    pub next: *mut LinkNode<T>,
    pub prev: *mut *mut LinkNode<T>,
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
    pub unsafe fn insert_head(&mut self, mut item: *mut LinkNode<T>) {
        if !self.empty() {
            (*item).next = self.head;
            (*(self.head)).prev = ptr::addr_of_mut!((*item).next);
        }
        (*item).prev = ptr::addr_of_mut!(self.head);
        self.head = item;
    }

    /// # Safety
    ///
    pub unsafe fn pop_head(&mut self) -> Option<*mut LinkNode<T>> {
        match self.empty() {
            true => None,
            false => {
                let item = self.head;
                if !(*item).next.is_null() {
                    (*((*item).next)).prev = (*item).prev;
                }
                self.head = (*item).next;
                (*item).next = ptr::null_mut();
                (*item).prev = ptr::null_mut();
                Some(item)
            }
        }
    }

    /// # Safety
    ///
    pub unsafe fn remove(item: *mut LinkNode<T>) {
        let mut item = *item;
        if !item.next.is_null() {
            (*(item.next)).prev = item.prev;
        }
        *(item.prev) = item.next;
        item.next = ptr::null_mut();
        item.prev = ptr::null_mut();
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
