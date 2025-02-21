use std::cell::UnsafeCell;

pub struct SyncUnsafeCell<T>(pub UnsafeCell<T>);
unsafe impl<T> Sync for SyncUnsafeCell<T> {}
unsafe impl<T> Send for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    pub const fn new(a: T) -> Self {
        Self(UnsafeCell::new(a))
    }
}
