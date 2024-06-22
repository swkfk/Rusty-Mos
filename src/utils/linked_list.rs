//! Link List implemented with Rust, which is similar to the kernel queue of
//! the mos / Linux
//!
//! The `LinkList` is a simple-linked-list, while the `TailLinkList` is a
//! tail-linked-list.
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
#[derive(Clone, Copy)]
pub struct LinkList<T: Copy> {
    /// Pointing to the first node of this link list.
    /// The list is empty if and only if the `head` is null.
    pub head: *mut LinkNode<T>,
}

/// The node struct of the LinkList
///
/// See also: [LinkList], [TailLinkList]
///
/// # Generics
/// The type `T` indicates the data stored in the link list.
#[derive(Clone, Copy)]
pub struct LinkNode<T: Copy> {
    /// Pointing the next node. If this is the last node, the field will be
    /// null.
    pub next: *mut LinkNode<T>,
    /// Pointing the previous node's `next` field. If this is the first node,
    /// the field will point to the head's `head` field.
    pub prev: *mut *mut LinkNode<T>,
    /// The data stored in the link list, with the type `T`.
    pub data: T,
}

impl<T: Copy> Default for LinkList<T> {
    /// Constructor for the default.
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
    pub fn insert_head(&mut self, item: *mut LinkNode<T>) {
        // If this list is not empty, the previous head's `prev` will be updated
        let item_p = item; // deceits
        if !self.empty() {
            unsafe {
                (*item_p).next = self.head;
                (*(self.head)).prev = ptr::addr_of_mut!((*item_p).next);
            }
        }
        unsafe { (*item_p).prev = ptr::addr_of_mut!(self.head) }
        self.head = item_p;
    }

    /// Get the first node of this list and removce it
    ///
    /// The return value will be `None` is the list is empty.
    ///
    /// # Safety
    /// All things in the list *SHALL* be valid!
    pub fn pop_head(&mut self) -> Option<*mut LinkNode<T>> {
        match self.empty() {
            true => None,
            false => {
                let item = self.head;
                Self::remove(item);
                Some(item)
            }
        }
    }

    /// Remove a specified node from the list contains this node.
    ///
    /// # Safety
    /// The parameter `item` *SHALL* be mutably-visitable and *SHALL* be in an
    /// valid link list!
    pub fn remove(item: *mut LinkNode<T>) {
        let item_p = item;
        if !unsafe { *item_p }.next.is_null() {
            unsafe { (*((*item_p).next)).prev = (*item_p).prev }
        }
        unsafe { *((*item_p).prev) = (*item_p).next }
        unsafe { (*item_p).next = ptr::null_mut() }
        unsafe { (*item_p).prev = ptr::null_mut() }
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
