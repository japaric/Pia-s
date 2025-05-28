use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;

use js::Float;
use music::{Degree, MajorScale, NoteName, NoteNames};
use spur::{Message, Publish as _, React};
use web::{Node, SVGEllipseElement, SVGSVGElement, SVGTextElement};

use crate::{
    broker::Broker,
    class::Class,
    consts,
    messages::{ActiveNotesChanged, NewScaleTonicSelected},
    svg,
};

pub fn initialize(parent: &Node) {
    let canvas = Canvas::new(&svg::svg(parent, Class::Tonnetz, false));

    Broker::publish(Initialize { tonnetz: canvas });
}

pub struct Tonnetz {
    state: Option<State>,
}

impl Tonnetz {
    pub const fn new() -> Self {
        Self { state: None }
    }
}

#[derive(Message)]
pub struct Initialize {
    tonnetz: Canvas,
}

impl React<Initialize> for Tonnetz {
    fn react(&mut self, Initialize { tonnetz }: Initialize) {
        let tonic = NoteName::CIRCLE_OF_FIFTHS[consts::INITIAL_SCALE_TONIC_INDEX as usize];
        self.state = Some(State {
            canvas: tonnetz,
            live: NoteNames::empty(),
            scale: MajorScale::new(tonic),
        })
    }
}

impl React<NewScaleTonicSelected> for Tonnetz {
    fn react(&mut self, NewScaleTonicSelected(index): NewScaleTonicSelected) {
        let Some(state) = &mut self.state else { return };

        let new_tonic = NoteName::CIRCLE_OF_FIFTHS[index];

        if new_tonic != state.scale.tonic() {
            state.scale = MajorScale::new(new_tonic);
            state.canvas.reset_highlights();

            for note in state.live.clone() {
                let degree = state.scale.name2degree(note);
                state.canvas.highlight_on(degree);
            }
        }
    }
}

impl React<ActiveNotesChanged> for Tonnetz {
    fn react(&mut self, ActiveNotesChanged { held, sustained }: ActiveNotesChanged) {
        let Some(state) = &mut self.state else { return };

        let mut new_live = NoteNames::empty();
        for note in held {
            new_live.insert(note.name());
        }
        for note in sustained {
            new_live.insert(note.name());
        }

        let old_live = state.live.clone();
        for note in new_live.clone() {
            if !old_live.contains(note) {
                let degree = state.scale.name2degree(note);
                state.canvas.highlight_on(degree);
            }
        }

        for note in old_live {
            if !new_live.contains(note) {
                let degree = state.scale.name2degree(note);
                state.canvas.highlight_off(degree);
            }
        }

        state.live = new_live;
    }
}

struct State {
    canvas: Canvas,
    live: NoteNames,
    scale: MajorScale,
}

struct Canvas {
    circles: BTreeMap<Degree, Vec<SVGEllipseElement>>,
    labels: BTreeMap<Degree, Vec<SVGTextElement>>,
}

impl Canvas {
    fn new(parent: &SVGSVGElement) -> Self {
        use Degree::*;

        const PADDING: f64 = 0.5;
        const HEIGHT_PX: f64 = 200.;
        const GR: f64 = 1.618033988749895;
        const SQRT3: f64 = 1.7320508075688772;

        let dy = (GR + 1.) / 2. * SQRT3;
        let dx = GR + 1.;
        let height = 4. * dy + 2. + 2. * PADDING;
        let width = 5. * dx + 2. + 2. * PADDING;

        let r_px = HEIGHT_PX / height;
        let width_px = r_px * width;
        parent.set_width(&Float::from(width_px));

        let mut circles = BTreeMap::<_, Vec<_>>::new();
        let mut labels = BTreeMap::<_, Vec<_>>::new();

        let label_class = Class::TonnetzLabel.as_str().into();

        let center_x = width_px / 2.;
        let center_y = HEIGHT_PX / 2.;

        let coordinates = [
            // row 0 (center)
            (0., 0., Some(One)),
            (dx, 0., Some(Five)),
            (2. * dx, 0., Some(Two)),
            (-dx, 0., Some(Four)),
            (-2. * dx, 0., Some(FlatSeven)),
            // row -1
            (-0.5 * dx, -dy, Some(FlatSix)),
            (-1.5 * dx, -dy, Some(FlatTwo)),
            (0.5 * dx, -dy, Some(FlatThree)),
            (1.5 * dx, -dy, Some(FlatSeven)),
            // row -2 (padding)
            (0., -2. * dy, None),
            (-dx, -2. * dy, None),
            (dx, -2. * dy, None),
            // row +1
            (-0.5 * dx, dy, Some(Six)),
            (-1.5 * dx, dy, Some(Two)),
            (-2.5 * dx, dy, Some(Five)),
            (0.5 * dx, dy, Some(Three)),
            (1.5 * dx, dy, Some(Seven)),
            (2.5 * dx, dy, Some(SharpFour)),
            // row +2
            (0., 2. * dy, Some(FlatTwo)),
            (-dx, 2. * dy, Some(SharpFour)),
            (-2. * dx, 2. * dy, Some(Seven)),
            (dx, 2. * dy, Some(FlatSix)),
            (2. * dx, 2. * dy, Some(FlatThree)),
        ];

        for (offset_x, offset_y, maybe_degree) in coordinates {
            let cx_px = (center_x + offset_x * r_px).into();
            let cy_px = (center_y + offset_y * r_px).into();
            let circle = svg::circle(parent, Class::TonnetzCircle, &cx_px, &cy_px, r_px);

            if let Some(degree) = maybe_degree {
                circles.entry(degree).or_default().push(circle);

                let label = svg::text(parent, &cx_px, &cy_px);
                label.set_class_name(&label_class);
                label.set_text_content(&degree.as_str().into());
                labels.entry(degree).or_default().push(label);
            }
        }

        Self { circles, labels }
    }

    fn highlight_on(&self, degree: Degree) {
        let class = Class::Highlight.as_str().into();
        if let Some(circles) = self.circles.get(&degree) {
            for circle in circles {
                circle.add_class(&class);
            }
        }
        if let Some(labels) = self.labels.get(&degree) {
            for label in labels {
                label.add_class(&class);
            }
        }
    }

    fn highlight_off(&self, degree: Degree) {
        let class = Class::Highlight.as_str().into();
        if let Some(circles) = self.circles.get(&degree) {
            for circle in circles {
                circle.rm_class(&class);
            }
        }
        if let Some(labels) = self.labels.get(&degree) {
            for label in labels {
                label.rm_class(&class);
            }
        }
    }

    fn reset_highlights(&self) {
        let class = Class::Highlight.as_str().into();
        for circles in self.circles.values() {
            for circle in circles {
                circle.rm_class(&class);
            }
        }

        for labels in self.labels.values() {
            for label in labels {
                label.rm_class(&class);
            }
        }
    }
}
