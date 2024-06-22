use core::cell::{Ref, RefCell, RefMut};

pub struct SyncImplRef<T: ?Sized>(pub RefCell<T>);

unsafe impl<T: ?Sized> Sync for SyncImplRef<T> {}

impl<T> SyncImplRef<T> {
    pub const fn new(inner: T) -> Self {
        Self(RefCell::new(inner))
    }
}

impl<T: ?Sized> SyncImplRef<T> {
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
}
