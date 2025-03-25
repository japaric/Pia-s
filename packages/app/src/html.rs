use web::{
    Document, HtmlDivElement, HtmlFieldsetElement, HtmlFormElement, HtmlLegendElement,
    HtmlOptionElement, HtmlSelectElement, HtmlSpanElement, HtmlSubElement, HtmlSupElement, Node,
};

use crate::class::Class;

pub fn sub(parent: &Node, text: &str) -> HtmlSubElement {
    let sub = Document.create_element::<HtmlSubElement>();
    sub.set_text_content(&text.into());
    parent.append_child(&sub);
    sub
}

pub fn sup(parent: &Node, text: &str) -> HtmlSupElement {
    let sup = Document.create_element::<HtmlSupElement>();
    sup.set_text_content(&text.into());
    parent.append_child(&sup);
    sup
}

pub fn div(parent: &Node, class: Class) -> HtmlDivElement {
    let div = Document.create_element::<HtmlDivElement>();
    div.set_class_name(&class.as_str().into());
    parent.append_child(&div);
    div
}

pub fn span(parent: &Node, text: &str) -> HtmlSpanElement {
    let span = Document.create_element::<HtmlSpanElement>();
    span.set_text_content(&text.into());
    parent.append_child(&span);
    span
}

pub fn form(parent: &Node, class: Class) -> Form {
    let form = Document.create_element::<HtmlFormElement>();
    form.set_class_name(&class.as_str().into());
    parent.append_child(&form);
    Form { inner: form }
}

pub fn option(
    select: &HtmlSelectElement,
    value: Option<&js::String>,
    text: &js::String,
) -> HtmlOptionElement {
    let option = Document.create_element::<HtmlOptionElement>();
    if let Some(value) = value {
        option.set_value(value);
    }
    option.set_text_content(text);
    select.add(&option);
    option
}

pub fn select(parent: &Node, id: &js::String) -> HtmlSelectElement {
    let select = Document.create_element::<HtmlSelectElement>();
    select.set_id(id);
    parent.append_child(&select);
    select
}

pub struct Form {
    inner: HtmlFormElement,
}

impl Form {
    pub fn fieldset(&self, legend: &js::String) -> HtmlDivElement {
        let legend_element = Document.create_element::<HtmlLegendElement>();
        legend_element.set_text_content(legend);

        let fieldset = Document.create_element::<HtmlFieldsetElement>();
        fieldset.append_child(&legend_element);
        self.inner.append_child(&fieldset);

        div(&fieldset, Class::ColumnContainer)
    }
}
