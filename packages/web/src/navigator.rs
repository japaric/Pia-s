use js::Downcast;

use crate::{HtmlElement, MIDIAccess};

#[derive(Clone, Copy)]
pub struct Navigator;

impl Navigator {
    pub fn request_midi_access(&self) -> js::Promise<MIDIAccess> {
        unsafe {
            js::call!(self, requestMIDIAccess)
                .unwrap_unchecked()
                .downcast()
        }
    }
}

impl core::ops::Deref for Navigator {
    type Target = HtmlElement;

    fn deref(&self) -> &Self::Target {
        static INDEX: u32 = 3; // see app.js.j2

        unsafe { &*core::ptr::addr_of!(INDEX).cast() }
    }
}
