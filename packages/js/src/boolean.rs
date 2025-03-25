use core::{ops, ptr::addr_of};

use crate::Value;

#[derive(Clone, Copy)]
pub struct True;

impl ops::Deref for True {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        static INDEX: u32 = 4; // see app.js.j2

        unsafe { &*addr_of!(INDEX).cast() }
    }
}

#[derive(Clone, Copy)]
pub struct False;

impl ops::Deref for False {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        static INDEX: u32 = 5; // see app.js.j2

        unsafe { &*addr_of!(INDEX).cast() }
    }
}

impl AsRef<Value> for bool {
    fn as_ref(&self) -> &Value {
        match self {
            true => &True,
            false => &False,
        }
    }
}
