use crate::{Element, Event, IsElement};

pub use form::*;
use js::Function;

mod form;

js::inheritance!(HtmlElement: Element);

impl HtmlElement {
    pub fn set_onchange(&self, listener: fn(Event)) {
        self.set(&"onchange".into(), &Function::from(listener))
    }
}

js::inheritance!(HtmlDivElement: HtmlElement);

impl IsElement for HtmlDivElement {
    const TAG_NAME: &'static str = "div";
}

js::inheritance!(HtmlSupElement: HtmlElement);

impl IsElement for HtmlSupElement {
    const TAG_NAME: &'static str = "sup";
}

js::inheritance!(HtmlSubElement: HtmlElement);

impl IsElement for HtmlSubElement {
    const TAG_NAME: &'static str = "sub";
}

js::inheritance!(HtmlSpanElement: HtmlElement);

impl IsElement for HtmlSpanElement {
    const TAG_NAME: &'static str = "span";
}
