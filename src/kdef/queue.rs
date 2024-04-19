//! Link List implemented with Rust, which is similar to the kernel queue of the mos / Linux
//!
//! The linking-filed in LinkNode struct contains two raw pointers:
//! - `next`: pointing to the next LinkNode
//! - `prev`: pointing to the previous LinkNode's `next` field
//!

use core::ptr;

/// The head struct of the LinkList
///
/// See also: [LinkNode]
///
/// # Generics
/// The type `T` indicates the data stored in the link list.
pub struct LinkList<T: Copy> {
    /// Pointing to the first node of this link list.
    /// The list is empty if and only if the `head` is null.
    pub head: *mut LinkNode<T>,
}

/// The node struct of the LinkList
///
/// See also: [LinkList]
///
/// # Generics
/// The type `T` indicates the data stored in the link list.
#[derive(Clone, Copy)]
pub struct LinkNode<T: Copy> {
    /// Pointing the next node. If this is the last node, the field will be null.
    pub next: *mut LinkNode<T>,
    /// Pointing the previous node's `next` field.
    /// If this is the first node, the field will point to the head's `head` field.
    pub prev: *mut *mut LinkNode<T>,
    /// The data stored in the link list, with the type `T`.
    pub data: T,
}

impl<T: Copy> Default for LinkList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy> LinkList<T> {
    /// Create an empty link list with its head null.
    pub const fn new() -> LinkList<T> {
        LinkList {
            head: ptr::null_mut(),
        }
    }

    /// Judge whether this list is empty.
    pub fn empty(&self) -> bool {
        self.head.is_null()
    }

    /// Insert a node to the head of the list
    ///
    /// # Safety
    /// The parameter `item` *SHALL* be mutably-visitable!
    pub unsafe fn insert_head(&mut self, item: *mut LinkNode<T>) {
        // If this list is not empty, the previous head's `prev` will be updated
        if !self.empty() {
            (*item).next = self.head;
            (*(self.head)).prev = ptr::addr_of_mut!((*item).next);
        }
        (*item).prev = ptr::addr_of_mut!(self.head);
        self.head = item;
    }

    /// Get the first node of this list and removce it
    ///
    /// The return value will be `None` is the list is empty.
    ///
    /// # Safety
    /// All things in the list *SHALL* be valid!
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

    /// Remove a specified node from the list contains this node.
    ///
    /// # Safety
    /// The parameter `item` *SHALL* be mutably-visitable and *SHALL* be in an valid link list!
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
    /// Create an empty link list node with its linking-field all null.
    pub const fn new(data: T) -> LinkNode<T> {
        LinkNode {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            data,
        }
    }
}
