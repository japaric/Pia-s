use js::Downcast;

use crate::{HtmlElement, IsElement, IsElementNs};

#[derive(Clone, Copy)]
pub struct Document;

impl Document {
    pub fn create_element<T>(&self) -> T
    where
        T: IsElement,
        js::Value: Downcast<T>,
    {
        unsafe {
            js::call!(self, createElement, &js::String::from(T::TAG_NAME))
                .unwrap_unchecked()
                .downcast()
        }
    }

    pub fn create_element_ns<T>(&self) -> T
    where
        T: IsElementNs,
        js::Value: Downcast<T>,
    {
        unsafe {
            js::call!(
                self,
                createElementNS,
                &js::String::from(T::NS),
                &js::String::from(T::TAG_NAME)
            )
            .unwrap_unchecked()
            .downcast()
        }
    }
}

impl core::ops::Deref for Document {
    type Target = HtmlElement;

    fn deref(&self) -> &Self::Target {
        static INDEX: u32 = 1; // see app.js.j2

        unsafe { &*core::ptr::addr_of!(INDEX).cast() }
    }
}
