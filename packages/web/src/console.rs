use js::Object;

#[derive(Clone, Copy)]
pub struct Console;

impl core::ops::Deref for Console {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        static INDEX: u32 = 7; // see app.js.j2

        unsafe { &*core::ptr::addr_of!(INDEX).cast() }
    }
}
