use js::Float;
use web::{
    Document, DominantBaseline, Node, SVGEllipseElement, SVGRectElement, SVGSVGElement,
    SVGTextElement, TextAnchor,
};

use crate::{class::Class, html};

pub fn svg(parent: &Node, class: Class, set_width: bool) -> SVGSVGElement {
    let parent = html::div(parent, class);
    let svg = Document.create_element_ns::<SVGSVGElement>();
    svg.set_height(&js::String::from("100%"));
    if set_width {
        svg.set_width(&js::String::from("100%"));
    }
    parent.append_child(&svg);
    svg
}

pub fn circle(
    svg: &SVGSVGElement,
    class: Class,
    cx: &Float,
    cy: &Float,
    r: f64,
) -> SVGEllipseElement {
    let ellipse = Document.create_element_ns::<SVGEllipseElement>();

    ellipse.set_class_name(&class.as_str().into());
    ellipse.set_cx(cx);
    ellipse.set_cy(cy);
    let r = Float::from(r);
    ellipse.set_rx(&r);
    ellipse.set_ry(&r);

    svg.append_child(&ellipse);

    ellipse
}

pub fn rect(
    svg: &SVGSVGElement,
    class: Class,
    x: &js::Value,
    y: &js::Value,
    width: &js::Value,
    height: &js::Value,
) -> SVGRectElement {
    let rect = Document.create_element_ns::<SVGRectElement>();

    rect.set_x(x);
    rect.set_y(y);
    rect.set_height(height);
    rect.set_width(width);
    rect.set_class_name(&class.as_str().into());

    svg.append_child(&rect);

    rect
}

pub fn text(svg: &SVGSVGElement, x: &js::Value, y: &js::Value) -> SVGTextElement {
    let text = Document.create_element_ns::<SVGTextElement>();
    text.set_x(x);
    text.set_y(y);
    text.set_text_anchor(TextAnchor::Middle);
    text.set_dominant_baseline(DominantBaseline::Central);

    svg.append_child(&text);

    text
}
