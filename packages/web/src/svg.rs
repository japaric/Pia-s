use crate::{Element, IsElementSvg};

js::inheritance!(SVGElement: Element);

impl SVGElement {
    pub fn set_class_name(&self, name: &js::String) {
        self.set_attribute(&"class".into(), name);
    }
}

js::inheritance!(SVGGraphicsElement: SVGElement);

js::inheritance!(SVGGeometryElement: SVGGraphicsElement);

js::inheritance!(SVGSVGElement: SVGGeometryElement);

unsafe impl IsElementSvg for SVGSVGElement {
    const TAG_NAME: &'static str = "svg";
}

impl SVGSVGElement {
    pub fn set_height(&self, height: &js::Value) {
        self.set_attribute(&"height".into(), height);
    }

    pub fn set_width(&self, width: &js::Value) {
        self.set_attribute(&"width".into(), width);
    }
}

js::inheritance!(SVGRectElement: SVGGeometryElement);

unsafe impl IsElementSvg for SVGRectElement {
    const TAG_NAME: &'static str = "rect";
}

impl SVGRectElement {
    pub fn set_height(&self, height: &js::Value) {
        self.set_attribute(&"height".into(), height);
    }

    pub fn set_width(&self, width: &js::Value) {
        self.set_attribute(&"width".into(), width);
    }

    pub fn set_opacity(&self, x: &js::Value) {
        self.set_attribute(&"opacity".into(), x);
    }

    pub fn set_x(&self, x: &js::Value) {
        self.set_attribute(&"x".into(), x);
    }

    pub fn set_y(&self, y: &js::Value) {
        self.set_attribute(&"y".into(), y);
    }
}

js::inheritance!(SVGEllipseElement: SVGGeometryElement);

impl SVGEllipseElement {
    pub fn set_cx(&self, cx: &js::Value) {
        self.set_attribute(&"cx".into(), cx);
    }

    pub fn set_cy(&self, cy: &js::Value) {
        self.set_attribute(&"cy".into(), cy);
    }

    pub fn set_rx(&self, rx: &js::Value) {
        self.set_attribute(&"rx".into(), rx);
    }

    pub fn set_ry(&self, ry: &js::Value) {
        self.set_attribute(&"ry".into(), ry);
    }
}

unsafe impl IsElementSvg for SVGEllipseElement {
    const TAG_NAME: &'static str = "ellipse";
}

js::inheritance!(SVGTextContentElement: SVGGraphicsElement);

js::inheritance!(SVGTextPositioningElement: SVGTextContentElement);

js::inheritance!(SVGTextElement: SVGTextPositioningElement);

unsafe impl IsElementSvg for SVGTextElement {
    const TAG_NAME: &'static str = "text";
}

impl SVGTextElement {
    pub fn set_x(&self, x: &js::Value) {
        self.set_attribute(&"x".into(), x);
    }

    pub fn set_y(&self, y: &js::Value) {
        self.set_attribute(&"y".into(), y);
    }

    pub fn set_dominant_baseline(&self, dominant_baseline: DominantBaseline) {
        self.set_attribute(
            &"dominant-baseline".into(),
            &js::String::from(dominant_baseline.as_str()),
        );
    }

    pub fn set_text_anchor(&self, text_anchor: TextAnchor) {
        self.set_attribute(
            &"text-anchor".into(),
            &js::String::from(text_anchor.as_str()),
        );
    }
}

#[derive(Clone, Copy)]
pub enum DominantBaseline {
    Central,
    Middle,
    TextTop,
}

impl DominantBaseline {
    fn as_str(&self) -> &'static str {
        use DominantBaseline::*;

        match self {
            Central => "central",
            Middle => "middle",
            TextTop => "text-top",
        }
    }
}

#[derive(Clone, Copy)]
pub enum TextAnchor {
    Middle,
}

impl TextAnchor {
    fn as_str(&self) -> &'static str {
        match self {
            TextAnchor::Middle => "middle",
        }
    }
}
