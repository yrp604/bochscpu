use std::cell::UnsafeCell;
use std::mem;

pub struct SyncUnsafeCell<T>(pub UnsafeCell<T>);
unsafe impl<T> Sync for SyncUnsafeCell<T> {}
unsafe impl<T> Send for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    pub const fn new(a: T) -> Self {
        Self(UnsafeCell::new(a))
    }
}

pub(crate) unsafe fn ptr_to_ref<T>(ptr: *const T) -> &'static T {
    unsafe { mem::transmute(ptr) }
}

pub(crate) unsafe fn ptr_to_ref_mut<T>(ptr: *mut T) -> &'static mut T {
    unsafe { mem::transmute(ptr) }
}
