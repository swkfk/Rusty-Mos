#[derive(Debug)]
pub struct Bitmap<const COUNT: usize> {
    bitmap: [u8; COUNT],
    used: usize,
    pointer: usize,
}

impl<const COUNT: usize> Default for Bitmap<COUNT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const COUNT: usize> Bitmap<COUNT> {
    pub const fn new() -> Self {
        Bitmap {
            bitmap: [0; COUNT],
            used: 0,
            pointer: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.used == COUNT << 3
    }

    pub fn peek(&self, index: usize) -> bool {
        assert!((0..COUNT << 3).contains(&index), "Bitmap out of index");
        (self.bitmap[index >> 3] >> (index & 0x7)) & 1 == 0
    }

    pub fn alloc(&mut self) -> Option<usize> {
        if self.empty() {
            return None;
        }
        if let Some(i) = (self.pointer..(COUNT << 3))
            .chain(0..self.pointer)
            .find(|index| self.peek(*index))
        {
            self.bitmap[i >> 3] |= 1 << (i & 0x7);
            self.pointer = i;
            self.used += 1;
            return Some(i);
        }
        None
    }

    pub fn free(&mut self, index: usize) {
        assert!(!self.peek(index), "Bitmap double free");
        self.bitmap[index >> 3] &= !(1 << (index & 0x7));
        self.used -= 1;
    }
}
