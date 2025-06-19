#![no_std]

pub use body::Body;
pub use console::Console;
pub use document::Document;
pub use element::Element;
pub use event::{Event, EventTarget};
pub use html::*;
use js::IsValue;
pub use midi::*;
pub use navigator::Navigator;
pub use node::Node;
pub use performance::Performance;
pub use svg::*;

mod body;
mod console;
mod document;
mod element;
mod event;
mod html;
#[doc(hidden)]
pub mod internal;
mod macros;
mod midi;
mod navigator;
mod node;
mod performance;
mod svg;

pub trait IsElement: js::Upcast<Supertype = HtmlElement> {
    const TAG_NAME: &'static str;
}

/// # Safety
pub unsafe trait IsElementNs: IsValue {
    const NS: &'static str;
    const TAG_NAME: &'static str;
}

/// # Safety
pub unsafe trait IsElementSvg: IsValue {
    const TAG_NAME: &'static str;
}

unsafe impl<T: IsElementSvg> IsElementNs for T {
    const NS: &'static str = "http://www.w3.org/2000/svg";
    const TAG_NAME: &'static str = T::TAG_NAME;
}

pub fn queue_microtask(function: fn()) {
    unsafe extern "C" {
        #[link_name = "$queueMicrotask"]
        fn ff(f: u32);
    }

    unsafe { ff(function as usize as u32) }
}
