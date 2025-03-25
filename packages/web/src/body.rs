use crate::HtmlElement;

#[derive(Clone, Copy)]
pub struct Body;

impl core::ops::Deref for Body {
    type Target = HtmlElement;

    fn deref(&self) -> &Self::Target {
        static INDEX: u32 = 2; // see app.js.j2

        unsafe { &*core::ptr::addr_of!(INDEX).cast() }
    }
}
