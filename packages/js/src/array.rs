use core::num::NonZeroU32;

use crate::{Integer, Object, Value};

inheritance!(Array: Object);

impl Array {
    pub fn new() -> Self {
        unsafe extern "C" {
            #[link_name = "$Array$constructor"]
            fn ff() -> Array;
        }
        unsafe { ff() }
    }

    pub fn at(&self, index: u32) -> Option<Value> {
        call!(self, at, &Integer::from(index))
    }

    pub fn push(&self, value: &Value) {
        unsafe extern "C" {
            #[link_name = "$Array$push"]
            fn ff(array: NonZeroU32, value: NonZeroU32);
        }

        unsafe { ff(self.index(), value.index()) }
    }
}

impl Default for Array {
    fn default() -> Self {
        Self::new()
    }
}
