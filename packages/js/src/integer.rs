use crate::{Downcast as _, Value};

inheritance!(Integer: Value);

impl From<u32> for Integer {
    fn from(value: u32) -> Self {
        unsafe extern "C" {
            #[link_name = "$Integer$from_u32"]
            fn ff(integer: u32) -> Value;
        }

        unsafe { ff(value).downcast() }
    }
}
