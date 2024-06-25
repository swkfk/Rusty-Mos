//! Provide a bitmap and related opeartions.

/// The bitmap itself. Contains the map and assistant field.
///
/// The `used` maintains the count of bit used to speed up the `empty` or
/// `full` judgement.
///
/// The algorithm of searching the next empty bit is as follows:
///
/// We maintain a `pointer`, the search will from the `pointer` one-by-one.
///
/// # Generic
///
/// The const `CCOUNT` marks the count of bit maintains. The bitmap can hold
/// (CCOUNT * 8) bits.
#[derive(Debug)]
pub struct Bitmap<const COUNT: usize> {
    /// The bitmap itself.
    bitmap: [u8; COUNT],
    /// Used bits count.
    used: usize,
    /// Search pointer.
    pointer: usize,
}

impl<const COUNT: usize> Default for Bitmap<COUNT> {
    /// Default constructions.
    fn default() -> Self {
        Self::new()
    }
}

impl<const COUNT: usize> Bitmap<COUNT> {
    /// Create a new bitmap and initialize the field as *zero*.
    pub const fn new() -> Self {
        Bitmap {
            bitmap: [0; COUNT],
            used: 0,
            pointer: 0,
        }
    }

    /// Judge whether the bitmap is empty.
    pub fn empty(&self) -> bool {
        self.used == COUNT << 3
    }

    /// To see whether the `index` is *not* used.
    pub fn peek(&self, index: usize) -> bool {
        assert!((0..COUNT << 3).contains(&index), "Bitmap out of index");
        (self.bitmap[index >> 3] >> (index & 0x7)) & 1 == 0
    }

    /// Alloc a new bit if available. A `None` will be returned if no bit rest.
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

    /// Free the specified bit. Double free is not allowed.
    pub fn free(&mut self, index: usize) {
        assert!(!self.peek(index), "Bitmap double free");
        self.bitmap[index >> 3] &= !(1 << (index & 0x7));
        self.used -= 1;
    }
}
