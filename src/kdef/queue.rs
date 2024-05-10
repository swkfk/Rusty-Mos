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

use core::ptr::{self, addr_of_mut, null_mut};

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

/// The LinkList with its **tail** recorded
///
/// See also: [LinkNode]
///
/// # Attention
/// Before use the TailLinkList object, the method `enable` **SHALL** be called
/// first.
///
/// # Generics
/// The type `T` indicates the data stored in the link list.
pub struct TailLinkList<T: Copy> {
    pub head: *mut LinkNode<T>,
    pub tail: *mut *mut LinkNode<T>,
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
    pub unsafe fn remove(item: *mut LinkNode<T>) {
        if !(*item).next.is_null() {
            (*((*item).next)).prev = (*item).prev;
        }
        *((*item).prev) = (*item).next;
        (*item).next = ptr::null_mut();
        (*item).prev = ptr::null_mut();
    }
}

impl<T: Copy> Default for TailLinkList<T> {
    /// Constructor for the default.
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy> TailLinkList<T> {
    /// Create an empty TailLinkList object. Not useable yet.
    pub const fn new() -> TailLinkList<T> {
        TailLinkList {
            head: null_mut(),
            tail: null_mut(),
        }
    }

    /// Judge whether the list is empty.
    pub fn empty(&self) -> bool {
        self.head.is_null()
    }

    /// Make the list useable after the construction.
    pub fn enable(&mut self) {
        self.tail = addr_of_mut!(self.head);
    }

    /// Insert the item into the head of the list.
    ///
    /// # Safety
    /// The `item` passed **SHALL NOT** be null.
    pub unsafe fn insert_head(&mut self, item: *mut LinkNode<T>) {
        (*item).next = self.head;
        if !(*item).next.is_null() {
            (*(self.head)).prev = ptr::addr_of_mut!((*item).next);
        } else {
            self.tail = ptr::addr_of_mut!((*item).next);
        }
        self.head = item;
        (*item).prev = ptr::addr_of_mut!(self.head);
    }

    /// Insert the item into the tail of the list.
    ///
    /// # Safety
    /// The `item` passed **SHALL NOT** be null.
    pub unsafe fn insert_tail(&mut self, item: *mut LinkNode<T>) {
        (*item).next = null_mut();
        (*item).prev = self.tail;
        *self.tail = item;
        self.tail = ptr::addr_of_mut!((*item).next);
    }

    /// Get the head and remove it if the list is not empty.
    ///
    /// # Return
    /// `None` -- The list is empty.
    /// `Some(item)` -- Otherwise
    ///
    /// # Safety
    /// The list **SHALL** be valid.
    pub unsafe fn pop_head(&mut self) -> Option<*mut LinkNode<T>> {
        match self.empty() {
            true => None,
            false => {
                let item = self.head;
                self.remove(item);
                Some(item)
            }
        }
    }

    /// Remove a specified node from the list contains this node.
    ///
    /// # Safety
    /// The parameter `item` *SHALL* be mutably-visitable and *SHALL* be in an
    /// valid link list!
    pub unsafe fn remove(&mut self, item: *mut LinkNode<T>) {
        if !(*item).next.is_null() {
            (*((*item).next)).prev = (*item).prev;
        } else {
            self.tail = (*item).prev;
        }
        *((*item).prev) = (*item).next;
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
