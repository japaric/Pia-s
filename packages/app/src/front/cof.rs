use alloc::collections::btree_map::BTreeMap;
use alloc::format;
use js::Float;
use music::{Degree, MajorScale, NoteName, ScaleType};
use spur::{Message, Publish as _, React};
use web::{Node, SVGPathElement, SVGSVGElement, SVGTextElement};

use crate::broker::Broker;
use crate::class::Class;
use crate::messages::{ActiveHarmonyChanged, NewScaleTonicSelected, NewScaleTypeSelected};
use crate::{consts, svg};

pub fn initialize(parent: &Node) {
    let canvas = Canvas::new(&svg::svg(parent, Class::Tonnetz, false));
    canvas.set_scale_type(ScaleType::ALL[consts::INITIAL_SCALE_TYPE_INDEX as usize]);

    Broker::publish(Initialize { canvas });
}

pub struct CircleOfFifths {
    state: Option<State>,
}

impl CircleOfFifths {
    pub const fn new() -> Self {
        Self { state: None }
    }
}

#[derive(Message)]
pub struct Initialize {
    canvas: Canvas,
}

impl React<Initialize> for CircleOfFifths {
    fn react(&mut self, Initialize { canvas }: Initialize) {
        self.state = Some(State::new(canvas));
    }
}

impl React<NewScaleTonicSelected> for CircleOfFifths {
    fn react(&mut self, NewScaleTonicSelected(index): NewScaleTonicSelected) {
        let Some(state) = &mut self.state else { return };

        let new_tonic = NoteName::CIRCLE_OF_FIFTHS[index];

        if new_tonic != state.scale.tonic() {
            state.scale = MajorScale::new(new_tonic);

            state.canvas.reset_highlights();
            for (note, is_minor) in &state.live {
                let degree = state.scale.name2degree(*note);

                state.canvas.highlight(degree, *is_minor);
            }
        }
    }
}

impl React<NewScaleTypeSelected> for CircleOfFifths {
    fn react(&mut self, NewScaleTypeSelected(index): NewScaleTypeSelected) {
        let Some(state) = &mut self.state else { return };

        let new_scale_type = ScaleType::ALL[index];

        state.canvas.reset_in_key();
        state.canvas.set_scale_type(new_scale_type);
    }
}

impl React<ActiveHarmonyChanged> for CircleOfFifths {
    fn react(&mut self, ActiveHarmonyChanged { tonics }: ActiveHarmonyChanged) {
        let Some(state) = &mut self.state else { return };

        state.canvas.reset_highlights();
        for (tonic, is_minor) in &tonics {
            let degree = state.scale.name2degree(*tonic);
            state.canvas.highlight(degree, *is_minor);
        }

        state.live = tonics;
    }
}

struct State {
    canvas: Canvas,
    live: BTreeMap<NoteName, bool>,
    scale: MajorScale,
}

impl State {
    fn new(canvas: Canvas) -> Self {
        let tonic = NoteName::CIRCLE_OF_FIFTHS[consts::INITIAL_SCALE_TONIC_INDEX as usize];
        Self {
            canvas,
            live: BTreeMap::new(),
            scale: MajorScale::new(tonic),
        }
    }
}

struct Canvas {
    svg: SVGSVGElement,
    items: BTreeMap<Degree, Items>,
}

const SIDE: f64 = 300.;

impl Canvas {
    fn new(parent: &SVGSVGElement) -> Self {
        use Degree::*;

        const SIN15: f64 = 0.25881904510252074;
        const COS15: f64 = 0.9659258262890683;

        const SIN30: f64 = 0.49999999999999994;
        const COS30: f64 = 0.8660254037844387;

        const SIN45: f64 = 0.7071067811865475;
        const COS45: f64 = SIN45;

        parent.set_width(&Float::from(SIDE));

        let mut items = BTreeMap::new();
        let mut last = (-SIN15, -COS15);

        let cases = [
            (One, (SIN15, -COS15), (0., -1.)),
            (Five, (SIN45, -COS45), (SIN30, -COS30)),
            (Two, (COS15, -SIN15), (COS30, -SIN30)),
            (Six, (COS15, SIN15), (1., 0.)),
            (Three, (SIN45, COS45), (COS30, SIN30)),
            (Seven, (SIN15, COS15), (SIN30, COS30)),
            (SharpFour, (-SIN15, COS15), (0., 1.)),
            (FlatTwo, (-SIN45, COS45), (-SIN30, COS30)),
            (FlatSix, (-COS15, SIN15), (-COS30, SIN30)),
            (FlatThree, (-COS15, -SIN15), (-1., 0.)),
            (FlatSeven, (-SIN45, -COS45), (-COS30, -SIN30)),
            (Four, (-SIN15, -COS15), (-SIN30, -COS30)),
        ];
        for (degree, curr, text) in cases {
            items.insert(degree, Items::new(parent, last, curr, degree, text));
            last = curr;
        }

        Self {
            svg: parent.clone(),
            items,
        }
    }

    fn set_scale_type(&self, scale_type: ScaleType) {
        let class = Class::InScale.as_str().into();
        for degree in scale_type.degrees() {
            let items = &self.items[degree];

            let item = if scale_type.contains(*degree, false) {
                &items.major
            } else {
                &items.minor
            };

            item.path.add_class(&class);
            item.label.add_class(&class);
        }
    }

    fn reset_in_key(&self) {
        let class = Class::InScale.as_str().into();
        for items in self.items.values() {
            for item in [&items.minor, &items.major] {
                item.path.rm_class(&class);
                item.label.rm_class(&class);
            }
        }
    }

    fn reset_highlights(&self) {
        let class = Class::Highlight.as_str().into();
        for items in self.items.values() {
            items.major.path.rm_class(&class);
            items.major.label.rm_class(&class);

            items.minor.path.rm_class(&class);
            items.minor.label.rm_class(&class);
        }
    }

    fn highlight(&self, degree: Degree, is_minor: bool) {
        let class_highlight = Class::Highlight.as_str().into();

        let items = &self.items[&degree];

        let item = if is_minor { &items.minor } else { &items.major };

        item.label.add_class(&class_highlight);

        item.path.add_class(&class_highlight);
        self.svg.append_child(&item.path);
    }
}

struct Items {
    major: Item,
    minor: Item,
}

impl Items {
    fn new(
        parent: &SVGSVGElement,
        start: (f64, f64),
        end: (f64, f64),
        degree: Degree,
        text: (f64, f64),
    ) -> Self {
        const R1: f64 = 140.;
        const R2: f64 = 100.;
        const R3: f64 = 50.;

        let class_label = Class::CofLabel.as_str().into();
        let degree_label = js::String::from(degree.as_str());

        let major = Item::draw(parent, (R1, R2), start, end, text);
        major.label.set_text_content(&degree.roman_major().into());
        major.label.set_class_name(&class_label);
        major.label.add_class(&degree_label);

        let minor = Item::draw(parent, (R2, R3), start, end, text);
        minor.label.set_text_content(&degree.roman_minor().into());
        minor.label.set_class_name(&class_label);
        minor.label.add_class(&degree_label);

        Items { major, minor }
    }
}

struct Item {
    path: SVGPathElement,
    label: SVGTextElement,
}

impl Item {
    fn draw(
        parent: &SVGSVGElement,
        (ro, ri): (f64, f64),
        (sin_start, cos_start): (f64, f64),
        (sin_end, cos_end): (f64, f64),
        (sin_text, cos_text): (f64, f64),
    ) -> Self {
        let x1 = SIDE / 2. + ro * sin_start;
        let y1 = SIDE / 2. + ro * cos_start;
        let x2 = SIDE / 2. + ro * sin_end;
        let y2 = SIDE / 2. + ro * cos_end;
        let x3 = SIDE / 2. + ri * sin_end;
        let y3 = SIDE / 2. + ri * cos_end;
        let x4 = SIDE / 2. + ri * sin_start;
        let y4 = SIDE / 2. + ri * cos_start;

        let path = svg::path(
            parent,
            Class::CofPath,
            &format!(
                "M {x1} {y1} A {ro} {ro} 0 0 1 {x2} {y2} L {x3} {y3} A {ri} {ri} 0 0 0 {x4} {y4} Z"
            ),
        );

        let x_text = SIDE / 2. + (ro + ri) / 2. * sin_text;
        let y_text = SIDE / 2. + (ro + ri) / 2. * cos_text;
        let label = svg::text(parent, &Float::from(x_text), &Float::from(y_text));

        Item { path, label }
    }
}
