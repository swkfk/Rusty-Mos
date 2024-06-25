//! Safe linked list managed by arrays.

/// Aligned arraies wrapper.
#[repr(align(4096))]
pub struct Aligned<T, const LEN: usize>(pub [T; LEN]);

/// The list itself. Maintain a head and tail and the node array.
///
/// # Generic
///
/// The const `LEN` means the node count. It should be specified at the compile
/// time.
///
/// # Invariant
///
/// Both `head` and `tail` *shall* be `None` together or not be `None`
/// together.
///
/// See Also: [ArrayLinkNode]
#[repr(C)]
pub struct ArrayLinkedList<const LEN: usize> {
    /// The array storing the linking field of each nodes.
    array: [ArrayLinkNode; LEN],
    /// The index of the first node. `None` means an empty node.
    head: Option<usize>,
    /// The index of the last node. `None` means an empty node.
    tail: Option<usize>,
}

impl<const LEN: usize> Default for ArrayLinkedList<LEN> {
    /// Default constructions.
    fn default() -> Self {
        Self::new()
    }
}

impl<const LEN: usize> ArrayLinkedList<LEN> {
    /// Create a new link list and initialize the array.
    pub const fn new() -> Self {
        Self {
            array: [ArrayLinkNode::new(); LEN],
            head: None,
            tail: None,
        }
    }

    /// Judge whether the list is empty.
    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

    /// Insert the index `item` node into the head.
    pub fn insert_head(&mut self, item: usize) {
        match self.head {
            None => {
                self.head = Some(item);
                self.tail = Some(item);
                self.array[item].next = None;
                self.array[item].prev = None;
            }
            Some(i) => {
                self.array[i].prev = Some(item);
                self.array[item].next = Some(i);
                self.array[item].prev = None;
                self.head = Some(item);
            }
        }
    }

    /// Peek the first node.
    pub fn peek_head(&self) -> Option<usize> {
        self.head
    }

    /// Get the first node. And remove it from the list if the list is not
    /// empty.
    pub fn pop_head(&mut self) -> Option<usize> {
        match self.head {
            None => None,
            Some(i) => {
                self.remove(i);
                Some(i)
            }
        }
    }

    /// Insert the index `item` node into the tail.
    pub fn insert_tail(&mut self, item: usize) {
        if self.head.is_none() {
            self.insert_head(item);
            return;
        }
        self.array[self.tail.unwrap()].next = Some(item);
        self.array[item].prev = self.tail;
        self.array[item].next = None;
        self.tail = Some(item);
    }

    /// Remove the index `item` node from the list. This method requires that
    /// the node was *in* the list.
    pub fn remove(&mut self, item: usize) {
        match self.array[item].prev {
            None => self.head = self.array[item].next,
            Some(i) => self.array[i].next = self.array[item].next,
        }
        match self.array[item].next {
            None => self.tail = self.array[item].prev,
            Some(i) => self.array[i].prev = self.array[item].prev,
        }
        self.array[item].next = None;
        self.array[item].prev = None;
    }

    /// Judge whether the node is in the list.
    pub fn contains(&self, item: usize) -> bool {
        if self.array[item].prev.is_none()
            && self.array[item].next.is_none()
            && (self.head != Some(item) || self.tail != Some(item))
        {
            return false;
        }
        true
    }
}

/// Linking field node. Contains a `next` and `prev` field.
#[derive(Clone, Copy, Default, Debug)]
pub struct ArrayLinkNode {
    /// Next node. If `None`, this node is the last node.
    pub next: Option<usize>,
    /// Previous node. If `None`, this node is the first node.
    pub prev: Option<usize>,
}

impl ArrayLinkNode {
    /// Default constructions.
    pub const fn new() -> ArrayLinkNode {
        ArrayLinkNode {
            prev: None,
            next: None,
        }
    }
}
