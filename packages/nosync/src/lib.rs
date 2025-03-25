//! Access static variables in abscence of threads, signal handlers and any other form of preemption

#![no_std]

use core::cell::{Cell, RefCell, RefMut, UnsafeCell};

/// Owned static variable
pub struct Owned<T> {
    inner: UnsafeCell<T>,
    taken: Cell<bool>,
}

impl<T> Owned<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
            taken: Cell::new(false),
        }
    }

    pub fn take(&self) -> Option<&mut T> {
        if self.taken.get() {
            None
        } else {
            self.taken.set(true);
            unsafe { Some(&mut *self.inner.get()) }
        }
    }
}

unsafe impl<T> Sync for Owned<T> {}

/// Shared static variable
pub struct Shared<T> {
    inner: RefCell<T>,
}

impl<T> Shared<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}

unsafe impl<T> Sync for Shared<T> {}
