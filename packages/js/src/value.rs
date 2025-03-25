use core::num::NonZeroU32;

use super::IsValue;

/// A JS value guaranteed to not be `undefined`
#[repr(transparent)]
pub struct Value {
    index: NonZeroU32,
}

impl Value {
    pub(super) fn index(&self) -> NonZeroU32 {
        self.index
    }

    pub fn to_u32(&self) -> u32 {
        unsafe extern "C" {
            #[link_name = "$Value$to_u32"]
            fn ff(index: NonZeroU32) -> u32;
        }

        unsafe { ff(self.index()) }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        unsafe extern "C" {
            #[link_name = "$Value$clone"]
            fn ff(index: NonZeroU32) -> NonZeroU32;
        }

        Self {
            index: unsafe { ff(self.index) },
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe extern "C" {
            #[link_name = "$Value$drop"]
            fn ff(index: NonZeroU32);
        }
        unsafe { ff(self.index) }
    }
}

unsafe impl IsValue for Value {}
