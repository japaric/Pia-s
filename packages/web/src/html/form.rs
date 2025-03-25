use js::{Downcast, Integer};

use crate::{HtmlElement, IsElement};

js::inheritance!(HtmlFieldsetElement: HtmlElement);

impl IsElement for HtmlFieldsetElement {
    const TAG_NAME: &'static str = "fieldset";
}

js::inheritance!(HtmlFormElement: HtmlElement);

impl IsElement for HtmlFormElement {
    const TAG_NAME: &'static str = "form";
}

js::inheritance!(HtmlLegendElement: HtmlElement);

impl IsElement for HtmlLegendElement {
    const TAG_NAME: &'static str = "legend";
}

js::inheritance!(HtmlOptionElement: HtmlElement);

impl IsElement for HtmlOptionElement {
    const TAG_NAME: &'static str = "option";
}

impl HtmlOptionElement {
    pub fn set_value(&self, value: &js::String) {
        self.set(&"value".into(), value)
    }
}

js::inheritance!(HtmlSelectElement: HtmlElement);

impl IsElement for HtmlSelectElement {
    const TAG_NAME: &'static str = "select";
}

impl HtmlSelectElement {
    pub fn add(&self, option: &HtmlOptionElement) {
        js::call!(self, add, option);
    }

    pub fn set_required(&self, required: bool) {
        self.set_attribute(&"required".into(), required.as_ref())
    }

    pub fn set_selected_index(&self, index: u32) {
        self.set(&"selectedIndex".into(), &Integer::from(index))
    }

    pub fn selected_index(&self) -> i32 {
        unsafe {
            self.get(&"selectedIndex".into())
                .unwrap_unchecked()
                .to_u32() as i32
        }
    }

    pub fn value(&self) -> js::String {
        unsafe { self.get(&"value".into()).unwrap_unchecked().downcast() }
    }
}
