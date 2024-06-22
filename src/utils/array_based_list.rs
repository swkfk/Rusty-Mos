#[repr(align(4096))]
pub struct Aligned<T, const LEN: usize>(pub [T; LEN]);

#[repr(C)]
pub struct ArrayLinkedList<const LEN: usize> {
    array: [ArrayLinkNode; LEN],
    head: Option<usize>,
    tail: Option<usize>,
}

impl<const LEN: usize> Default for ArrayLinkedList<LEN> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const LEN: usize> ArrayLinkedList<LEN> {
    pub fn new() -> Self {
        Self {
            array: [ArrayLinkNode::default(); LEN],
            head: None,
            tail: None,
        }
    }

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

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

    pub fn pop_head(&mut self) -> Option<usize> {
        match self.head {
            None => None,
            Some(i) => {
                self.head = self.array[i].next;
                if self.head.is_none() {
                    self.tail = None;
                }
                self.array[i].next = None;
                Some(i)
            }
        }
    }

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

    pub fn remove(&mut self, item: usize) {
        match self.array[item].prev {
            None => self.head = self.array[item].next,
            Some(i) => self.array[i].next = self.array[item].next,
        }
        if self.array[item].next.is_none() {
            self.tail = self.array[item].prev
        }
        self.array[item].next = None;
        self.array[item].prev = None;
    }
}

#[derive(Clone, Copy, Default)]
pub struct ArrayLinkNode {
    pub next: Option<usize>,
    pub prev: Option<usize>,
    pub data: usize,
}

impl ArrayLinkNode {
    pub const fn new(data: usize) -> ArrayLinkNode {
        ArrayLinkNode {
            prev: None,
            next: None,
            data,
        }
    }
}
