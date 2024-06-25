//! `RefCell` wrapper. It implements `Sync` for global use.

use core::cell::{Ref, RefCell, RefMut};

/// The wrapper.
pub struct SyncImplRef<T: ?Sized>(pub RefCell<T>);

/// For global use. The OS kernel itself is single-thread.
unsafe impl<T: ?Sized> Sync for SyncImplRef<T> {}

impl<T> SyncImplRef<T> {
    /// Default constructions. Put the target into the *interior mutability*
    /// container.
    pub const fn new(inner: T) -> Self {
        Self(RefCell::new(inner))
    }
}

impl<T: ?Sized> SyncImplRef<T> {
    /// A convenient alias for borrow.
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    /// A convenient alias for mut borrow.
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
}
