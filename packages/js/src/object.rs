use core::num::NonZeroU32;

use crate::{Array, String, Value};

inheritance!(Object: Value);

impl Object {
    pub fn call(&self, property: &String, args: &Array) -> Option<Value> {
        unsafe extern "C" {
            #[link_name = "$Object$call"]
            fn ff(object: NonZeroU32, property: NonZeroU32, args: NonZeroU32) -> Option<Value>;
        }

        unsafe { ff(self.index(), property.index(), args.index()) }
    }

    pub fn get(&self, property: &String) -> Option<Value> {
        unsafe extern "C" {
            #[link_name = "$Object$get"]
            fn ff(object: NonZeroU32, property: NonZeroU32) -> Option<Value>;
        }

        unsafe { ff(self.index(), property.index()) }
    }

    pub fn set(&self, property: &String, value: &Value) {
        unsafe extern "C" {
            #[link_name = "$Object$set"]
            fn ff(object: NonZeroU32, property: NonZeroU32, value: NonZeroU32);
        }

        unsafe { ff(self.index(), property.index(), value.index()) }
    }
}
