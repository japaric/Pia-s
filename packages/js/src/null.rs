use crate::Value;

#[derive(Clone, Copy)]
pub struct Null;

impl core::ops::Deref for Null {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        static INDEX: u32 = 6; // see app.js.j2

        unsafe { &*core::ptr::addr_of!(INDEX).cast() }
    }
}
